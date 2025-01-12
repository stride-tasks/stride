import 'dart:async';

import 'package:bloc/bloc.dart';
import 'package:flutter/material.dart';
import 'package:stride/blocs/dialog_bloc.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/bridge/api/error.dart';
import 'package:stride/bridge/api/filter.dart';
import 'package:stride/bridge/api/repository/git.dart';
import 'package:stride/bridge/git/known_hosts.dart';
import 'package:stride/bridge/task.dart';
import 'package:stride/routes/encryption_key_route.dart';
import 'package:uuid/uuid.dart';

@immutable
abstract class TaskEvent {}

final class TaskFetchEvent extends TaskEvent {}

final class TaskChangeRepository extends TaskEvent {
  final UuidValue uuid;
  TaskChangeRepository({required this.uuid});
}

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
  final Object? syncingError;
  const TaskState({
    required this.tasks,
    this.syncing = false,
    this.syncingError,
  });
}

class TaskBloc extends Bloc<TaskEvent, TaskState> {
  final DialogBloc dialogBloc;
  final SettingsBloc settingsBloc;
  final LogBloc logBloc;
  StreamSubscription<SettingsState>? settingsSubscription;

  UuidValue? repositoryUuid;
  TaskStorage? storage;
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

  TaskStorage? repository() {
    if (storage == null) {
      repositoryUuid ??= settingsBloc.settings.currentRepository ??
          settingsBloc.settings.repositories.firstOrNull?.uuid;
      if (repositoryUuid == null) {
        return null;
      }

      settingsBloc.add(
        SettingsUpdateEvent(
          settings: settingsBloc.settings.copyWith(
            currentRepository: repositoryUuid,
          ),
        ),
      );

      storage = TaskStorage.load(uuid: repositoryUuid!);
    } else if (repositoryUuid != settingsBloc.settings.currentRepository) {
      repositoryUuid = settingsBloc.settings.currentRepository;
      storage = TaskStorage.load(uuid: repositoryUuid!);
    }
    return storage;
  }

  TaskBloc({
    required this.settingsBloc,
    required this.logBloc,
    required this.dialogBloc,
    this.storage,
  }) : super(const TaskState(tasks: [])) {
    _initializeSettingsStream();

    on<TaskFetchEvent>((event, emit) async {
      emit(TaskState(tasks: await _tasks()));
    });

    on<TaskChangeRepository>((event, emit) async {
      if (storage != null) {
        storage!.unload();
        storage = null;
      }

      repositoryUuid = event.uuid;
      storage = TaskStorage.load(uuid: event.uuid);
      add(TaskFetchEvent());
    });

    on<TaskAddEvent>((event, emit) async {
      await repository()?.add(task: event.task);
      emit(TaskState(tasks: await _tasks()));
    });

    on<TaskRemoveEvent>((event, emit) async {
      if (event.task.status == TaskStatus.deleted) {
        await repository()?.removeByTask(task: event.task);
      } else {
        await repository()?.changeCategory(
          task: event.task,
          status: TaskStatus.deleted,
        );
      }

      emit(TaskState(tasks: await _tasks()));
    });

    on<TaskRemoveAllEvent>((event, emit) async {
      await repository()?.clear();
      emit(TaskState(tasks: await _tasks()));
    });

    on<TaskForcePushEvent>((event, emit) async {
      await repository()?.push(force: true);
      emit(TaskState(tasks: await _tasks()));
    });

    on<TaskChangeStatusEvent>((event, emit) async {
      await repository()?.changeCategory(
        task: event.task,
        status: event.status,
      );
      emit(TaskState(tasks: await _tasks()));
    });

    on<TaskUpdateEvent>((event, emit) async {
      await repository()?.update(task: event.task);
      emit(TaskState(tasks: await _tasks()));
    });

    on<TaskSyncEvent>((event, emit) async {
      final tasks = await _tasks();
      emit(TaskState(tasks: tasks, syncing: true));

      try {
        await repository()?.sync_();
      } catch (error) {
        emit(TaskState(tasks: tasks, syncingError: error));
        rethrow;
      }
      emit(TaskState(tasks: await _tasks()));
    });

    on<TaskFilterEvent>((event, emit) async {
      filter = event.filter;
      emit(TaskState(tasks: await _tasks()));
    });

    on<TaskCheckoutBranchEvent>((event, emit) async {
      await repository()?.checkout();
      emit(TaskState(tasks: await _tasks()));
    });
  }

  Future<List<Task>> _tasks() async {
    if (filter == null) {
      final tasks = await repository()?.tasksWithFilter(
        filter: await Filter.default_(),
      );
      return tasks ?? [];
    } else {
      final tasks = await repository()?.tasksWithFilter(filter: filter!);
      return tasks ?? [];
    }
  }

  @override
  void onError(Object error, StackTrace stackTrace) {
    super.onError(error, stackTrace);
    logBloc.add(
      LogErrorEvent(error: error, stackTrace: stackTrace, message: 'task'),
    );
    if (error is RustError) {
      final host = error.asUnknownHost();
      if (host != null) {
        dialogBloc.add(
          DialogAlertEvent(
            title: 'Accept Unknown Host: ${host.hostname}',
            content: 'Host Key: ${host.keyType.name} ${host.key}',
            onConfirm: (context) async {
              await logBloc.catch_(message: 'known hosts', () async {
                final knownHosts = await KnownHosts.load();
                KnownHosts.save(
                  this_: knownHosts.copyWith(
                    hosts: knownHosts.hosts.toList()..add(host),
                  ),
                );
              });
              if (context.mounted) Navigator.pop(context);
              return true;
            },
          ),
        );
      } else if (error.isKeyStoreVerification()) {
        dialogBloc.add(
          DialogAlertEvent(
            title: "Couldn't decrypt tasks using encryption key",
            content: 'Are you sure the encryption key is correct?',
            onConfirm: (context) async {
              Navigator.pop(context);
              await Navigator.of(context).push<void>(
                MaterialPageRoute(
                  builder: (context) => EncryptionKeyRoute(
                    repository: settingsBloc.settings.repositories.first,
                  ),
                ),
              );
              return true;
            },
          ),
        );
      }
    }
  }

  @override
  Future<void> close() {
    settingsSubscription?.cancel();
    return super.close();
  }
}
