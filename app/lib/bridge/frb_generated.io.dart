// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.4.0.

// ignore_for_file: require_trailing_commas
// ignore_for_file: avoid_unused_constructor_parameters
// ignore_for_file: avoid_dynamic_calls
// ignore_for_file: avoid_equals_and_hash_code_on_mutable_classes
// ignore_for_file: argument_type_not_assignable
// ignore_for_file: inference_failure_on_instance_creation

// ignore_for_file: unused_import, unused_element, unnecessary_import, duplicate_ignore, invalid_use_of_internal_member, annotate_overrides, non_constant_identifier_names, curly_braces_in_flow_control_structures, prefer_const_literals_to_create_immutables, unused_field

import 'dart:async';
import 'dart:convert';
import 'dart:ffi' as ffi;

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_io.dart';
import 'package:stride/bridge/api/error.dart';
import 'package:stride/bridge/api/filter.dart';
import 'package:stride/bridge/api/logging.dart';
import 'package:stride/bridge/api/repository.dart';
import 'package:stride/bridge/api/settings.dart';
import 'package:stride/bridge/frb_generated.dart';
import 'package:stride/bridge/git/known_hosts.dart';
import 'package:stride/bridge/task.dart';
import 'package:stride/bridge/task/annotation.dart';
import 'package:uuid/uuid.dart';

abstract class RustLibApiImplPlatform extends BaseApiImpl<RustLibWire> {
  RustLibApiImplPlatform({
    required super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.portManager,
  });

  CrossPlatformFinalizerArg get rust_arc_decrement_strong_count_OidPtr => wire
      ._rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOidPtr;

  CrossPlatformFinalizerArg get rust_arc_decrement_strong_count_RustErrorPtr =>
      wire._rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustErrorPtr;

  CrossPlatformFinalizerArg get rust_arc_decrement_strong_count_SshKeyPtr => wire
      ._rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSshKeyPtr;

  CrossPlatformFinalizerArg
      get rust_arc_decrement_strong_count_TaskStoragePtr => wire
          ._rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTaskStoragePtr;

  @protected
  AnyhowException dco_decode_AnyhowException(dynamic raw);

  @protected
  Oid dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOid(
      dynamic raw);

  @protected
  RustError
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustError(
          dynamic raw);

  @protected
  SshKey
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSshKey(
          dynamic raw);

  @protected
  TaskStorage
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTaskStorage(
          dynamic raw);

  @protected
  TaskStorage
      dco_decode_Auto_RefMut_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTaskStorage(
          dynamic raw);

  @protected
  Oid dco_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOid(
      dynamic raw);

  @protected
  RustError
      dco_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustError(
          dynamic raw);

  @protected
  SshKey
      dco_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSshKey(
          dynamic raw);

  @protected
  TaskStorage
      dco_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTaskStorage(
          dynamic raw);

  @protected
  DateTime dco_decode_Chrono_Utc(dynamic raw);

  @protected
  Map<String, String> dco_decode_Map_String_String(dynamic raw);

  @protected
  Oid dco_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOid(
      dynamic raw);

  @protected
  RustError
      dco_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustError(
          dynamic raw);

  @protected
  SshKey
      dco_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSshKey(
          dynamic raw);

  @protected
  TaskStorage
      dco_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTaskStorage(
          dynamic raw);

  @protected
  Set<TaskStatus> dco_decode_Set_task_status(dynamic raw);

  @protected
  RustStreamSink<Settings> dco_decode_StreamSink_settings_Sse(dynamic raw);

  @protected
  String dco_decode_String(dynamic raw);

  @protected
  UuidValue dco_decode_Uuid(dynamic raw);

  @protected
  Annotation dco_decode_annotation(dynamic raw);

  @protected
  ApplicationPaths dco_decode_application_paths(dynamic raw);

  @protected
  bool dco_decode_bool(dynamic raw);

  @protected
  Oid dco_decode_box_autoadd_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOid(
      dynamic raw);

