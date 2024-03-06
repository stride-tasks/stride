import 'package:bloc/bloc.dart';
import 'package:flutter/material.dart';
import 'package:stride/src/rust/api/simple.dart';
import 'package:stride/src/rust/task.dart';

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

final class TaskUpdateEvent extends TaskEvent {
  final Task task;
  TaskUpdateEvent({required this.task});
}

final class TaskSearchEvent extends TaskEvent {
  final String text;
  TaskSearchEvent({required this.text});
}

class TaskState {
  final List<Task> tasks;
  const TaskState({required this.tasks});
}

class TaskBloc extends Bloc<TaskEvent, TaskState> {
  final TaskRepository repository;

  TaskBloc({required this.repository}) : super(const TaskState(tasks: [])) {
    on<TaskFetchEvent>((event, emit) async {
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

    on<TaskUpdateEvent>((event, emit) async {
      await repository.update(task: event.task);
      final tasks = await repository.tasks();
      emit(TaskState(tasks: tasks));
    });

    on<TaskSearchEvent>((event, emit) async {
      final tasks = await repository.tasksByDescription(search: event.text);
      emit(TaskState(tasks: tasks));
    });
  }
}
