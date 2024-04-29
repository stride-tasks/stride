import 'package:bloc/bloc.dart';
import 'package:flutter/material.dart';
import 'package:stride/src/rust/api/repository.dart';
import 'package:stride/src/rust/task.dart';
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
  TaskRemoveAllEvent();
}

final class TaskCompleteEvent extends TaskEvent {
  final UuidValue uuid;
  TaskCompleteEvent({required this.uuid});
}

final class TaskUpdateEvent extends TaskEvent {
  final Task task;
  TaskUpdateEvent({required this.task});
}

final class TaskSyncEvent extends TaskEvent {
  TaskSyncEvent();
}

final class TaskSearchEvent extends TaskEvent {
  final String text;
  TaskSearchEvent({required this.text});
}

final class TaskLoadDeletedEvent extends TaskEvent {
  TaskLoadDeletedEvent();
}

class TaskState {
  final List<Task> tasks;
  final bool syncing;
  const TaskState({required this.tasks, this.syncing = false});
}

class TaskBloc extends Bloc<TaskEvent, TaskState> {
  final TaskStorage repository;

  TaskBloc({required this.repository}) : super(const TaskState(tasks: [])) {
    on<TaskFetchEvent>((event, emit) async {
      await repository.load();
      final tasks = await repository.tasks();
      emit(TaskState(tasks: tasks));
    });

    on<TaskAddEvent>((event, emit) async {
      await repository.add(task: event.task);
      final tasks = await repository.tasks();
      emit(TaskState(tasks: tasks));
    });

    on<TaskRemoveEvent>((event, emit) async {
      await repository.delete(uuid: event.task.uuid);
      final tasks = await repository.tasks();
      emit(TaskState(tasks: tasks));
    });

    on<TaskRemoveAllEvent>((event, emit) async {
      await repository.clearContents();
      final tasks = await repository.tasks();
      emit(TaskState(tasks: tasks));
    });

    on<TaskCompleteEvent>((event, emit) async {
      await repository.complete(uuid: event.uuid);
      final tasks = await repository.tasks();
      emit(TaskState(tasks: tasks));
    });

    on<TaskUpdateEvent>((event, emit) async {
      await repository.update(task: event.task);
      final tasks = await repository.tasks();
      emit(TaskState(tasks: tasks));
    });

    on<TaskSyncEvent>((event, emit) async {
      final tasksOld = await repository.tasks();
      emit(TaskState(tasks: tasksOld, syncing: true));

      await repository.sync();
      final tasksNew = await repository.tasks();
      emit(TaskState(tasks: tasksNew));
    });

    on<TaskSearchEvent>((event, emit) async {
      final tasks = await repository.tasksByDescription(search: event.text);
      emit(TaskState(tasks: tasks));
    });

    on<TaskLoadDeletedEvent>((event, emit) async {
      await repository.loadDeleted();
      final tasks = await repository.deletedTasks();
      emit(TaskState(tasks: tasks));
    });
  }
}