  @protected
  DateTime dco_decode_box_autoadd_Chrono_Utc(dynamic raw);

  @protected
  ApplicationPaths dco_decode_box_autoadd_application_paths(dynamic raw);

  @protected
  EncryptionKey dco_decode_box_autoadd_encryption_key(dynamic raw);

  @protected
  Filter dco_decode_box_autoadd_filter(dynamic raw);

  @protected
  FilterSelection dco_decode_box_autoadd_filter_selection(dynamic raw);

  @protected
  Host dco_decode_box_autoadd_host(dynamic raw);

  @protected
  KnownHosts dco_decode_box_autoadd_known_hosts(dynamic raw);

  @protected
  Settings dco_decode_box_autoadd_settings(dynamic raw);

  @protected
  Task dco_decode_box_autoadd_task(dynamic raw);

  @protected
  TaskPriority dco_decode_box_autoadd_task_priority(dynamic raw);

  @protected
  int dco_decode_box_autoadd_u_32(dynamic raw);

  @protected
  CommitItem dco_decode_commit_item(dynamic raw);

  @protected
  EncryptionKey dco_decode_encryption_key(dynamic raw);

  @protected
  double dco_decode_f_32(dynamic raw);

  @protected
  Filter dco_decode_filter(dynamic raw);

  @protected
  FilterSelection dco_decode_filter_selection(dynamic raw);

  @protected
  Host dco_decode_host(dynamic raw);

  @protected
  HostKeyType dco_decode_host_key_type(dynamic raw);

  @protected
  int dco_decode_i_32(dynamic raw);

  @protected
  PlatformInt64 dco_decode_i_64(dynamic raw);

  @protected
  KnownHosts dco_decode_known_hosts(dynamic raw);

  @protected
  List<SshKey>
      dco_decode_list_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSshKey(
          dynamic raw);

  @protected
  List<UuidValue> dco_decode_list_Uuid(dynamic raw);

  @protected
  List<Annotation> dco_decode_list_annotation(dynamic raw);

  @protected
  List<CommitItem> dco_decode_list_commit_item(dynamic raw);

  @protected
  List<Filter> dco_decode_list_filter(dynamic raw);

  @protected
  List<Host> dco_decode_list_host(dynamic raw);

  @protected
  Uint32List dco_decode_list_prim_u_32_strict(dynamic raw);

  @protected
  Uint8List dco_decode_list_prim_u_8_strict(dynamic raw);

  @protected
  List<(String, String)> dco_decode_list_record_string_string(dynamic raw);

  @protected
  List<Task> dco_decode_list_task(dynamic raw);

  @protected
  List<TaskStatus> dco_decode_list_task_status(dynamic raw);

  @protected
  Logger dco_decode_logger(dynamic raw);

  @protected
  UuidValue? dco_decode_opt_Uuid(dynamic raw);

  @protected
  Oid?
      dco_decode_opt_box_autoadd_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOid(
          dynamic raw);

  @protected
  DateTime? dco_decode_opt_box_autoadd_Chrono_Utc(dynamic raw);

  @protected
  EncryptionKey? dco_decode_opt_box_autoadd_encryption_key(dynamic raw);

  @protected
  FilterSelection? dco_decode_opt_box_autoadd_filter_selection(dynamic raw);

  @protected
  Host? dco_decode_opt_box_autoadd_host(dynamic raw);

  @protected
  Task? dco_decode_opt_box_autoadd_task(dynamic raw);

  @protected
  TaskPriority? dco_decode_opt_box_autoadd_task_priority(dynamic raw);

  @protected
  int? dco_decode_opt_box_autoadd_u_32(dynamic raw);

  @protected
  List<CommitItem>? dco_decode_opt_list_commit_item(dynamic raw);

  @protected
  (String, String) dco_decode_record_string_string(dynamic raw);

  @protected
  Repository dco_decode_repository(dynamic raw);

