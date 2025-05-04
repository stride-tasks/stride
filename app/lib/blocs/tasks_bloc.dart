import 'dart:async';

import 'package:bloc/bloc.dart';
import 'package:flutter/material.dart';
import 'package:stride/blocs/dialog_bloc.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/bridge/api/error.dart';
import 'package:stride/bridge/api/filter.dart';
import 'package:stride/bridge/api/git.dart';
import 'package:stride/bridge/api/repository.dart';
import 'package:stride/bridge/third_party/stride_backend/git/known_hosts.dart';
import 'package:stride/bridge/third_party/stride_core/event.dart';
import 'package:stride/bridge/third_party/stride_core/task.dart';
import 'package:stride/routes/encryption_key_route.dart';
import 'package:uuid/uuid.dart';

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
  final bool all;
  TaskRemoveAllEvent({this.all = false});
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
  final Task current;
  final Task? previous;
  TaskUpdateEvent({required this.current, this.previous});
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
  Repository? database;
  Filter? filter;

  Timer? syncTimer;

  void _initializeSettingsStream() {
    if (settingsBloc.settings.periodicSync) {
      syncTimer = Timer.periodic(
        const Duration(minutes: 5),
        (timer) => add(TaskSyncEvent()),
      );
    }

    repositoryUuid ??= settingsBloc.settings.currentRepositoryUuidOrFirst();

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

      final nextRepositoryUuid = event.settings.currentRepositoryUuidOrFirst();
      if (repositoryUuid != nextRepositoryUuid) {
        database = null;
        repositoryUuid = nextRepositoryUuid;
        add(TaskFetchEvent());
      }
    });
  }

  Repository? repository() {
    if (database != null) {
      return database;
    }
    if (repositoryUuid == null) {
      return null;
    }
    return database = Repository.open(uuid: repositoryUuid!);
  }

  TaskBloc({
    required this.settingsBloc,
    required this.logBloc,
    required this.dialogBloc,
    this.database,
  }) : super(const TaskState(tasks: [])) {
    _initializeSettingsStream();

    on<TaskFetchEvent>((event, emit) async {
      emit(TaskState(tasks: await _tasks()));
    });

    on<TaskAddEvent>((event, emit) async {
      await repository()?.insertTask(task: event.task);
      emit(TaskState(tasks: await _tasks()));
    });

    on<TaskRemoveEvent>((event, emit) async {
      if (event.task.status == TaskStatus.deleted) {
        await repository()?.purgeTaskById(id: event.task.uuid);
      } else {
        await repository()?.updateTask(
          task: event.task.copyWith(status: TaskStatus.deleted),
        );
      }

      emit(TaskState(tasks: await _tasks()));
    });

    on<TaskRemoveAllEvent>((event, emit) async {
      if (event.all) {
        throw UnimplementedError();
      } else {
        // await repository()?.deleteDatabase();
        // throw UnimplementedError();
      }
      // emit(TaskState(tasks: await _tasks()));
    });

    on<TaskForcePushEvent>((event, emit) async {
      // await repository()?.push(force: true);
      throw UnimplementedError();
      // emit(TaskState(tasks: await _tasks()));
    });

    on<TaskChangeStatusEvent>((event, emit) async {
      await repository()?.updateTask(
        task: event.task.copyWith(status: event.status),
      );
      emit(TaskState(tasks: await _tasks()));
    });

    on<TaskUpdateEvent>((event, emit) async {
      await repository()?.updateTask(task: event.current);
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
      // await repository()?.checkout();
      throw UnimplementedError();
      // emit(TaskState(tasks: await _tasks()));
    });
  }

  Future<List<Task>> _tasks() async {
    final tasks = await repository()?.allTasks(
      filter: filter ?? await Filter.default_(),
    );
    return tasks ?? [];
  }

  Future<List<Task>> query(TaskQuery query) async {
    final tasks = await repository()?.taskQuery(query: query);
    return tasks ?? [];
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
            content:
                'Host Key: ${hostKeyTypeName(keyType: host.keyType)} ${host.key}',
            onConfirm: (context) async {
              await logBloc.catch_(message: 'known hosts', () async {
                final knownHosts = await KnownHosts.load();
                KnownHosts.save(
                  this_: KnownHosts(
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
