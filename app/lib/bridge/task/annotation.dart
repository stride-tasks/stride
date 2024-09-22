// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.4.0.

// ignore_for_file: require_trailing_commas
// ignore_for_file: always_use_package_imports
// ignore_for_file: directives_ordering
// ignore_for_file: sort_unnamed_constructors_first
// ignore_for_file: avoid_unused_constructor_parameters
// ignore_for_file: avoid_dynamic_calls
// ignore_for_file: avoid_equals_and_hash_code_on_mutable_classes
// ignore_for_file: prefer_final_locals
// ignore_for_file: argument_type_not_assignable
// ignore_for_file: inference_failure_on_instance_creation
// ignore_for_file: prefer_single_quotes
// ignore_for_file: avoid_redundant_argument_values
// ignore_for_file: noop_primitive_operations
// ignore_for_file: prefer_const_constructors
// ignore_for_file: prefer_is_empty
// ignore_for_file: unnecessary_parenthesis

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

class Annotation {
  final DateTime entry;
  final String description;

  const Annotation({
    required this.entry,
    required this.description,
  });

  @override
  int get hashCode => entry.hashCode ^ description.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Annotation &&
          runtimeType == other.runtimeType &&
          entry == other.entry &&
          description == other.description;
}
