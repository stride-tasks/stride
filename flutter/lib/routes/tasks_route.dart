import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/routes/task_filter_route.dart';
import 'package:stride/routes/task_route.dart';
import 'package:stride/src/rust/api/filter.dart';
import 'package:stride/src/rust/task.dart';
import 'package:stride/utils/functions.dart';
import 'package:stride/widgets/custom_app_bar.dart';
import 'package:stride/widgets/task_item_widget.dart';

class TasksRoute extends StatefulWidget {
  const TasksRoute({super.key});

  @override
  State<TasksRoute> createState() => _TasksRouteState();
}

class _TasksRouteState extends State<TasksRoute> {
  @override
  void initState() {
    context.read<TaskBloc>().add(TaskFetchEvent());
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: _drawer(),
      appBar: const CustomAppBar(title: 'Task List'),
      body: BlocBuilder<TaskBloc, TaskState>(
        builder: (context, state) => Padding(
          padding: const EdgeInsets.symmetric(vertical: 10.0, horizontal: 5.0),
          child: Column(
            children: [
              Expanded(
                child: ScrollConfiguration(
                  behavior: ScrollConfiguration.of(context).copyWith(
                    dragDevices: {
                      PointerDeviceKind.touch,
                      PointerDeviceKind.mouse,
                      PointerDeviceKind.trackpad,
                      PointerDeviceKind.stylus,
                    },
                  ),
                  child: RefreshIndicator(
                    onRefresh: _onRefresh,
                    child: ListView.builder(
                      itemCount: state.tasks.length + 1,
                      itemBuilder: (context, index) {
                        if (index == state.tasks.length) {
                          return const SizedBox(height: 50);
                        }

                        final task = state.tasks[index];
                        return Card(
                          child: _taskItem(task, context),
                        );
                      },
                    ),
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
      floatingActionButton: FloatingActionButton(
        shape: const CircleBorder(),
        onPressed: () async {
          await Navigator.of(context).push<void>(
            MaterialPageRoute(builder: (context) => const TaskRoute()),
          );
        },
        child: const Icon(Icons.add_circle, size: 50),
      ),
    );
  }

  Future<void> _onRefresh() async {
    final stream = context.read<TaskBloc>().stream.asBroadcastStream();
    context.read<TaskBloc>().add(TaskSyncEvent());
    await for (final state in stream) {
      if (!state.syncing) {
        break;
      }
    }
  }

  TaskItem _taskItem(Task task, BuildContext context) {
    Future<bool> onSwipeRight() async {
      var status = TaskStatus.complete;
      if (task.status == TaskStatus.complete ||
          task.status == TaskStatus.deleted) {
        status = TaskStatus.pending;
      }
      context
          .read<TaskBloc>()
          .add(TaskChangeStatusEvent(task: task, status: status));
      return true;
    }

    Future<bool> onSwipeLeft() async {
      final additionalMessage =
          task.status == TaskStatus.deleted ? ' forever' : '';

      return showAlertDialog(
        context: context,
        content: Text(
          'Are you sure you want to delete this task$additionalMessage?',
          style: const TextStyle(fontWeight: FontWeight.bold),
          textAlign: TextAlign.center,
        ),
        onConfirm: (context) async {
          context.read<TaskBloc>().add(TaskRemoveEvent(task: task));
          Navigator.of(context).pop();
          return true;
        },
      );
    }

    return TaskItem(
      task: task,
      onSwipeRight: onSwipeRight,
      swipeRightIcon: (task.status == TaskStatus.complete ||
              task.status == TaskStatus.deleted)
          ? const Icon(Icons.calendar_month, color: Colors.white)
          : const Icon(Icons.check, color: Colors.white),
      swipeRightColor: (task.status == TaskStatus.complete ||
              task.status == TaskStatus.deleted)
          ? Colors.purpleAccent
          : Colors.greenAccent,
      swipeRightText: (task.status == TaskStatus.complete ||
              task.status == TaskStatus.deleted)
          ? 'Pending'
          : null,
      onSwipeLeft: onSwipeLeft,
      swipeLeftIcon: task.status == TaskStatus.deleted
          ? const Icon(Icons.delete_forever, color: Colors.white)
          : const Icon(Icons.delete, color: Colors.white),
      onLongPress: () {
        Navigator.of(context).push<void>(
          MaterialPageRoute(builder: (context) => TaskRoute(task: task)),
        );
      },
    );
  }

  Drawer _drawer() {
    return Drawer(
      child: BlocBuilder<SettingsBloc, SettingsState>(
        builder: (context, state) {
          final filters = state.settings.filters;
          return Padding(
            padding: const EdgeInsets.all(10.0),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text(
                  'Filters',
                  style: TextStyle(fontWeight: FontWeight.bold, fontSize: 18),
                ),
                const Divider(),
                Expanded(
                  child: ListView.builder(
                    shrinkWrap: true,
                    itemCount: filters.length,
                    itemBuilder: (context, index) {
                      final filter = filters[index];
                      final selected = state.settings.selectedFilter != null &&
                          state.settings.selectedFilter ==
                              FilterSelection.predefined(uuid: filter.uuid);
                      return Card(
                        child: ListTile(
                          title: Text(filter.name),
                          selected: selected,
                          selectedColor: Colors.amber[900],
                          onLongPress: () {
                            Navigator.of(context).push<void>(
                              MaterialPageRoute(
                                builder: (context) =>
                                    TaskFilterRoute(filter: filter),
                              ),
                            );
                          },
                          onTap: () {
                            if (selected) {
                              final newSettings = state.settings.copyWith(
                                selectedFilter: null,
                              );
                              context.read<SettingsBloc>().add(
                                    SettingsUpdateEvent(settings: newSettings),
                                  );
                              context.read<TaskBloc>().add(TaskFilterEvent());
                              return;
                            }

                            final newSettings = state.settings.copyWith(
                              selectedFilter: FilterSelection.predefined(
                                uuid: filter.uuid,
                              ),
                            );
                            context.read<SettingsBloc>().add(
                                  SettingsUpdateEvent(settings: newSettings),
                                );
                            context
                                .read<TaskBloc>()
                                .add(TaskFilterEvent(filter: filter));
                          },
                        ),
                      );
                    },
                  ),
                ),
              ],
            ),
          );
        },
      ),
    );
  }
}
