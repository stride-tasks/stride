import 'dart:async';

import 'package:bloc/bloc.dart';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/bridge/api/error.dart';
import 'package:stride/bridge/api/plugin.dart';
import 'package:stride/bridge/api/plugin_manager.dart' as pm;
import 'package:stride/bridge/third_party/stride_core/event.dart';
import 'package:stride/bridge/third_party/stride_plugin_manager/manifest.dart';
import 'package:uuid/uuid.dart';

@immutable
abstract class PluginManagerEvent {}

class PluginManagerFetchEvent extends PluginManagerEvent {}

class PluginManagerState {
  final List<PluginManifestPluginState> plugins;
  const PluginManagerState({required this.plugins});
}

@immutable
class TimerRecord {
  final Duration duration;
  final Timer timer;
  const TimerRecord({required this.duration, required this.timer});
}

class PluginManagerBloc extends Bloc<PluginManagerEvent, PluginManagerState> {
  final LogBloc logBloc;
  final TaskBloc taskBloc;

  final Map<String, TimerRecord> _timers = {};
  late Stream<void> _stream;
  late StreamSubscription<void> _streamSubscription;

  PluginManagerBloc({
    required this.logBloc,
    required this.taskBloc,
    required PluginManagerState state,
  }) : super(state) {
    _stream = pm.createStream();
    _initTimers(state.plugins);
    _streamSubscription = _stream.listen(_processHostEvents);

    on<PluginManagerFetchEvent>((event, emit) async {
      final plugins = await pm.pluginManifests();
      _initTimers(plugins);
      emit(PluginManagerState(plugins: plugins));
    });
  }

  void _initTimers(List<PluginManifestPluginState> plugins) {
    final pluginNames = <String>[];
    for (final plugin in plugins) {
      final name = pluginInstanceManifestName(manifest: plugin);
      final enabled = pluginInstanceManifestEnabled(manifest: plugin);
      final event = pluginInstanceManifestEvent(manifest: plugin);
      final timer = event.timer;

      pluginNames.add(name);

      if (!enabled || timer == null) {
        final record = _timers.remove(name);
        record?.timer.cancel();
        continue;
      }

      final interval = timer.interval;
      final duration = Duration(seconds: interval);

      // If there is already a record present and the duration does not match
      // then we need to create a new timer for it.
      final record = _timers[name];
      if (record?.duration == duration) {
        continue;
      }
      record?.timer.cancel();

      _timers[name] = TimerRecord(
        duration: duration,
        timer: Timer.periodic(
          duration,
          (timer) async => pm.emit(
            event: HostEvent.timer(interval: interval),
            pluginName: name,
          ),
        ),
      );
    }

    for (final name in _timers.keys.toList()) {
      if (pluginNames.contains(name)) {
        continue;
      }

      final record = _timers.remove(name);
      record?.timer.cancel();
    }
  }

  Future<void> _processHostEvents(void _) async {
    try {
      await pm.processHostEvent();
    } on RustError catch (error, stackTrace) {
      if (error.isOutOfFuelTrapCode()) {
        final pluginName = error.pluginName();
        if (pluginName != null) {
          disable(
            pluginName,
            reason: 'plugin exceeded computation limit',
          );

          logBloc.add(
            LogErrorEvent(
              error: error,
              stackTrace: stackTrace,
              message: 'plugin ($pluginName)',
            ),
          );

          return;
        }
      }

      logBloc.add(
        LogErrorEvent(
          error: error,
          stackTrace: stackTrace,
          message: 'plugin manager',
        ),
      );
    }

    while (true) {
      final action = await pm.processPluginEvent();
      if (action == null) {
        return;
      }

      switch (action) {
        case PluginAction_Event(:final pluginName, :final event):
          switch (event) {
            case PluginEvent_TaskCreate(:final task):
              taskBloc.add(
                TaskAddEvent(
                  task: task.copyWith(
                    // Make sure to give a new UUID that what the plugin provided.
                    uuid: UuidValue.fromString(const Uuid().v7()),
                  ),
                ),
              );
            case PluginEvent_TaskModify(:final task):
              taskBloc.add(TaskUpdateEvent(current: task));
            case PluginEvent_TaskQuery(:final query):
              final tasks = await taskBloc.query(query);
              pm.emit(
                event: HostEvent.taskQuery(tasks: tasks),
                pluginName: pluginName,
              );
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
                },
                onError: (error) {
                  // TODO: Log error/notify
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

  Future<void> disable(String pluginName, {String? reason}) async {
    await pm.disable(pluginName: pluginName, reason: reason);
    add(PluginManagerFetchEvent());
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
    for (final timer in _timers.values) {
      timer.timer.cancel();
    }
    return super.close();
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