  @protected
  Settings dco_decode_settings(dynamic raw);

  @protected
  Task dco_decode_task(dynamic raw);

  @protected
  TaskPriority dco_decode_task_priority(dynamic raw);

  @protected
  TaskStatus dco_decode_task_status(dynamic raw);

  @protected
  int dco_decode_u_32(dynamic raw);

  @protected
  int dco_decode_u_8(dynamic raw);

  @protected
  void dco_decode_unit(dynamic raw);

  @protected
  BigInt dco_decode_usize(dynamic raw);

  @protected
  AnyhowException sse_decode_AnyhowException(SseDeserializer deserializer);

  @protected
  Oid sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOid(
      SseDeserializer deserializer);

  @protected
  RustError
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustError(
          SseDeserializer deserializer);

  @protected
  SshKey
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSshKey(
          SseDeserializer deserializer);

  @protected
  TaskStorage
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTaskStorage(
          SseDeserializer deserializer);

  @protected
  TaskStorage
      sse_decode_Auto_RefMut_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTaskStorage(
          SseDeserializer deserializer);

  @protected
  Oid sse_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOid(
      SseDeserializer deserializer);

  @protected
  RustError
      sse_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustError(
          SseDeserializer deserializer);

  @protected
  SshKey
      sse_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSshKey(
          SseDeserializer deserializer);

  @protected
  TaskStorage
      sse_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTaskStorage(
          SseDeserializer deserializer);

  @protected
  DateTime sse_decode_Chrono_Utc(SseDeserializer deserializer);

  @protected
  Map<String, String> sse_decode_Map_String_String(
      SseDeserializer deserializer);

  @protected
  Oid sse_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOid(
      SseDeserializer deserializer);

  @protected
  RustError
      sse_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustError(
          SseDeserializer deserializer);

  @protected
  SshKey
      sse_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSshKey(
          SseDeserializer deserializer);

  @protected
  TaskStorage
      sse_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTaskStorage(
          SseDeserializer deserializer);

  @protected
  Set<TaskStatus> sse_decode_Set_task_status(SseDeserializer deserializer);

  @protected
  RustStreamSink<Settings> sse_decode_StreamSink_settings_Sse(
      SseDeserializer deserializer);

  @protected
  String sse_decode_String(SseDeserializer deserializer);

  @protected
  UuidValue sse_decode_Uuid(SseDeserializer deserializer);

  @protected
  Annotation sse_decode_annotation(SseDeserializer deserializer);

  @protected
  ApplicationPaths sse_decode_application_paths(SseDeserializer deserializer);

  @protected
  bool sse_decode_bool(SseDeserializer deserializer);

  @protected
  Oid sse_decode_box_autoadd_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOid(
      SseDeserializer deserializer);

  @protected
  DateTime sse_decode_box_autoadd_Chrono_Utc(SseDeserializer deserializer);

  @protected
  ApplicationPaths sse_decode_box_autoadd_application_paths(
      SseDeserializer deserializer);

  @protected
  EncryptionKey sse_decode_box_autoadd_encryption_key(
      SseDeserializer deserializer);

  @protected
  Filter sse_decode_box_autoadd_filter(SseDeserializer deserializer);

  @protected
  FilterSelection sse_decode_box_autoadd_filter_selection(
      SseDeserializer deserializer);

  @protected
  Host sse_decode_box_autoadd_host(SseDeserializer deserializer);

  @protected
  KnownHosts sse_decode_box_autoadd_known_hosts(SseDeserializer deserializer);

  @protected
  Settings sse_decode_box_autoadd_settings(SseDeserializer deserializer);

  @protected
  Task sse_decode_box_autoadd_task(SseDeserializer deserializer);

  @protected
  TaskPriority sse_decode_box_autoadd_task_priority(
      SseDeserializer deserializer);

  @protected
  int sse_decode_box_autoadd_u_32(SseDeserializer deserializer);

