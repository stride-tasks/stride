import 'dart:async';

import 'package:bloc/bloc.dart';
import 'package:flutter/material.dart';
import 'package:stride/api.dart';
import 'package:stride/background.dart';
import 'package:stride/context.dart';
import 'package:stride/logging.dart';
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

final class TaskRepositoriesChangedEvent extends TaskEvent {}

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
  late List<RepositoryDescriptor> previousRepositories;
  UuidValue? repositoryUuid;
  TaskQuery? activeQuery;
  bool previousSettings = RustContext.setting.value.periodicSync;

  void _initializeSettingsStream() {
    previousRepositories = RustContext.repositories.value;
    RustContext.repositories.addListener(_onRepositoriesChanged);
    repositoryUuid ??= _currentRepositoryUuid();

    _registerPeriodicTasks();

    RustContext.setting.addListener(_onPeriodicSyncChanged);
  }

  Future<void> _onPeriodicSyncChanged() async {
    final previous = previousSettings;
    final next = RustContext.setting.value.periodicSync;
    previousSettings = next;

    if (!next && previous) {
      // Sync was turned off — cancel all repository tasks.
      await _unregisterPeriodicTasks(RustContext.repositories.value);
    } else {
      // Sync is on — register/replace tasks for current repositories.
      await _registerPeriodicTasks();
    }

    final nextRepositoryUuid = _currentRepositoryUuid();
    if (repositoryUuid != nextRepositoryUuid) {
      repositoryUuid = nextRepositoryUuid;
      add(TaskFetchEvent());
    }
  }

  void _onRepositoriesChanged() {
    add(TaskRepositoriesChangedEvent());
  }

  Future<void> _handleRepositoriesChanged() async {
    final nextRepositories = RustContext.repositories.value;
    final removedRepositories = previousRepositories
        .where(
          (repository) =>
              !nextRepositories.any((next) => next.id == repository.id),
        )
        .toList();
    if (removedRepositories.isNotEmpty) {
      await _unregisterPeriodicTasks(removedRepositories);
    }
    previousRepositories = nextRepositories;

    await _registerPeriodicTasks();

    final nextRepositoryUuid = _currentRepositoryUuid();
    if (repositoryUuid != nextRepositoryUuid) {
      repositoryUuid = nextRepositoryUuid;
      add(TaskFetchEvent());
    }
  }

  Future<void> _registerPeriodicTasks() async {
    if (!RustContext.setting.value.periodicSync) return;
    for (final repository in RustContext.repositories.value.asMap().entries) {
      await Background.periodic(
        TaskSyncBackgroundTask(repositoryId: repository.value.id),
        initialDelay: const Duration(seconds: 10),
        frequency: Duration(minutes: 5, seconds: repository.key * 10),
        existingWorkPolicy: ExistingPeriodicWorkPolicy.replace,
      );
    }
  }

  Future<void> _unregisterPeriodicTasks(
    List<RepositoryDescriptor> repositories,
  ) async {
    for (final repository in repositories) {
      await Background.cancel(
        TaskSyncBackgroundTask(repositoryId: repository.id),
      );
    }
  }

  UuidValue? _currentRepositoryUuid() {
    final repositories = RustContext.repositories.value;
    final selected = RustContext.setting.value.currentRepository;
    if (selected != null &&
        repositories.any((repository) => repository.id == selected)) {
      return selected;
    }
    return repositories.firstOrNull?.id;
  }

  TaskBloc() : super(const TaskState(tasks: [])) {
    _initializeSettingsStream();

    on<TaskRepositoriesChangedEvent>((event, emit) async {
      await _handleRepositoriesChanged();
    });

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
      await RustContext.refreshRepositories();
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
        final uuid = repositoryUuid ?? _currentRepositoryUuid();
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
    final repositoryId = repositoryUuid ?? _currentRepositoryUuid();
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
    final repositoryId = repositoryUuid ?? _currentRepositoryUuid();
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
      _currentRepositoryUuid() ??
      (throw StateError('No repository is selected'));

  Future<void> undo() async {
    throw UnimplementedError();
    // add(TaskFetchEvent());
  }

  @override
  void onError(Object error, StackTrace stackTrace) {
    super.onError(error, stackTrace);
    Log.reportError(error, stackTrace: stackTrace, message: 'task');
  }

  @override
  Future<void> close() {
    RustContext.repositories.removeListener(_onRepositoriesChanged);
    RustContext.setting.removeListener(_onPeriodicSyncChanged);
    return super.close();
  }
}
