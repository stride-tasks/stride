import 'dart:async';
import 'dart:convert';
import 'dart:ffi';
import 'dart:io';
import 'dart:isolate';
import 'dart:ui';

import 'package:ffi/ffi.dart';
import 'package:flutter/foundation.dart';

/// Why a Rust call failed.
enum RustErrorKind {
  // Reported by Rust; [code] is the wire name.
  io('io'),
  invalidArgument('invalid_argument'),
  notInitialized('not_initialized'),
  methodNotFound('method_not_found'),
  methodFailed('method_failed'),
  other('other'),
  panic('panic'),

  // Raised on the Dart side.
  internal('internal'),
  initFailed('init_failed'),
  workerClosed('worker_closed');

  const RustErrorKind(this.code);

  final String code;

  static RustErrorKind fromCode(String? code) => RustErrorKind.values
      .firstWhere((kind) => kind.code == code, orElse: () => other);
}

/// A failed Rust call (or a failure to reach the Rust worker).
class RustException implements Exception {
  const RustException(this.kind, this.message);

  final RustErrorKind kind;
  final String message;

  @override
  String toString() => 'RustException(${kind.name}): $message';
}

@immutable
class NativeNotification extends Notification {
  final String name;
  final String params;
  const NativeNotification(this.name, this.params);

  @override
  String getName() {
    return name;
  }
}

/// Client of the single process-wide isolate that executes all Rust calls.
///
/// [init] (from any isolate, app or background) starts the isolate and
/// registers it by name; [connect] attaches to it.
class RustWorker {
  RustWorker._(this._requests);

  static const _portName = 'stride-rust-worker';

  static const _workerClosedId = -1;

  var _nextId = 1;
  var _closed = false;
  final _responses = ReceivePort();
  // Subscribe to the worker's notifications only while someone listens.
  late final _notifications = StreamController<NativeNotification>.broadcast(
    onListen: () => _requests.send(['subscribe', _responses.sendPort]),
    onCancel: () => _requests.send(['unsubscribe', _responses.sendPort]),
  );
  final _pending = <int, Completer<String>>{};
  final SendPort _requests;

  /// Broadcast stream of Rust notifications.
  Stream<NativeNotification> get notifications => _notifications.stream;

  static RustWorker? _instance;

  /// Starts the process-wide worker isolate, replacing a previous one.
  ///
  /// Clients of the previous worker (including this isolate's) are detached;
  /// call [connect] again afterwards.
  static Future<void> init({
    required String supportPath,
    required String cachePath,
  }) async {
    disconnect();
    final previous = IsolateNameServer.lookupPortByName(_portName);
    if (previous != null) IsolateNameServer.removePortNameMapping(_portName);

    final ready = ReceivePort();
    try {
      await Isolate.spawn(_main, [ready.sendPort, supportPath, cachePath]);
      final [ok, payload] = await ready.first as List<Object?>;
      if (ok != true) {
        throw RustException(RustErrorKind.initFailed, payload! as String);
      }
      // Register only once Rust is initialized, so no client can attach to a
      // worker that fails to start.
      final registered = IsolateNameServer.registerPortWithName(
        payload! as SendPort,
        _portName,
      );
      assert(registered, 'init() must not be called concurrently');
    } finally {
      ready.close();
      // The new worker has replaced the native callback (or cleared it on
      // failure), so the old callback can be released safely now.
      previous?.send(['shutdown']);
    }
  }

  /// Closes this isolate's client, if any.
  static void disconnect() => _instance?.close();

  /// Returns this isolate's client of the running worker (see [init]).
  // ignore: prefer_constructors_over_static_methods
  static RustWorker connect() {
    final existing = _instance;
    if (existing != null) return existing;
    final requests = IsolateNameServer.lookupPortByName(_portName);
    if (requests == null) {
      throw StateError('Rust worker is not running');
    }
    final worker = RustWorker._(requests);
    worker._responses.listen(worker._onMessage);
    return _instance = worker;
  }

