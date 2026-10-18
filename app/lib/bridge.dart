import 'dart:async';
import 'dart:convert';
import 'dart:ffi';
import 'dart:io';
import 'dart:isolate';

import 'package:ffi/ffi.dart';

typedef _NotificationCallbackNative = Void Function(Pointer<Utf8>);
typedef _CreateStreamNative =
    Pointer<Utf8> Function(
      Pointer<Utf8>,
      Pointer<Utf8>,
      Pointer<NativeFunction<_NotificationCallbackNative>>,
    );
typedef _CreateStreamDart =
    Pointer<Utf8> Function(
      Pointer<Utf8>,
      Pointer<Utf8>,
      Pointer<NativeFunction<_NotificationCallbackNative>>,
    );
typedef _ExecuteNative = Pointer<Utf8> Function(Pointer<Utf8>, Pointer<Utf8>);
typedef _ExecuteDart = Pointer<Utf8> Function(Pointer<Utf8>, Pointer<Utf8>);
typedef _FreeStringNative = Void Function(Pointer<Utf8>);
typedef _FreeStringDart = void Function(Pointer<Utf8>);

class _RustBindings {
  static const kLibraryName = 'stride_flutter_bridge';

  static const kCreateStream = 'stride__create_stream';
  static const kExecute = 'stride__execute';
  static const kFreeString = 'stride__free_string';

  _RustBindings({String libraryName = kLibraryName})
    : _library = _openLibrary(libraryName);

  final DynamicLibrary _library;

  late final createStream = _library
      .lookupFunction<_CreateStreamNative, _CreateStreamDart>(kCreateStream);
  late final execute = _library.lookupFunction<_ExecuteNative, _ExecuteDart>(
    kExecute,
  );
  late final freeString = _library
      .lookupFunction<_FreeStringNative, _FreeStringDart>(kFreeString);

  static DynamicLibrary _openLibrary(String libraryName) {
    if (Platform.isIOS || Platform.isMacOS) {
      return DynamicLibrary.process();
    }
    final platformLibraryName = Platform.isWindows
        ? '$libraryName.dll'
        : 'lib$libraryName.so';
    return DynamicLibrary.open(platformLibraryName);
  }
}

class RustBridge {
  static final _bindings = _RustBindings();
  static final _notifications = StreamController<String>.broadcast();
  static final _notificationCallback =
      NativeCallable<_NotificationCallbackNative>.listener(_handleNotification);

  static Stream<String> createStream({
    required String supportPath,
    required String cachePath,
    bool listenToNotifications = true,
  }) {
    final supportPathPointer = supportPath.toNativeUtf8();
    final cachePathPointer = cachePath.toNativeUtf8();
    try {
      final errorPointer = _bindings.createStream(
        supportPathPointer,
        cachePathPointer,
        listenToNotifications
            ? _notificationCallback.nativeFunction
            : nullptr.cast<NativeFunction<_NotificationCallbackNative>>(),
      );
      if (errorPointer.address != 0) {
        try {
          throw StateError(
            'Failed to initialize Rust: ${errorPointer.toDartString()}',
          );
        } finally {
          _bindings.freeString(errorPointer);
        }
      }
    } finally {
      malloc
        ..free(supportPathPointer)
        ..free(cachePathPointer);
    }
    return _notifications.stream;
  }

  static Future<String> execute({
    required String method,
    required String args,
  }) => Isolate.run(() => _executeInIsolate(method, args));

  static String _executeInIsolate(String method, String args) {
    final methodPointer = method.toNativeUtf8();
    final argsPointer = args.toNativeUtf8();
    try {
      final responsePointer = _bindings.execute(methodPointer, argsPointer);
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
        _bindings.freeString(responsePointer);
      }
    } finally {
      malloc
        ..free(methodPointer)
        ..free(argsPointer);
    }
  }

  static void _handleNotification(Pointer<Utf8> messagePointer) {
    try {
      _notifications.add(messagePointer.toDartString());
      // ignore: avoid_catches_without_on_clauses
    } catch (error, stackTrace) {
      _notifications.addError(error, stackTrace);
    } finally {
      _bindings.freeString(messagePointer);
    }
  }
}
