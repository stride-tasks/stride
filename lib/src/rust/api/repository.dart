// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import '../git/known_hosts.dart';
import '../task.dart';
import '../task/annotation.dart';
import 'filter.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
import 'package:uuid/uuid.dart';
part 'repository.freezed.dart';

// These functions are ignored because they are not marked as `pub`: `append`, `clear`, `do_merge`, `fast_forward`, `filter`, `get_by_id`, `get_index`, `load`, `new`, `pull`, `remove`, `remove`, `save`, `storage_mut`, `unload`, `update`, `with_authentication`, `with_uuid`
// These types are ignored because they are not used by any `pub` functions: `Storage`
// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `clone`, `fmt`, `fmt`

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<TaskStorage>>
abstract class TaskStorage implements RustOpaqueInterface {
  Future<void> add({required Task task});

  Future<bool> addAndCommit({required String message});

  Future<bool> changeCategory({required Task task, required TaskStatus status});

  Future<void> checkout();

  Future<void> clear();

  Future<void> cloneRepository();

  Future<void> initRepotitory();

  factory TaskStorage({required String path}) =>
      RustLib.instance.api.crateApiRepositoryTaskStorageNew(path: path);

  Future<void> push();

  Future<bool> removeTask({required Task task});

  Future<void> sync();

  Future<Task?> taskByUuid({required UuidValue uuid});

  Future<List<Task>> tasks();

  Future<List<Task>> tasksWithFilter({required Filter filter});

  Future<void> unload();

  Future<bool> update({required Task task});
}

@freezed
sealed class ConnectionError with _$ConnectionError implements FrbException {
  const ConnectionError._();

  const factory ConnectionError.network({
    required String message,
  }) = ConnectionError_Network;
  const factory ConnectionError.noSshKeysProvided() =
      ConnectionError_NoSshKeysProvided;
  const factory ConnectionError.authentication({
    required String message,
  }) = ConnectionError_Authentication;
  const factory ConnectionError.unknownHost({
    required String hostname,
    required HostKeyType keyType,
    required String hostKey,
  }) = ConnectionError_UnknownHost;
  const factory ConnectionError.missingHostKey({
    required String hostname,
  }) = ConnectionError_MissingHostKey;
  const factory ConnectionError.unknownKeyType() =
      ConnectionError_UnknownKeyType;
  const factory ConnectionError.missmatchRemoteKey({
    required String expected,
    required String actual,
  }) = ConnectionError_MissmatchRemoteKey;
  const factory ConnectionError.other({
    required String message,
  }) = ConnectionError_Other;
}
