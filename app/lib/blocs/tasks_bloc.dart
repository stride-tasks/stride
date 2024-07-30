import 'dart:async';

import 'package:bloc/bloc.dart';
import 'package:flutter/material.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/bridge/api/error.dart';
import 'package:stride/bridge/api/filter.dart';
import 'package:stride/bridge/api/logging.dart';
import 'package:stride/bridge/api/repository.dart';
import 'package:stride/bridge/task.dart';

@immutable
abstract class TaskEvent {}

final class TaskFetchEvent extends TaskEvent {}

final class TaskAddEvent extends TaskEvent {
  final Task task;
  TaskAddEvent({required this.task});
}

final class TaskRemoveEvent extends TaskEvent {
  final Task task;
  TaskRemoveEvent({required this.task});
}

final class TaskRemoveAllEvent extends TaskEvent {
  TaskRemoveAllEvent();
}

final class TaskForcePushEvent extends TaskEvent {
  TaskForcePushEvent();
}

final class TaskChangeStatusEvent extends TaskEvent {
  final Task task;
  final TaskStatus status;
  TaskChangeStatusEvent({required this.task, required this.status});
}

final class TaskUpdateEvent extends TaskEvent {
  final Task task;
  TaskUpdateEvent({required this.task});
}

final class TaskSyncEvent extends TaskEvent {
  TaskSyncEvent();
}

final class TaskFilterEvent extends TaskEvent {
  final Filter? filter;
  TaskFilterEvent({this.filter});
}

final class TaskCheckoutBranchEvent extends TaskEvent {
  TaskCheckoutBranchEvent();
}

class TaskState {
  final List<Task> tasks;
  final bool syncing;
  final RustError? error;
  const TaskState({required this.tasks, this.syncing = false, this.error});
}

class TaskBloc extends Bloc<TaskEvent, TaskState> {
  final SettingsBloc settingsBloc;
  StreamSubscription<SettingsState>? settingsSubscription;

  final TaskStorage repository;
  Filter? filter;

  Timer? syncTimer;

  void _initializeSettingsStream() {
    if (settingsBloc.settings.periodicSync) {
      syncTimer = Timer.periodic(
        const Duration(minutes: 5),
        (timer) => add(TaskSyncEvent()),
      );
    }

    settingsSubscription = settingsBloc.stream.listen((event) {
      if (event.settings.periodicSync) {
        syncTimer ??= Timer.periodic(
          const Duration(minutes: 5),
          (timer) => add(TaskSyncEvent()),
        );
      } else {
        syncTimer?.cancel();
        syncTimer = null;
      }
    });
  }

  TaskBloc({
    required this.settingsBloc,
    required this.repository,
  }) : super(const TaskState(tasks: [])) {
    _initializeSettingsStream();

    on<TaskFetchEvent>((event, emit) async {
      final tasks = await repository.tasks();
      emit(TaskState(tasks: tasks));
    });

    on<TaskAddEvent>((event, emit) async {
      await repository.add(task: event.task);
      emit(TaskState(tasks: await _tasks()));
    });

    on<TaskRemoveEvent>((event, emit) async {
      if (event.task.status == TaskStatus.deleted) {
        await repository.removeTask(task: event.task);
      } else {
        await repository.changeCategory(
          task: event.task,
          status: TaskStatus.deleted,
        );
      }

      emit(TaskState(tasks: await _tasks()));
    });

    on<TaskRemoveAllEvent>((event, emit) async {
      await repository.clear();
      emit(TaskState(tasks: await _tasks()));
    });

    on<TaskForcePushEvent>((event, emit) async {
      try {
        await repository.push(force: true);
      } on RustError catch (error) {
        Logger.error(message: error.toErrorString());
        return;
      }
      emit(TaskState(tasks: await _tasks()));
    });

    on<TaskChangeStatusEvent>((event, emit) async {
      await repository.changeCategory(
        task: event.task,
        status: event.status,
      );
      emit(TaskState(tasks: await _tasks()));
    });

    on<TaskUpdateEvent>((event, emit) async {
      await repository.update(task: event.task);
      emit(TaskState(tasks: await _tasks()));
    });

    on<TaskSyncEvent>((event, emit) async {
      final tasksOld = await _tasks();
      emit(TaskState(tasks: tasksOld, syncing: true));

      try {
        await repository.sync_();
      } on RustError catch (error) {
        Logger.error(message: error.toErrorString());
        emit(TaskState(tasks: tasksOld, error: error));
        return;
      }

      emit(TaskState(tasks: await _tasks()));
    });

    on<TaskFilterEvent>((event, emit) async {
      filter = event.filter;
      emit(TaskState(tasks: await _tasks()));
    });

    on<TaskCheckoutBranchEvent>((event, emit) async {
      await repository.checkout();
      emit(TaskState(tasks: await _tasks()));
    });
  }

  Future<List<Task>> _tasks() async {
    // TODO: Handle possible thrown error.
    // try {
    if (filter == null) {
      final tasks = await repository.tasks();
      return tasks;
    } else {
      final tasks = await repository.tasksWithFilter(filter: filter!);
      return tasks;
    }
    // } on RustError catch (error) {
    //   Logger.error(message: 'repository load error: ${error.toErrorString()}');
    // } on Exception catch (error) {
    //   Logger.error(message: 'repository load error: $error');
    // }
    // return [];
  }

  @override
  Future<void> close() {
    settingsSubscription?.cancel();
    return super.close();
  }
}
