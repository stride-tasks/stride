import 'dart:async';
import 'dart:convert';
import 'dart:ffi';
import 'dart:io';
import 'dart:isolate';
import 'dart:ui';

import 'package:ffi/ffi.dart';

/// Client of the single process-wide isolate that executes all Rust calls.
///
/// [init] (from any isolate, app or background) starts the isolate and
/// registers it by name; [connect] attaches to it.
class RustWorker {
  RustWorker._(this._requests);

  static const _portName = 'stride-rust-worker';

  var _nextId = 1;
  final _responses = ReceivePort();
  final _notifications = StreamController<String>.broadcast();
  final _pending = <int, Completer<String>>{};
  final SendPort _requests;

  /// Broadcast stream of Rust notifications.
  Stream<String> get notifications => _notifications.stream;

  static RustWorker? _instance;

  /// Starts the process-wide worker isolate unless one is already running.
  static Future<void> init({
    required String supportPath,
    required String cachePath,
  }) async {
    if (IsolateNameServer.lookupPortByName(_portName) != null) return;

    final ready = ReceivePort();
    try {
      await Isolate.spawn(_main, [ready.sendPort, supportPath, cachePath]);
      final [ok, error] = await ready.first as List<Object?>;
      if (ok != true) throw StateError(error! as String);
    } finally {
      ready.close();
    }
  }

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
    requests.send(['subscribe', worker._responses.sendPort]);
    return _instance = worker;
  }

  Future<String> execute({required String method, required String args}) async {
    final id = _nextId++;
    final completer = _pending[id] = Completer<String>();
    _requests.send(['execute', _responses.sendPort, id, method, args]);
    return completer.future;
  }

  /// Detaches this client. The shared worker keeps running.
  void close() {
    _instance = null;
    _requests.send(['unsubscribe', _responses.sendPort]);
    _responses.close();
    unawaited(_notifications.close());
  }

  // Messages are `[id, ok, payload]`; a null id is a notification.
  void _onMessage(Object? message) {
    final [id, ok, payload] = message! as List<Object?>;
    if (id == null) {
      _notifications.add(payload! as String);
      return;
    }
    final completer = _pending.remove(id);
    if (ok == true) {
      completer?.complete(payload! as String);
    } else {
      completer?.completeError(StateError(payload! as String));
    }
  }

  static Future<void> _main(List<Object?> init) async {
    final ready = init[0]! as SendPort;
    final supportPath = init[1]! as String;
    final cachePath = init[2]! as String;

    final requests = ReceivePort();

    // Another isolate won the race to start the worker.
    if (!IsolateNameServer.registerPortWithName(requests.sendPort, _portName)) {
      requests.close();
      ready.send([true, null]);
      return;
    }

    final subscribers = <SendPort>{};
    NativeCallable<_Callback>? callback;
    try {
      final native = _Native();

      callback = NativeCallable<_Callback>.listener((Pointer<Utf8> pointer) {
        try {
          final notification = [null, true, pointer.toDartString()];
          for (final subscriber in subscribers) {
            subscriber.send(notification);
          }
        } finally {
          native.freeString(pointer);
        }
      });

      native.createStream(supportPath, cachePath, callback.nativeFunction);

      // Signal that the worker is ready.
      ready.send([true, null]);

      await for (final message in requests) {
        switch (message) {
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
              port.send([id, false, _errorMessage(error)]);
            }
        }
      }
    } on Object catch (error) {
      IsolateNameServer.removePortNameMapping(_portName);
      requests.close();
      callback?.close();
      ready.send([false, _errorMessage(error)]);
    }
  }

  static String _errorMessage(Object error) =>
      error is StateError ? error.message : error.toString();
}

typedef _Callback = Void Function(Pointer<Utf8>);
typedef _CreateStream =
    Pointer<Utf8> Function(
      Pointer<Utf8>,
      Pointer<Utf8>,
      Pointer<NativeFunction<_Callback>>,
    );
typedef _FreeNative = Void Function(Pointer<Utf8>);
typedef _Execute = Pointer<Utf8> Function(Pointer<Utf8>, Pointer<Utf8>);

/// Synchronous bindings; only used inside the worker isolate.
class _Native {
  _Native() : _library = _openLibrary();

  static const _libraryName = 'stride_flutter_bridge';
  static const _createStreamFunctionName = 'stride__create_stream';
  static const _executeFunctionName = 'stride__execute';
  static const _freeStringFunctionName = 'stride__free_string';

  final DynamicLibrary _library;

  late final _createStream = _library
      .lookupFunction<_CreateStream, _CreateStream>(_createStreamFunctionName);
  late final _execute = _library.lookupFunction<_Execute, _Execute>(
    _executeFunctionName,
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
        throw StateError('Rust execute returned a null response');
      }
      try {
        final response =
            jsonDecode(responsePointer.toDartString()) as Map<String, dynamic>;
        if (response['ok'] != true) {
          throw StateError(
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
