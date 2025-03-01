import 'package:bloc/bloc.dart';
import 'package:flutter/material.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/bridge/api/plugin_manager.dart' as pm;
import 'package:stride/bridge/third_party/stride_core/event.dart';
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

  PluginManagerBloc({
    required this.logBloc,
    required PluginManagerState state,
  }) : super(state) {
    on<PluginManagerFetchEvent>((event, emit) async {
      final plugins = await pm.pluginManifests();
      emit(PluginManagerState(plugins: plugins));
    });
  }

  Future<void> emitHostEvent(HostEvent event, TaskBloc bloc) async {
    await pm.emit(event: event);

    await pm.processHostEvent();

    while (true) {
      final action = await pm.processPluginEvent();
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
          pm.disable(pluginName: pluginName, reason: reason);
          add(PluginManagerFetchEvent());
      }
    }
  }

  Future<void> toggle(String pluginName) async {
    await pm.toggle(pluginName: pluginName);
    add(PluginManagerFetchEvent());
  }

  Future<void> import(String filepath) async {
    await pm.import_(filepath: filepath);
    add(PluginManagerFetchEvent());
  }

  Future<void> remove(String pluginName) async {
    await pm.remove(pluginName: pluginName);
    add(PluginManagerFetchEvent());
  }
}
