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
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
import 'package:stride/bridge/api/error.dart';
import 'package:stride/bridge/api/filter.dart';
import 'package:stride/bridge/frb_generated.dart';
import 'package:stride/bridge/third_party/stride_core/task.dart';
import 'package:uuid/uuid.dart';

part 'settings.freezed.dart';

// These functions are ignored because they are not marked as `pub`: `application_cache_path`, `application_document_path`, `application_log_path`, `application_support_path`, `default_author`, `default_branch_name`, `default_email`, `default_repository_name`, `default_theme_mode`, `repository_mut`, `repository`, `ssh_key_path`, `ssh_key`
// These types are ignored because they are neither used by any `pub` functions nor (for structs and enums) marked `#[frb(unignore)]`: `State`
// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `clone`, `clone`, `clone`, `clone`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`
// These functions are ignored (category: IgnoreBecauseOwnerTyShouldIgnore): `default`

Future<List<SshKey>> sshKeys() =>
    RustLib.instance.api.crateApiSettingsSshKeys();

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SshKey>>
abstract class SshKey implements RustOpaqueInterface {
  static Future<SshKey> generate() =>
      RustLib.instance.api.crateApiSettingsSshKeyGenerate();

  String get publicKey;

  static Future<void> removeKey({required UuidValue uuid}) =>
      RustLib.instance.api.crateApiSettingsSshKeyRemoveKey(uuid: uuid);

  static Future<SshKey> save(
          {required String publicKey, required String privateKey}) =>
      RustLib.instance.api.crateApiSettingsSshKeySave(
          publicKey: publicKey, privateKey: privateKey);

  static Future<SshKey> update(
          {required UuidValue uuid,
          required String publicKey,
          required String privateKey}) =>
      RustLib.instance.api.crateApiSettingsSshKeyUpdate(
          uuid: uuid, publicKey: publicKey, privateKey: privateKey);

  UuidValue get uuid;
}

class ApplicationPaths {
  final String supportPath;
  final String documentPath;
  final String cachePath;
  final String logPath;

  const ApplicationPaths({
    required this.supportPath,
    required this.documentPath,
    required this.cachePath,
    required this.logPath,
  });

  static Future<ApplicationPaths> default_() =>
      RustLib.instance.api.crateApiSettingsApplicationPathsDefault();

  @override
  int get hashCode =>
      supportPath.hashCode ^
      documentPath.hashCode ^
      cachePath.hashCode ^
      logPath.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ApplicationPaths &&
          runtimeType == other.runtimeType &&
          supportPath == other.supportPath &&
          documentPath == other.documentPath &&
          cachePath == other.cachePath &&
          logPath == other.logPath;
}

class EncryptionKey {
  final String key;

  const EncryptionKey({
    required this.key,
  });

  static Future<EncryptionKey> generate() =>
      RustLib.instance.api.crateApiSettingsEncryptionKeyGenerate();

  static Future<bool> removeKey({required UuidValue repositoryUuid}) => RustLib
      .instance.api
      .crateApiSettingsEncryptionKeyRemoveKey(repositoryUuid: repositoryUuid);

  static Future<EncryptionKey> save(
          {required UuidValue repositoryUuid, required String key}) =>
      RustLib.instance.api.crateApiSettingsEncryptionKeySave(
          repositoryUuid: repositoryUuid, key: key);

  static String? validate({required String key}) =>
      RustLib.instance.api.crateApiSettingsEncryptionKeyValidate(key: key);

  @override
  int get hashCode => key.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is EncryptionKey &&
          runtimeType == other.runtimeType &&
          key == other.key;
}

@freezed
class Repository with _$Repository {
  const factory Repository({
    required UuidValue uuid,
    required String name,
    required String origin,
    required String author,
    required String email,
    required String branch,
    UuidValue? sshKeyUuid,
    EncryptionKey? encryption,
  }) = _Repository;
  const Repository._();
  static Future<Repository> default_() =>
      RustLib.instance.api.crateApiSettingsRepositoryDefault();
}

@freezed
class Settings with _$Settings {
  factory Settings() => RustLib.instance.api.crateApiSettingsSettingsNew();
  const Settings._();
  const factory Settings.raw({
    required bool darkMode,
    required bool periodicSync,
    required List<Filter> filters,
    FilterSelection? selectedFilter,
    UuidValue? currentRepository,
    required List<Repository> repositories,
  }) = _Settings;
  static Stream<Settings> createStream() =>
      RustLib.instance.api.crateApiSettingsSettingsCreateStream();

  static Future<Settings> default_() =>
      RustLib.instance.api.crateApiSettingsSettingsDefault();

  static Future<Settings> get_() =>
      RustLib.instance.api.crateApiSettingsSettingsGet();

  static Future<Settings> load({required ApplicationPaths paths}) =>
      RustLib.instance.api.crateApiSettingsSettingsLoad(paths: paths);

  static Future<void> save({required Settings settings}) =>
      RustLib.instance.api.crateApiSettingsSettingsSave(settings: settings);
}
