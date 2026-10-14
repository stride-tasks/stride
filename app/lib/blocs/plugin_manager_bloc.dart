import 'package:bloc/bloc.dart';
import 'package:flutter/material.dart';
import 'package:stride/api.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/context.dart';

@immutable
abstract class PluginManagerEvent {}

class PluginManagerFetchEvent extends PluginManagerEvent {}

class PluginManagerState {
  final List<PluginManifestInstance> plugins;
  const PluginManagerState({required this.plugins});
}

class PluginManagerBloc extends Bloc<PluginManagerEvent, PluginManagerState> {
  final LogBloc logBloc;

  PluginManagerBloc({required this.logBloc, required PluginManagerState state})
    : super(state) {
    on<PluginManagerFetchEvent>((event, emit) async {
      final plugins = (await PluginListMethod().execute()).plugins;
      emit(PluginManagerState(plugins: plugins));
    });
  }

  Future<void> disable(String pluginName, {String? reason}) async {
    await PluginDisableMethod(
      pluginName: pluginName,
      reason: reason,
    ).execute();
    add(PluginManagerFetchEvent());
  }

  Future<void> toggle(String pluginName) async {
    await PluginToggleMethod(pluginName: pluginName).execute();
    add(PluginManagerFetchEvent());
  }

  Future<void> import(String filePath) async {
    await PluginImportMethod(filePath: filePath).execute();
    add(PluginManagerFetchEvent());
  }

  Future<void> remove(String pluginName) async {
    await PluginRemoveMethod(pluginName: pluginName).execute();
    add(PluginManagerFetchEvent());
  }

  @override
  void onError(Object error, StackTrace stackTrace) {
    super.onError(error, stackTrace);
    logBloc.add(
      LogErrorEvent(
        error: error,
        stackTrace: stackTrace,
        message: 'plugin manager',
      ),
    );
  }
}
