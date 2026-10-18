import 'dart:async';
import 'dart:convert';
import 'dart:ffi';
import 'dart:io';
import 'dart:isolate';

import 'package:ffi/ffi.dart';

/// Owns a persistent isolate in which all Rust calls are executed.
class RustWorker {
  RustWorker._();

  var _nextId = 1;
  final _responses = ReceivePort();
  final _notifications = StreamController<String>.broadcast();
  final _pending = <int, Completer<Object?>>{};

  late final SendPort _requests;

  /// Broadcast stream of Rust notifications.
  Stream<String> get notifications => _notifications.stream;

  static Future<RustWorker> spawn({
    required String supportPath,
    required String cachePath,
  }) async {
    final worker = RustWorker._();
    worker._responses.listen(worker._onMessage);

    // Id 0 is the startup handshake.
    final ready = worker._pending[0] = Completer<Object?>();
    try {
      await Isolate.spawn(_main, [
        worker._responses.sendPort,
        supportPath,
        cachePath,
      ]);
      worker._requests = (await ready.future)! as SendPort;
      return worker;
    } on Object {
      worker._dispose();
      rethrow;
    }
  }

  Future<String> execute({required String method, required String args}) async {
    final id = _nextId++;
    final completer = _pending[id] = Completer<Object?>();
    _requests.send([id, method, args]);
    return (await completer.future)! as String;
  }

  /// Stops the isolate. The worker must not be used afterwards.
  void close() {
    _requests.send(null);
    _dispose();
  }

  void _dispose() {
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
      completer?.complete(payload);
    } else {
      completer?.completeError(StateError(payload! as String));
    }
  }

  static Future<void> _main(List<Object?> init) async {
    final responses = init[0]! as SendPort;
    final requests = ReceivePort();
    NativeCallable<_Callback>? callback;
    try {
      final native = _Native();

      callback = NativeCallable<_Callback>.listener((Pointer<Utf8> pointer) {
        try {
          responses.send([null, true, pointer.toDartString()]);
        } finally {
          native.freeString(pointer);
        }
      });

      final supportPath = init[1]! as String;
      final cachePath = init[2]! as String;
      native.createStream(supportPath, cachePath, callback.nativeFunction);

      await for (final message in requests) {
        if (message == null) {
          requests.close();
          callback.close();
          return;
        }

        final [id, method, args] = message as List<Object?>;
        try {
          final result = native.execute(method! as String, args! as String);
          responses.send([id, true, result]);
        } on Object catch (error) {
          responses.send([id, false, _errorMessage(error)]);
        }
      }
      responses.send([0, true, requests.sendPort]);
    } on Object catch (error) {
      requests.close();
      callback?.close();
      responses.send([0, false, _errorMessage(error)]);
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

  final DynamicLibrary _library;

  late final _createStream = _library
      .lookupFunction<_CreateStream, _CreateStream>('stride__create_stream');
  late final _execute = _library.lookupFunction<_Execute, _Execute>(
    'stride__execute',
  );
  late final freeString = _library
      .lookupFunction<_FreeNative, void Function(Pointer<Utf8>)>(
        'stride__free_string',
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
