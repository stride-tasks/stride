// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';
import 'task/annotation.dart';

class Task {
  final UuidValue uuid;
  final TaskStatus status;
  final String description;
  final DateTime? modified;
  final DateTime? due;
  final int? project;
  final Uint32List tags;
  final List<Annotation> annotations;
  final int? priority;
  final DateTime? wait;
  final DateTime? end;
  final List<UuidValue> depends;
  final Map<String, String> uda;

  const Task.raw({
    required this.uuid,
    required this.status,
    required this.description,
    this.modified,
    this.due,
    this.project,
    required this.tags,
    required this.annotations,
    this.priority,
    this.wait,
    this.end,
    required this.depends,
    required this.uda,
  });

  factory Task({required String description}) =>
      RustLib.instance.api.crateTaskTaskNew(description: description);

  @override
  int get hashCode =>
      uuid.hashCode ^
      status.hashCode ^
      description.hashCode ^
      modified.hashCode ^
      due.hashCode ^
      project.hashCode ^
      tags.hashCode ^
      annotations.hashCode ^
      priority.hashCode ^
      wait.hashCode ^
      end.hashCode ^
      depends.hashCode ^
      uda.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Task &&
          runtimeType == other.runtimeType &&
          uuid == other.uuid &&
          status == other.status &&
          description == other.description &&
          modified == other.modified &&
          due == other.due &&
          project == other.project &&
          tags == other.tags &&
          annotations == other.annotations &&
          priority == other.priority &&
          wait == other.wait &&
          end == other.end &&
          depends == other.depends &&
          uda == other.uda;
}

enum TaskStatus {
  pending,
  waiting,
  recurring,
  deleted,
  complete,
  ;
}
