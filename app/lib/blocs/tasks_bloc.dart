import 'dart:async';

import 'package:bloc/bloc.dart';
import 'package:flutter/material.dart';
import 'package:stride/api.dart';
import 'package:stride/background.dart';
import 'package:stride/blocs/dialog_bloc.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/bridge/api/settings.dart';
import 'package:stride/context.dart';
import 'package:uuid/uuid.dart';
import 'package:workmanager/workmanager.dart' hide TaskStatus;

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

final class TaskRemoveRepositoryEvent extends TaskEvent {
  final UuidValue uuid;
  TaskRemoveRepositoryEvent({required this.uuid});
}

final class TaskChangeStatusEvent extends TaskEvent {
  final Task task;
  final TaskStatus status;
  TaskChangeStatusEvent({required this.task, required this.status});
}

final class TaskUpdateEvent extends TaskEvent {
  final Task current;
  final Task? previous;
  TaskUpdateEvent({required this.current, this.previous});
}

final class TaskSyncEvent extends TaskEvent {
  TaskSyncEvent();
}

final class TaskFilterEvent extends TaskEvent {
  final TaskQuery? query;
  TaskFilterEvent({this.query});
}

class TaskState {
  final List<Task> tasks;
  final bool syncing;
  final Object? syncingError;
  final TaskQuery? query;
  const TaskState({
    required this.tasks,
    this.syncing = false,
    this.syncingError,
    this.query,
  });
}

class TaskBloc extends Bloc<TaskEvent, TaskState> {
  final DialogBloc dialogBloc;
  final SettingsBloc settingsBloc;
  final LogBloc logBloc;
  StreamSubscription<SettingsState>? settingsSubscription;
  UuidValue? repositoryUuid;
  TaskQuery? activeQuery;

  void _initializeSettingsStream() {
    repositoryUuid ??= settingsBloc.settings.currentRepositoryUuidOrFirst();

    _registerPeriodicTasks(settingsBloc.settings);

    var previousSettings = settingsBloc.settings;

    settingsSubscription = settingsBloc.stream.listen((event) async {
      final previous = previousSettings;
      final next = event.settings;
      previousSettings = next;

      if (!next.periodicSync && previous.periodicSync) {
        // Sync was turned off — cancel all repository tasks.
        await _unregisterPeriodicTasks(previous.repositories);
      } else {
        // Sync is on — register/replace tasks for current repositories.
        await _registerPeriodicTasks(next);

        // Cancel tasks for repositories that were removed.
        final removedRepositories = previous.repositories
            .where((r) => !next.repositories.any((n) => n.uuid == r.uuid))
            .toList();
        if (removedRepositories.isNotEmpty) {
          await _unregisterPeriodicTasks(removedRepositories);
        }
      }

      final nextRepositoryUuid = event.settings.currentRepositoryUuidOrFirst();
      if (repositoryUuid != nextRepositoryUuid) {
        repositoryUuid = nextRepositoryUuid;
        add(TaskFetchEvent());
      }
    });
  }

  Future<void> _registerPeriodicTasks(Settings settings) async {
    if (!settings.periodicSync) return;
    for (final repository in settings.repositories.asMap().entries) {
      await Background.periodic(
        TaskSyncBackgroundTask(repositoryId: repository.value.uuid),
        initialDelay: const Duration(seconds: 10),
        frequency: Duration(minutes: 5, seconds: repository.key * 10),
        existingWorkPolicy: ExistingPeriodicWorkPolicy.replace,
      );
    }
  }

  Future<void> _unregisterPeriodicTasks(
    List<RepositorySpecification> repositories,
  ) async {
    for (final repository in repositories) {
      await Background.cancel(
        TaskSyncBackgroundTask(repositoryId: repository.uuid),
      );
    }
  }

