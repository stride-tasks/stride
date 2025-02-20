// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.7.1.

// ignore_for_file: require_trailing_commas
// ignore_for_file: avoid_unused_constructor_parameters
// ignore_for_file: avoid_dynamic_calls
// ignore_for_file: avoid_equals_and_hash_code_on_mutable_classes
// ignore_for_file: argument_type_not_assignable
// ignore_for_file: inference_failure_on_instance_creation


// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
import 'package:uuid/uuid.dart';
import 'task/annotation.dart';
part 'task.freezed.dart';

            

            

            @freezed
class Task with _$Task  {
                const Task._();
                const factory Task.raw({ required  UuidValue uuid, required  TaskStatus status, required  String title, required  bool active,  DateTime? modified,  DateTime? due,  int? project, required  Uint32List tags, required  List<Annotation> annotations,  TaskPriority? priority,  DateTime? wait, required  List<UuidValue> depends, required  Map<String, String> uda,}) = _Task;
                factory Task({required String title })=>RustLib.instance.api.crateTaskTaskNew(title: title);


 double  urgency()=>RustLib.instance.api.crateTaskTaskUrgency(that: this, );


static Future<Task>  withUuid({required UuidValue uuid , required String title })=>RustLib.instance.api.crateTaskTaskWithUuid(uuid: uuid, title: title);


                
            }

enum TaskPriority {
                    h,
m,
l,
                    ;
                    
                }

enum TaskStatus {
                    pending,
waiting,
recurring,
deleted,
complete,
                    ;
                    
                }
            