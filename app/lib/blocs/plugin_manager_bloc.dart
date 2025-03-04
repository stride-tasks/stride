import 'dart:async';

import 'package:bloc/bloc.dart';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
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
  final TaskBloc taskBloc;

  late Stream<void> _stream;
  late StreamSubscription<void> _streamSubscription;

  PluginManagerBloc({
    required this.logBloc,
    required this.taskBloc,
    required PluginManagerState state,
  }) : super(state) {
    _stream = pm.createStream();
    _streamSubscription = _stream.listen(_processHostEvents);

    on<PluginManagerFetchEvent>((event, emit) async {
      final plugins = await pm.pluginManifests();
      emit(PluginManagerState(plugins: plugins));
    });
  }

  Future<void> _processHostEvents(void _) async {
    await pm.processHostEvent();

    while (true) {
      final action = await pm.processPluginEvent();
      if (action == null) {
        return;
      }

      switch (action) {
        case PluginAction_Event(:final pluginName, :final event):
          switch (event) {
            case PluginEvent_TaskCreate(:final task):
              taskBloc.add(TaskAddEvent(task: task));
            case PluginEvent_TaskModify(:final task):
              taskBloc.add(TaskUpdateEvent(current: task));
            case PluginEvent_TaskSync():
              taskBloc.add(TaskSyncEvent());
            case PluginEvent_NetworkRequest(:final ty, :final host):
              assert(
                ty == NetworkRequestType.get_,
                'expected network request to have GET method',
              );
              http.get(Uri.parse(host)).then(
                (value) async {
                  await pm.emit(
                    pluginName: pluginName,
                    event: HostEvent.networkResponse(
                      host: host,
                      content: value.bodyBytes,
                    ),
                  );
                  // print(value.body);
                },
                onError: (error) {
                  print(error);
                },
              );
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

  Future<void> emitHostEvent(HostEvent event) async {
    await pm.emitBroadcast(event: event);
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

  @override
  Future<void> close() {
    _streamSubscription.cancel();
    return super.close();
  }
}