  TaskBloc({
    required this.settingsBloc,
    required this.logBloc,
    required this.dialogBloc,
  }) : super(const TaskState(tasks: [])) {
    _initializeSettingsStream();

    on<TaskFetchEvent>((event, emit) async {
      emit(TaskState(tasks: await _tasks(), query: activeQuery));
    });

    on<TaskAddEvent>((event, emit) async {
      await RepositoryTaskAddMethod(
        repositoryId: _requireRepositoryUuid(),
        task: event.task,
      ).execute();
      emit(TaskState(tasks: await _tasks(), query: activeQuery));
    });

    on<TaskRemoveEvent>((event, emit) async {
      if (event.task.status == TaskStatus.deleted) {
        await RepositoryTaskRemoveMethod(
          repositoryId: _requireRepositoryUuid(),
          taskId: event.task.id,
        ).execute();
      } else {
        await RepositoryTaskUpdateMethod(
          repositoryId: _requireRepositoryUuid(),
          task: event.task.copyWith(status: TaskStatus.deleted),
        ).execute();
      }

      emit(TaskState(tasks: await _tasks(), query: activeQuery));
    });

    on<TaskRemoveRepositoryEvent>((event, emit) async {
      await RepositoryRemoveMethod(repositoryId: event.uuid).execute();
    });

    on<TaskChangeStatusEvent>((event, emit) async {
      await RepositoryTaskUpdateMethod(
        repositoryId: _requireRepositoryUuid(),
        task: event.task.copyWith(status: event.status),
      ).execute();
      emit(TaskState(tasks: await _tasks(), query: activeQuery));
    });

    on<TaskUpdateEvent>((event, emit) async {
      await RepositoryTaskUpdateMethod(
        repositoryId: _requireRepositoryUuid(),
        task: event.current,
      ).execute();
      emit(TaskState(tasks: await _tasks(), query: activeQuery));
    });

    on<TaskSyncEvent>((event, emit) async {
      final tasks = await _tasks();
      emit(TaskState(tasks: tasks, syncing: true, query: activeQuery));

      try {
        final uuid =
            repositoryUuid ??
            settingsBloc.settings.currentRepositoryUuidOrFirst();
        if (uuid != null) {
          await Background.run(
            TaskSyncBackgroundTask(repositoryId: uuid),
            existingWorkPolicy: ExistingWorkPolicy.replace,
          );
        }
      } catch (error) {
        emit(TaskState(tasks: tasks, syncingError: error, query: activeQuery));
        rethrow;
      }
      emit(TaskState(tasks: await _tasks(), query: activeQuery));
    });

    on<TaskFilterEvent>((event, emit) async {
      activeQuery = event.query;
      emit(TaskState(tasks: await _tasks(), query: activeQuery));
    });
  }

  Future<List<Task>> _tasks() async {
    final repositoryId =
        repositoryUuid ?? settingsBloc.settings.currentRepositoryUuidOrFirst();
    if (repositoryId == null) {
      return const [];
    }

    final result = await RepositoryTaskQueryMethod(
      repositoryId: repositoryId,
      query:
          activeQuery ??
          TaskQuery.filter(search: '', statuses: [TaskStatus.pending]),
    ).execute();
    return result.tasks;
  }

  Future<List<Task>> query(TaskQuery query) async {
    final repositoryId =
        repositoryUuid ?? settingsBloc.settings.currentRepositoryUuidOrFirst();
    if (repositoryId == null) {
      return const [];
    }
    final result = await RepositoryTaskQueryMethod(
      repositoryId: repositoryId,
      query: query,
    ).execute();
    return result.tasks;
  }

  UuidValue _requireRepositoryUuid() =>
      repositoryUuid ??
      settingsBloc.settings.currentRepositoryUuidOrFirst() ??
      (throw StateError('No repository is selected'));

  Future<void> undo() async {
    throw UnimplementedError();
    // add(TaskFetchEvent());
  }

  @override
  void onError(Object error, StackTrace stackTrace) {
    super.onError(error, stackTrace);
    logBloc.add(
      LogErrorEvent(error: error, stackTrace: stackTrace, message: 'task'),
    );
  }

  @override
  Future<void> close() {
    settingsSubscription?.cancel();
    return super.close();
  }
}