  Future<String> execute({required String method, required String args}) {
    if (_closed) {
      return Future.error(
        const RustException(
          RustErrorKind.workerClosed,
          'Rust worker client is closed',
        ),
      );
    }
    final id = _nextId++;
    final completer = _pending[id] = Completer<String>();
    _requests.send(['execute', _responses.sendPort, id, method, args]);
    return completer.future;
  }

  /// Detaches this client. The shared worker keeps running.
  void close() {
    if (_closed) return;
    _requests.send(['unsubscribe', _responses.sendPort]);
    _closeClient('Rust worker client was closed');
  }

  // Fails outstanding calls so that no caller waits forever.
  void _closeClient(String reason) {
    _closed = true;
    if (identical(_instance, this)) _instance = null;
    for (final completer in _pending.values) {
      completer.completeError(
        RustException(RustErrorKind.workerClosed, reason),
      );
    }
    _pending.clear();
    _responses.close();
    unawaited(_notifications.close());
  }

  // Messages are `[id, ok, payload]`; a null id is a notification and
  // `_workerClosedId` means the worker stopped.
  void _onMessage(Object? message) {
    final [id, ok, payload] = message! as List<Object?>;
    if (id == null) {
      _notifications.add(payload! as NativeNotification);
      return;
    }
    if (id == _workerClosedId) {
      _closeClient(payload! as String);
      return;
    }
    final completer = _pending.remove(id);
    if (ok == true) {
      completer?.complete(payload! as String);
    } else {
      final [kind, message] = payload! as List<Object?>;
      completer?.completeError(
        RustException(kind! as RustErrorKind, message! as String),
      );
    }
  }

  static Future<void> _main(List<Object?> init) async {
    final ready = init[0]! as SendPort;
    final supportPath = init[1]! as String;
    final cachePath = init[2]! as String;

    final requests = ReceivePort();

    final subscribers = <SendPort>{};
    NativeCallable<_Callback>? callback;
    _Native? loaded;
    try {
      final native = loaded = _Native();

      callback = NativeCallable<_Callback>.listener((
        Pointer<Utf8> namePointer,
        Pointer<Utf8> paramsPointer,
      ) {
        try {
          final name = namePointer.toDartString();
          final params = paramsPointer.toDartString();
          final notification = [null, true, NativeNotification(name, params)];
          for (final subscriber in subscribers) {
            subscriber.send(notification);
          }
        } finally {
          native.freeString(namePointer);
          native.freeString(paramsPointer);
        }
      });

      native.createStream(supportPath, cachePath, callback.nativeFunction);

      // Tell `init` that the worker is ready and how to reach it.
      ready.send([true, requests.sendPort]);

      await _serve(native, requests, subscribers, callback);
    } on Object catch (error) {
      // Startup failed. Rust may hold this callback or the previous worker's;
      // drop it.
      requests.close();
      loaded?.clearCallback();
      callback?.close();
      ready.send([false, _errorPayload(error)[1]]);
    }
  }

  // Handles requests until shut down. Per-request errors are reported to the
  // caller, anything else stops the worker and tells its clients.
  static Future<void> _serve(
    _Native native,
    ReceivePort requests,
    Set<SendPort> subscribers,
    NativeCallable<_Callback> callback,
  ) async {
    var reason = 'Rust worker was replaced';
    var crashed = false;
    try {
      await for (final message in requests) {
        switch (message) {
          case ['shutdown']:
            requests.close();
          case ['subscribe', final SendPort port]:
            subscribers.add(port);
          case ['unsubscribe', final SendPort port]:
            subscribers.remove(port);
          case [
            'execute',
            final SendPort port,
            final int id,
            final String method,
            final String args,
          ]:
            try {
              port.send([id, true, native.execute(method, args)]);
            } on Object catch (error) {
              port.send([id, false, _errorPayload(error)]);
            }
        }
      }
    } on Object catch (error) {
      crashed = true;
      reason = 'Rust worker stopped: ${_errorPayload(error)[1]}';
    }

    // On replacement the name already belongs to the new worker.
    if (IsolateNameServer.lookupPortByName(_portName) == requests.sendPort) {
      IsolateNameServer.removePortNameMapping(_portName);
    }

    requests.close();
    for (final subscriber in subscribers) {
      subscriber.send([_workerClosedId, false, reason]);
    }

    // After a replacement Rust already holds the new worker's callback.
    if (crashed) native.clearCallback();
    callback.close();
  }

