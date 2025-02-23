import 'package:bloc/bloc.dart';
import 'package:flutter/material.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/bridge/api/plugin.dart';
import 'package:stride/bridge/third_party/stride_core/event.dart';
import 'package:stride/bridge/third_party/stride_plugin_manager/manager.dart';
import 'package:stride/bridge/third_party/stride_plugin_manager/manifest.dart';

@immutable
abstract class PluginManagerEvent {}

class PluginManagerFetchEvent extends PluginManagerEvent {}

class PluginManagerState {
  final List<PluginManifestPluginState> plugins;
  const PluginManagerState({required this.plugins});
}

class PluginManagerBloc extends Bloc<PluginManagerEvent, PluginManagerState> {
  final LogBloc logBloc;
  final PluginManager pluginManager;

  PluginManagerBloc({
    required this.logBloc,
    required this.pluginManager,
    required PluginManagerState state,
  }) : super(state) {
    on<PluginManagerFetchEvent>((event, emit) async {
      final plugins = await pluginManifests(pluginManager: pluginManager);
      emit(PluginManagerState(plugins: plugins));
    });
  }

  Future<void> emitHostEvent(HostEvent event, TaskBloc bloc) async {
    await pluginManagerEmit(pluginManager: pluginManager, event: event);

    while (true) {
      final action = await pluginManager.processEvent();
      if (action == null) {
        return;
      }

      switch (action) {
        case PluginAction_Event(:final event):
          switch (event) {
            case PluginEvent_TaskCreate(:final task):
              bloc.add(TaskAddEvent(task: task, fromHost: false));
            case PluginEvent_TaskModify(:final task):
              bloc.add(TaskUpdateEvent(current: task, fromHost: false));
            case PluginEvent_TaskSync():
              bloc.add(TaskSyncEvent(fromHost: false));
          }
        case PluginAction_Disable(:final pluginName, :final reason):
          logBloc.add(
            LogMessageEvent(
              message: 'Disabling plugin $pluginName: $reason',
              isError: true,
            ),
          );
          pluginManagerDisable(
            pluginManager: pluginManager,
            pluginName: pluginName,
            reason: reason,
          );
          add(PluginManagerFetchEvent());
      }
    }
  }

  Future<void> toggle(String pluginName) async {
    await pluginManagerToggle(
      pluginManager: pluginManager,
      pluginName: pluginName,
    );
    add(PluginManagerFetchEvent());
  }

  Future<void> import(String filepath) async {
    await pluginManagerImport(pluginManager: pluginManager, filepath: filepath);
    add(PluginManagerFetchEvent());
  }

  Future<void> remove(String pluginName) async {
    await pluginManagerRemove(
      pluginManager: pluginManager,
      pluginName: pluginName,
    );
    add(PluginManagerFetchEvent());
  }
}