  @protected
  CommitItem sse_decode_commit_item(SseDeserializer deserializer);

  @protected
  EncryptionKey sse_decode_encryption_key(SseDeserializer deserializer);

  @protected
  double sse_decode_f_32(SseDeserializer deserializer);

  @protected
  Filter sse_decode_filter(SseDeserializer deserializer);

  @protected
  FilterSelection sse_decode_filter_selection(SseDeserializer deserializer);

  @protected
  Host sse_decode_host(SseDeserializer deserializer);

  @protected
  HostKeyType sse_decode_host_key_type(SseDeserializer deserializer);

  @protected
  int sse_decode_i_32(SseDeserializer deserializer);

  @protected
  PlatformInt64 sse_decode_i_64(SseDeserializer deserializer);

  @protected
  KnownHosts sse_decode_known_hosts(SseDeserializer deserializer);

  @protected
  List<SshKey>
      sse_decode_list_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSshKey(
          SseDeserializer deserializer);

  @protected
  List<UuidValue> sse_decode_list_Uuid(SseDeserializer deserializer);

  @protected
  List<Annotation> sse_decode_list_annotation(SseDeserializer deserializer);

  @protected
  List<CommitItem> sse_decode_list_commit_item(SseDeserializer deserializer);

  @protected
  List<Filter> sse_decode_list_filter(SseDeserializer deserializer);

  @protected
  List<Host> sse_decode_list_host(SseDeserializer deserializer);

  @protected
  Uint32List sse_decode_list_prim_u_32_strict(SseDeserializer deserializer);

  @protected
  Uint8List sse_decode_list_prim_u_8_strict(SseDeserializer deserializer);

  @protected
  List<(String, String)> sse_decode_list_record_string_string(
      SseDeserializer deserializer);

  @protected
  List<Task> sse_decode_list_task(SseDeserializer deserializer);

  @protected
  List<TaskStatus> sse_decode_list_task_status(SseDeserializer deserializer);

  @protected
  Logger sse_decode_logger(SseDeserializer deserializer);

  @protected
  UuidValue? sse_decode_opt_Uuid(SseDeserializer deserializer);

  @protected
  Oid?
      sse_decode_opt_box_autoadd_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOid(
          SseDeserializer deserializer);

  @protected
  DateTime? sse_decode_opt_box_autoadd_Chrono_Utc(SseDeserializer deserializer);

  @protected
  EncryptionKey? sse_decode_opt_box_autoadd_encryption_key(
      SseDeserializer deserializer);

  @protected
  FilterSelection? sse_decode_opt_box_autoadd_filter_selection(
      SseDeserializer deserializer);

  @protected
  Host? sse_decode_opt_box_autoadd_host(SseDeserializer deserializer);

  @protected
  Task? sse_decode_opt_box_autoadd_task(SseDeserializer deserializer);

  @protected
  TaskPriority? sse_decode_opt_box_autoadd_task_priority(
      SseDeserializer deserializer);

  @protected
  int? sse_decode_opt_box_autoadd_u_32(SseDeserializer deserializer);

  @protected
  List<CommitItem>? sse_decode_opt_list_commit_item(
      SseDeserializer deserializer);

  @protected
  (String, String) sse_decode_record_string_string(
      SseDeserializer deserializer);

  @protected
  Repository sse_decode_repository(SseDeserializer deserializer);

  @protected
  Settings sse_decode_settings(SseDeserializer deserializer);

  @protected
  Task sse_decode_task(SseDeserializer deserializer);

  @protected
  TaskPriority sse_decode_task_priority(SseDeserializer deserializer);

  @protected
  TaskStatus sse_decode_task_status(SseDeserializer deserializer);

  @protected
  int sse_decode_u_32(SseDeserializer deserializer);

  @protected
  int sse_decode_u_8(SseDeserializer deserializer);

  @protected
  void sse_decode_unit(SseDeserializer deserializer);

  @protected
  BigInt sse_decode_usize(SseDeserializer deserializer);

