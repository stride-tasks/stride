// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.9.0.

// ignore_for_file: require_trailing_commas
// ignore_for_file: avoid_unused_constructor_parameters
// ignore_for_file: avoid_dynamic_calls
// ignore_for_file: avoid_equals_and_hash_code_on_mutable_classes
// ignore_for_file: argument_type_not_assignable
// ignore_for_file: inference_failure_on_instance_creation

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:stride/bridge/frb_generated.dart';

/// # Panics
///
/// If the logger file cannot be read.
Future<String> getLogs() => RustLib.instance.api.crateApiLoggingGetLogs();

Future<void> debug({required String message}) =>
    RustLib.instance.api.crateApiLoggingDebug(message: message);

Future<void> trace({required String message}) =>
    RustLib.instance.api.crateApiLoggingTrace(message: message);

Future<void> info({required String message}) =>
    RustLib.instance.api.crateApiLoggingInfo(message: message);

Future<void> warn({required String message}) =>
    RustLib.instance.api.crateApiLoggingWarn(message: message);

Future<void> error({required String message}) =>
    RustLib.instance.api.crateApiLoggingError(message: message);
