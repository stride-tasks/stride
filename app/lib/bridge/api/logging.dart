// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.5.0.

// ignore_for_file: require_trailing_commas
// ignore_for_file: avoid_unused_constructor_parameters
// ignore_for_file: avoid_dynamic_calls
// ignore_for_file: avoid_equals_and_hash_code_on_mutable_classes
// ignore_for_file: argument_type_not_assignable
// ignore_for_file: inference_failure_on_instance_creation

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:stride/bridge/frb_generated.dart';

// These functions are ignored because they are not marked as `pub`: `init_logger`
// These types are ignored because they are not used by any `pub` functions: `LogOutput`
// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `clone`, `flush`, `fmt`, `write`

/// # Panics
///
/// If the logger file cannot be read.
Future<String> getLogs() => RustLib.instance.api.crateApiLoggingGetLogs();

class Logger {
  const Logger();

  static Future<void> debug({required String message}) =>
      RustLib.instance.api.crateApiLoggingLoggerDebug(message: message);

  static Future<void> error({required String message}) =>
      RustLib.instance.api.crateApiLoggingLoggerError(message: message);

  static Future<void> info({required String message}) =>
      RustLib.instance.api.crateApiLoggingLoggerInfo(message: message);

  static Future<void> trace({required String message}) =>
      RustLib.instance.api.crateApiLoggingLoggerTrace(message: message);

  static Future<void> warn({required String message}) =>
      RustLib.instance.api.crateApiLoggingLoggerWarn(message: message);

  @override
  int get hashCode => 0;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Logger && runtimeType == other.runtimeType;
}
