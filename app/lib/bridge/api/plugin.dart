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
import 'package:stride/bridge/third_party/stride_core/event.dart';
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
bool pluginInstanceManifestEnabled(
        {required PluginManifestPluginState manifest}) =>
    RustLib.instance.api
        .crateApiPluginPluginInstanceManifestEnabled(manifest: manifest);

/// flutter_rust_bridge:sync
String? pluginInstanceManifestDisabledReason(
        {required PluginManifestPluginState manifest}) =>
    RustLib.instance.api
        .crateApiPluginPluginInstanceManifestDisabledReason(manifest: manifest);

/// flutter_rust_bridge:sync
ManifestEvents pluginInstanceManifestEvents(
        {required PluginManifestPluginState manifest}) =>
    RustLib.instance.api
        .crateApiPluginPluginInstanceManifestEvents(manifest: manifest);

Future<void> pluginManagerEmit(
        {required PluginManager pluginManager, required HostEvent event}) =>
    RustLib.instance.api.crateApiPluginPluginManagerEmit(
        pluginManager: pluginManager, event: event);

Future<void> pluginManagerImport(
        {required PluginManager pluginManager, required String filepath}) =>
    RustLib.instance.api.crateApiPluginPluginManagerImport(
        pluginManager: pluginManager, filepath: filepath);

Future<bool> pluginManagerRemove(
        {required PluginManager pluginManager, required String pluginName}) =>
    RustLib.instance.api.crateApiPluginPluginManagerRemove(
        pluginManager: pluginManager, pluginName: pluginName);

Future<bool> pluginManagerDisable(
        {required PluginManager pluginManager,
        required String pluginName,
        String? reason}) =>
    RustLib.instance.api.crateApiPluginPluginManagerDisable(
        pluginManager: pluginManager, pluginName: pluginName, reason: reason);

Future<bool> pluginManagerToggle(
        {required PluginManager pluginManager, required String pluginName}) =>
    RustLib.instance.api.crateApiPluginPluginManagerToggle(
        pluginManager: pluginManager, pluginName: pluginName);
