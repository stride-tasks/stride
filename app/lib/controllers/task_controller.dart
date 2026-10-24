import 'dart:async';

import 'package:flutter/material.dart';
import 'package:stride/api.dart';
import 'package:stride/context.dart';
import 'package:stride/logging.dart';
import 'package:uuid/uuid.dart';

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

class TaskController {
  final ValueNotifier<TaskState> state = ValueNotifier(
    const TaskState(tasks: []),
  );

  UuidValue? repositoryUuid;
  TaskQuery? activeQuery;
  StreamSubscription<RepositoryChangedNotification>?
  _repositoryChangedSubscription;

  TaskController() {
    RustContext.repositories.addListener(_onRepositoriesChanged);
    repositoryUuid = _currentRepositoryUuid();

    _repositoryChangedSubscription =
        RustContext.listenTo<RepositoryChangedNotification>((event) async {
          if (event.repositoryId == repositoryUuid &&
              event.changes.isNotEmpty) {
            await fetch();
          }
        });
  }

  void _onRepositoriesChanged() {
    unawaited(_dispatch(_handleRepositoriesChanged));
  }

  Future<void> _handleRepositoriesChanged() async {
    final nextRepositoryUuid = _currentRepositoryUuid();
    if (repositoryUuid != nextRepositoryUuid) {
      repositoryUuid = nextRepositoryUuid;
      await fetch();
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

  Future<List<Task>> _tasks() async {
    final id = repositoryUuid ?? _currentRepositoryUuid();
    if (id == null) return const [];

    final result = await RepositoryTaskQueryMethod(
      repositoryId: id,
      query:
          activeQuery ??
          TaskQuery.filter(search: '', statuses: [TaskStatus.pending]),
    ).execute();
    return result.tasks;
  }

  Future<List<Task>> query(TaskQuery query) async {
    final id = repositoryUuid ?? _currentRepositoryUuid();
    if (id == null) return const [];
    final result = await RepositoryTaskQueryMethod(
      repositoryId: id,
      query: query,
    ).execute();
    return result.tasks;
  }

  UuidValue _requireRepositoryUuid() =>
      repositoryUuid ??
      _currentRepositoryUuid() ??
      (throw StateError('No repository is selected'));

  Future<void> fetch() => _dispatch(() async {
    _updateState(tasks: await _tasks());
  });

  Future<void> addTask(Task task) => _dispatch(() async {
    await RepositoryTaskAddMethod(
      repositoryId: _requireRepositoryUuid(),
      task: task,
    ).execute();
    _updateState(tasks: await _tasks());
  });

  Future<void> removeTask(Task task) => _dispatch(() async {
    if (task.status == TaskStatus.deleted) {
      await RepositoryTaskRemoveMethod(
        repositoryId: _requireRepositoryUuid(),
        taskId: task.id,
      ).execute();
    } else {
      await RepositoryTaskUpdateMethod(
        repositoryId: _requireRepositoryUuid(),
        task: task.copyWith(status: TaskStatus.deleted),
      ).execute();
    }
    _updateState(tasks: await _tasks());
  });

  Future<void> removeRepository(UuidValue id) => _dispatch(() async {
    await RepositoryRemoveMethod(repositoryId: id).execute();
    await RustContext.refreshRepositories();
  });

  Future<void> changeStatus(Task task, TaskStatus status) =>
      _dispatch(() async {
        await RepositoryTaskUpdateMethod(
          repositoryId: _requireRepositoryUuid(),
          task: task.copyWith(status: status),
        ).execute();
        _updateState(tasks: await _tasks());
      });

  Future<void> updateTask(Task task) => _dispatch(() async {
    await RepositoryTaskUpdateMethod(
      repositoryId: _requireRepositoryUuid(),
      task: task,
    ).execute();
    _updateState(tasks: await _tasks());
  });

  Future<void> sync() => _dispatch(() async {
    final tasks = await _tasks();
    _updateState(tasks: tasks, syncing: true);

    try {
      final id = repositoryUuid ?? _currentRepositoryUuid();
      if (id != null) {
        await RepositorySyncMethod(id: id).execute();
      }
    } catch (error) {
      _updateState(tasks: tasks, syncingError: error);
      rethrow;
    }
    _updateState(tasks: await _tasks());
  });

  Future<void> filter(TaskQuery? query) => _dispatch(() async {
    activeQuery = query;
    _updateState(tasks: await _tasks());
  });

  void _updateState({
    required List<Task> tasks,
    bool syncing = false,
    Object? syncingError,
  }) {
    state.value = TaskState(
      tasks: tasks,
      syncing: syncing,
      syncingError: syncingError,
      query: activeQuery,
    );
  }

  Future<void> _dispatch(Future<void> Function() action) async {
    try {
      await action();
    } catch (error, stackTrace) {
      Log.reportError(error, stackTrace: stackTrace, message: 'task');
    }
  }

  void dispose() {
    RustContext.repositories.removeListener(_onRepositoriesChanged);
    state.dispose();
    _repositoryChangedSubscription?.cancel();
  }
}

class TaskControllerProvider extends StatefulWidget {
  const TaskControllerProvider({super.key, required this.child});

  final Widget child;

  @override
  State<TaskControllerProvider> createState() => _TaskControllerProviderState();
}

class _TaskControllerProviderState extends State<TaskControllerProvider> {
  late final TaskController controller = TaskController();

  @override
  void dispose() {
    controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) =>
      _TaskControllerScope(controller: controller, child: widget.child);
}

class _TaskControllerScope extends InheritedWidget {
  const _TaskControllerScope({required this.controller, required super.child});

  final TaskController controller;

  @override
  bool updateShouldNotify(_TaskControllerScope oldWidget) =>
      controller != oldWidget.controller;

  static TaskController of(BuildContext context) {
    final scope = context.getInheritedWidgetOfExactType<_TaskControllerScope>();
    if (scope == null) {
      throw FlutterError(
        'TaskControllerProvider is missing from the widget tree',
      );
    }
    return scope.controller;
  }
}

extension TaskControllerContext on BuildContext {
  TaskController get taskController => _TaskControllerScope.of(this);
}
