// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.8.0.

// ignore_for_file: require_trailing_commas
// ignore_for_file: avoid_unused_constructor_parameters
// ignore_for_file: avoid_dynamic_calls
// ignore_for_file: avoid_equals_and_hash_code_on_mutable_classes
// ignore_for_file: argument_type_not_assignable
// ignore_for_file: inference_failure_on_instance_creation

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
import 'package:stride/bridge/frb_generated.dart';
import 'package:stride/bridge/third_party/stride_core/event.dart';
import 'package:stride/bridge/third_party/stride_core/task.dart';
import 'package:stride/bridge/third_party/stride_core/task/annotation.dart';
import 'package:uuid/uuid.dart';

part 'manifest.freezed.dart';

// These types are ignored because they are neither used by any `pub` functions nor (for structs and enums) marked `#[frb(unignore)]`: `PluginApi`
// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `assert_receiver_is_total_eq`, `assert_receiver_is_total_eq`, `assert_receiver_is_total_eq`, `assert_receiver_is_total_eq`, `assert_receiver_is_total_eq`, `assert_receiver_is_total_eq`, `clone`, `clone`, `clone`, `clone`, `clone`, `clone`, `clone`, `clone`, `clone`, `clone`, `cmp`, `cmp`, `cmp`, `cmp`, `cmp`, `cmp`, `eq`, `eq`, `eq`, `eq`, `eq`, `eq`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `hash`, `hash`, `hash`, `hash`, `hash`, `hash`, `partial_cmp`, `partial_cmp`, `partial_cmp`, `partial_cmp`, `partial_cmp`, `partial_cmp`
// These functions are ignored (category: IgnoreBecauseNotAllowedOwner): `skip_serializing`
// These functions are ignored (category: IgnoreBecauseOwnerTyShouldIgnore): `events`, `name`

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<PluginManifest < PluginState >>>
abstract class PluginManifestPluginState implements RustOpaqueInterface {}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<PluginState>>
abstract class PluginState implements RustOpaqueInterface, ManifestState {
  static Future<PluginState> default_() =>
      RustLib.instance.api.stridePluginManagerManifestPluginStateDefault();

  Future<bool> isEnabled();

  @override
  Future<bool> skipSerializing();
}

abstract class ManifestState {
  Future<bool> skipSerializing();
}

class ManifestEventTask {
  final bool create;
  final bool modify;
  final bool sync_;

  const ManifestEventTask({
    required this.create,
    required this.modify,
    required this.sync_,
  });

  static Future<ManifestEventTask> default_() => RustLib.instance.api
      .stridePluginManagerManifestManifestEventTaskDefault();

  @override
  int get hashCode => create.hashCode ^ modify.hashCode ^ sync_.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ManifestEventTask &&
          runtimeType == other.runtimeType &&
          create == other.create &&
          modify == other.modify &&
          sync_ == other.sync_;
}

class ManifestEventTimer {
  final int interval;

  const ManifestEventTimer({
    required this.interval,
  });

  static Future<ManifestEventTimer> default_() => RustLib.instance.api
      .stridePluginManagerManifestManifestEventTimerDefault();

  @override
  int get hashCode => interval.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ManifestEventTimer &&
          runtimeType == other.runtimeType &&
          interval == other.interval;
}

class ManifestEvents {
  final ManifestEventTask task;
  final ManifestEventTimer? timer;

  const ManifestEvents({
    required this.task,
    this.timer,
  });

  static Future<ManifestEvents> default_() =>
      RustLib.instance.api.stridePluginManagerManifestManifestEventsDefault();

  @override
  int get hashCode => task.hashCode ^ timer.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ManifestEvents &&
          runtimeType == other.runtimeType &&
          task == other.task &&
          timer == other.timer;
}

class ManifestPermissionNetwork {
  final List<String> urls;

  const ManifestPermissionNetwork({
    required this.urls,
  });

  static Future<ManifestPermissionNetwork> default_() => RustLib.instance.api
      .stridePluginManagerManifestManifestPermissionNetworkDefault();

  @override
  int get hashCode => urls.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ManifestPermissionNetwork &&
          runtimeType == other.runtimeType &&
          urls == other.urls;
}

class ManifestPermissionTask {
  final bool create;
  final bool modify;
  final bool sync_;

  const ManifestPermissionTask({
    required this.create,
    required this.modify,
    required this.sync_,
  });

  static Future<ManifestPermissionTask> default_() => RustLib.instance.api
      .stridePluginManagerManifestManifestPermissionTaskDefault();

  @override
  int get hashCode => create.hashCode ^ modify.hashCode ^ sync_.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ManifestPermissionTask &&
          runtimeType == other.runtimeType &&
          create == other.create &&
          modify == other.modify &&
          sync_ == other.sync_;
}

class ManifestPermissions {
  final ManifestPermissionTask task;
  final ManifestPermissionNetwork? network;

  const ManifestPermissions({
    required this.task,
    this.network,
  });

  static Future<ManifestPermissions> default_() => RustLib.instance.api
      .stridePluginManagerManifestManifestPermissionsDefault();

  @override
  int get hashCode => task.hashCode ^ network.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ManifestPermissions &&
          runtimeType == other.runtimeType &&
          task == other.task &&
          network == other.network;
}

@freezed
sealed class PluginAction with _$PluginAction {
  const PluginAction._();

  const factory PluginAction.event({
    required String pluginName,
    required PluginEvent event,
  }) = PluginAction_Event;
  const factory PluginAction.disable({
    required String pluginName,
    required String reason,
  }) = PluginAction_Disable;
}