  // Errors cross the isolate boundary as `[kind, message]`.
  static List<Object> _errorPayload(Object error) => switch (error) {
    RustException(:final kind, :final message) => [kind, message],
    StateError(:final message) => [RustErrorKind.internal, message],
    _ => [RustErrorKind.internal, error.toString()],
  };
}

typedef _Callback = Void Function(Pointer<Utf8> name, Pointer<Utf8> params);
typedef _CreateStream =
    Pointer<Utf8> Function(
      Pointer<Utf8>,
      Pointer<Utf8>,
      Pointer<NativeFunction<_Callback>>,
    );
typedef _ClearCallback = Void Function();
typedef _FreeNative = Void Function(Pointer<Utf8>);
typedef _Execute = Pointer<Utf8> Function(Pointer<Utf8>, Pointer<Utf8>);

/// Synchronous bindings; only used inside the worker isolate.
class _Native {
  _Native() : _library = _openLibrary();

  static const _libraryName = 'stride_flutter_bridge';
  static const _createStreamFunctionName = 'stride__create_stream';
  static const _executeFunctionName = 'stride__execute';
  static const _freeStringFunctionName = 'stride__free_string';
  static const _clearCallbackFunctionName = 'stride__clear_stream_callback';

  final DynamicLibrary _library;

  late final _createStream = _library
      .lookupFunction<_CreateStream, _CreateStream>(_createStreamFunctionName);
  late final _execute = _library.lookupFunction<_Execute, _Execute>(
    _executeFunctionName,
  );
  late final clearCallback = _library
      .lookupFunction<_ClearCallback, void Function()>(
        _clearCallbackFunctionName,
      );
  late final freeString = _library
      .lookupFunction<_FreeNative, void Function(Pointer<Utf8>)>(
        _freeStringFunctionName,
      );

  static DynamicLibrary _openLibrary() {
    if (Platform.isIOS || Platform.isMacOS) {
      return DynamicLibrary.process();
    }
    return DynamicLibrary.open(
      Platform.isWindows ? '$_libraryName.dll' : 'lib$_libraryName.so',
    );
  }

  void createStream(
    String supportPath,
    String cachePath,
    Pointer<NativeFunction<_Callback>> callback,
  ) {
    final supportPathPointer = supportPath.toNativeUtf8();
    final cachePathPointer = cachePath.toNativeUtf8();
    try {
      final errorPointer = _createStream(
        supportPathPointer,
        cachePathPointer,
        callback,
      );
      if (errorPointer.address != 0) {
        try {
          throw StateError(
            'Failed to initialize Rust: ${errorPointer.toDartString()}',
          );
        } finally {
          freeString(errorPointer);
        }
      }
    } finally {
      malloc
        ..free(supportPathPointer)
        ..free(cachePathPointer);
    }
  }

  String execute(String method, String args) {
    final methodPointer = method.toNativeUtf8();
    final argsPointer = args.toNativeUtf8();
    try {
      final responsePointer = _execute(methodPointer, argsPointer);
      if (responsePointer.address == 0) {
        throw const RustException(
          RustErrorKind.internal,
          'Rust execute returned a null response',
        );
      }
      try {
        final response =
            jsonDecode(responsePointer.toDartString()) as Map<String, dynamic>;
        if (response['ok'] != true) {
          throw RustException(
            RustErrorKind.fromCode(response['kind'] as String?),
            response['error'] as String? ?? 'Rust execute failed',
          );
        }
        return response['result'] as String;
      } finally {
        freeString(responsePointer);
      }
    } finally {
      malloc
        ..free(methodPointer)
        ..free(argsPointer);
    }
  }
}
