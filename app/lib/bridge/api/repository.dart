// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.4.0.

// ignore_for_file: require_trailing_commas
// ignore_for_file: avoid_unused_constructor_parameters
// ignore_for_file: avoid_dynamic_calls
// ignore_for_file: avoid_equals_and_hash_code_on_mutable_classes
// ignore_for_file: argument_type_not_assignable
// ignore_for_file: inference_failure_on_instance_creation

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:stride/bridge/api/error.dart';
import 'package:stride/bridge/api/filter.dart';
import 'package:stride/bridge/api/settings.dart';
import 'package:stride/bridge/frb_generated.dart';
import 'package:stride/bridge/task.dart';
import 'package:stride/bridge/task/annotation.dart';
import 'package:uuid/uuid.dart';

// These functions are ignored because they are not marked as `pub`: `append`, `clear`, `do_merge`, `fast_forward`, `filter`, `generate_iv`, `get_by_id`, `get_index`, `load`, `new`, `rebase`, `remove_task2`, `remove`, `remove`, `resolve_conflicts`, `save`, `ssh_key`, `storage_mut`, `unload`, `update2`, `update`, `with_authentication`
// These types are ignored because they are not used by any `pub` functions: `DecryptedTask`, `LogIter`, `Storage`, `TaskDiff`
// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `clone`, `fmt`
// These functions are ignored (category: IgnoreBecauseExplicitAttribute): `pull`
// These functions are ignored (category: IgnoreBecauseOwnerTyShouldIgnore): `next`

String oidToString({required Oid oid}) =>
    RustLib.instance.api.crateApiRepositoryOidToString(oid: oid);

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Oid>>
abstract class Oid implements RustOpaqueInterface {}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<TaskStorage>>
abstract class TaskStorage implements RustOpaqueInterface {
  Future<void> add({required Task task});

  Future<bool> addAndCommit({required String message});

  Future<bool> changeCategory({required Task task, required TaskStatus status});

  Future<void> checkout();

  Future<void> clear();

  Future<void> cloneRepository();

  Future<String> export_();

  Future<void> forceHardReset({required Oid commit});

  Future<void> import_({required String content});

  Future<void> initRepotitory();

  Future<List<CommitItem>?> log({Oid? oid, int? n});

  factory TaskStorage({required String path, required Settings settings}) =>
      RustLib.instance.api
          .crateApiRepositoryTaskStorageNew(path: path, settings: settings);

  Future<void> push({required bool force});

  Future<bool> removeTask({required Task task});

  Future<void> sync_();

  Future<Task?> taskByUuid({required UuidValue uuid});

  Future<List<Task>> tasks();

  Future<List<Task>> tasksWithFilter({required Filter filter});

  Future<void> unload();

  Future<bool> update({required Task task});
}

class CommitItem {
  final Oid oid;
  final Oid? parent;
  final String message;
  final String author;
  final String email;

  const CommitItem({
    required this.oid,
    this.parent,
    required this.message,
    required this.author,
    required this.email,
  });

  @override
  int get hashCode =>
      oid.hashCode ^
      parent.hashCode ^
      message.hashCode ^
      author.hashCode ^
      email.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CommitItem &&
          runtimeType == other.runtimeType &&
          oid == other.oid &&
          parent == other.parent &&
          message == other.message &&
          author == other.author &&
          email == other.email;
}