  @protected
  void sse_encode_AnyhowException(
      AnyhowException self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOid(
          Oid self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustError(
          RustError self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSshKey(
          SshKey self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTaskStorage(
          TaskStorage self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_RefMut_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTaskStorage(
          TaskStorage self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOid(
          Oid self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustError(
          RustError self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSshKey(
          SshKey self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTaskStorage(
          TaskStorage self, SseSerializer serializer);

  @protected
  void sse_encode_Chrono_Utc(DateTime self, SseSerializer serializer);

  @protected
  void sse_encode_Map_String_String(
      Map<String, String> self, SseSerializer serializer);

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOid(
          Oid self, SseSerializer serializer);

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustError(
          RustError self, SseSerializer serializer);

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSshKey(
          SshKey self, SseSerializer serializer);

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTaskStorage(
          TaskStorage self, SseSerializer serializer);

  @protected
  void sse_encode_Set_task_status(
      Set<TaskStatus> self, SseSerializer serializer);

  @protected
  void sse_encode_StreamSink_settings_Sse(
      RustStreamSink<Settings> self, SseSerializer serializer);

  @protected
  void sse_encode_String(String self, SseSerializer serializer);

  @protected
  void sse_encode_Uuid(UuidValue self, SseSerializer serializer);

  @protected
  void sse_encode_annotation(Annotation self, SseSerializer serializer);

  @protected
  void sse_encode_application_paths(
      ApplicationPaths self, SseSerializer serializer);

  @protected
  void sse_encode_bool(bool self, SseSerializer serializer);

  @protected
  void
      sse_encode_box_autoadd_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOid(
          Oid self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_Chrono_Utc(
      DateTime self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_application_paths(
      ApplicationPaths self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_encryption_key(
      EncryptionKey self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_filter(Filter self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_filter_selection(
      FilterSelection self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_host(Host self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_known_hosts(
      KnownHosts self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_settings(Settings self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_task(Task self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_task_priority(
      TaskPriority self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_u_32(int self, SseSerializer serializer);

  @protected
  void sse_encode_commit_item(CommitItem self, SseSerializer serializer);

  @protected
  void sse_encode_encryption_key(EncryptionKey self, SseSerializer serializer);

  @protected
  void sse_encode_f_32(double self, SseSerializer serializer);

  @protected
  void sse_encode_filter(Filter self, SseSerializer serializer);

  @protected
  void sse_encode_filter_selection(
      FilterSelection self, SseSerializer serializer);

  @protected
  void sse_encode_host(Host self, SseSerializer serializer);

  @protected
  void sse_encode_host_key_type(HostKeyType self, SseSerializer serializer);

  @protected
  void sse_encode_i_32(int self, SseSerializer serializer);

  @protected
  void sse_encode_i_64(PlatformInt64 self, SseSerializer serializer);

  @protected
  void sse_encode_known_hosts(KnownHosts self, SseSerializer serializer);

  @protected
  void
      sse_encode_list_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSshKey(
          List<SshKey> self, SseSerializer serializer);

  @protected
  void sse_encode_list_Uuid(List<UuidValue> self, SseSerializer serializer);

  @protected
  void sse_encode_list_annotation(
      List<Annotation> self, SseSerializer serializer);

  @protected
  void sse_encode_list_commit_item(
      List<CommitItem> self, SseSerializer serializer);

  @protected
  void sse_encode_list_filter(List<Filter> self, SseSerializer serializer);

  @protected
  void sse_encode_list_host(List<Host> self, SseSerializer serializer);

  @protected
  void sse_encode_list_prim_u_32_strict(
      Uint32List self, SseSerializer serializer);

  @protected
  void sse_encode_list_prim_u_8_strict(
      Uint8List self, SseSerializer serializer);

  @protected
  void sse_encode_list_record_string_string(
      List<(String, String)> self, SseSerializer serializer);

  @protected
  void sse_encode_list_task(List<Task> self, SseSerializer serializer);

  @protected
  void sse_encode_list_task_status(
      List<TaskStatus> self, SseSerializer serializer);

  @protected
  void sse_encode_logger(Logger self, SseSerializer serializer);

  @protected
  void sse_encode_opt_Uuid(UuidValue? self, SseSerializer serializer);

  @protected
  void
      sse_encode_opt_box_autoadd_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOid(
          Oid? self, SseSerializer serializer);

  @protected
  void sse_encode_opt_box_autoadd_Chrono_Utc(
      DateTime? self, SseSerializer serializer);

  @protected
  void sse_encode_opt_box_autoadd_encryption_key(
      EncryptionKey? self, SseSerializer serializer);

  @protected
  void sse_encode_opt_box_autoadd_filter_selection(
      FilterSelection? self, SseSerializer serializer);

  @protected
  void sse_encode_opt_box_autoadd_host(Host? self, SseSerializer serializer);

  @protected
  void sse_encode_opt_box_autoadd_task(Task? self, SseSerializer serializer);

  @protected
  void sse_encode_opt_box_autoadd_task_priority(
      TaskPriority? self, SseSerializer serializer);

  @protected
  void sse_encode_opt_box_autoadd_u_32(int? self, SseSerializer serializer);

  @protected
  void sse_encode_opt_list_commit_item(
      List<CommitItem>? self, SseSerializer serializer);

  @protected
  void sse_encode_record_string_string(
      (String, String) self, SseSerializer serializer);

  @protected
  void sse_encode_repository(Repository self, SseSerializer serializer);

  @protected
  void sse_encode_settings(Settings self, SseSerializer serializer);

  @protected
  void sse_encode_task(Task self, SseSerializer serializer);

  @protected
  void sse_encode_task_priority(TaskPriority self, SseSerializer serializer);

  @protected
  void sse_encode_task_status(TaskStatus self, SseSerializer serializer);

  @protected
  void sse_encode_u_32(int self, SseSerializer serializer);

  @protected
  void sse_encode_u_8(int self, SseSerializer serializer);

  @protected
  void sse_encode_unit(void self, SseSerializer serializer);

  @protected
  void sse_encode_usize(BigInt self, SseSerializer serializer);
}

// Section: wire_class

class RustLibWire implements BaseWire {
  /// The symbols are looked up in [dynamicLibrary].
  RustLibWire(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;

  factory RustLibWire.fromExternalLibrary(ExternalLibrary lib) =>
      RustLibWire(lib.ffiDynamicLibrary);

  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  void
      rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOid(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOid(
      ptr,
    );
  }

  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOidPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_stride_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOid');
  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOid =
      _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOidPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void
      rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOid(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOid(
      ptr,
    );
  }

  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOidPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_stride_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOid');
  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOid =
      _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOidPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void
      rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustError(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustError(
      ptr,
    );
  }

  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustErrorPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_stride_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustError');
  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustError =
      _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustErrorPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void
      rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustError(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustError(
      ptr,
    );
  }

  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustErrorPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_stride_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustError');
  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustError =
      _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustErrorPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void
      rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSshKey(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSshKey(
      ptr,
    );
  }

  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSshKeyPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_stride_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSshKey');
  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSshKey =
      _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSshKeyPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void
      rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSshKey(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSshKey(
      ptr,
    );
  }

  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSshKeyPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_stride_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSshKey');
  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSshKey =
      _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSshKeyPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void
      rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTaskStorage(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTaskStorage(
      ptr,
    );
  }

  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTaskStoragePtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_stride_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTaskStorage');
  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTaskStorage =
      _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTaskStoragePtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void
      rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTaskStorage(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTaskStorage(
      ptr,
    );
  }

  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTaskStoragePtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_stride_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTaskStorage');
  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTaskStorage =
      _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerTaskStoragePtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();
}
