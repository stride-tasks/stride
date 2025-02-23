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
import 'package:stride/bridge/api/error.dart';
import 'package:stride/bridge/frb_generated.dart';
import 'package:stride/bridge/third_party/stride_plugin_manager/manager.dart';
import 'package:stride/bridge/third_party/stride_plugin_manager/manifest.dart';

Future<List<PluginManifestPluginState>> pluginManifests(
        {required PluginManager pluginManager}) =>
    RustLib.instance.api
        .crateApiPluginPluginManifests(pluginManager: pluginManager);

/// flutter_rust_bridge:sync
String pluginInstanceManifestName(
        {required PluginManifestPluginState manifest}) =>
    RustLib.instance.api
        .crateApiPluginPluginInstanceManifestName(manifest: manifest);

/// flutter_rust_bridge:sync
ManifestEvents pluginInstanceManifestEvents(
        {required PluginManifestPluginState manifest}) =>
    RustLib.instance.api
        .crateApiPluginPluginInstanceManifestEvents(manifest: manifest);
