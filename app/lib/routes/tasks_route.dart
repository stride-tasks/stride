import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/api.dart';
import 'package:stride/background.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/context.dart';
import 'package:stride/routes/initial_route.dart';
import 'package:stride/routes/repository_route.dart';
import 'package:stride/routes/task_route.dart';
import 'package:stride/utils/functions.dart';
import 'package:stride/widgets/custom_app_bar.dart';
import 'package:stride/widgets/infinite_rotation_animation.dart';
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
    return BlocBuilder<TaskBloc, TaskState>(
      builder: (context, state) => Scaffold(
        drawer: _drawer(),
        appBar: CustomAppBar(title: 'Tasks ( ${state.tasks.length} )'),
        body: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 5.0),
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

                        return _taskItem(state.tasks[index], context);
                      },
                    ),
                  ),
                ),
              ),
            ],
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
      var status = TaskStatus.done;
      if (task.status == TaskStatus.done || task.status == TaskStatus.deleted) {
        status = TaskStatus.pending;
      }
      context.read<TaskBloc>().add(
        TaskChangeStatusEvent(task: task, status: status),
      );
      return true;
    }

    Future<bool> onSwipeLeft() async {
      final additionalMessage = task.status == TaskStatus.deleted
          ? ' forever'
          : '';

      return await showAlertDialog<bool>(
            context: context,
            content: Text(
              'Are you sure you want to delete this task$additionalMessage?',
              style: const TextStyle(fontWeight: FontWeight.bold),
              textAlign: TextAlign.center,
            ),
            onConfirm: (context) async {
              context.read<TaskBloc>().add(TaskRemoveEvent(task: task));
              return true;
            },
          ) ??
          false;
    }

    return TaskItem(
      task: task,
      onSwipeRight: onSwipeRight,
      swipeRightIcon:
          (task.status == TaskStatus.done || task.status == TaskStatus.deleted)
          ? const Icon(Icons.calendar_month, color: Colors.white)
          : const Icon(Icons.check, color: Colors.white),
      swipeRightColor:
          (task.status == TaskStatus.done || task.status == TaskStatus.deleted)
          ? Colors.purpleAccent
          : Colors.greenAccent,
      swipeRightText:
          (task.status == TaskStatus.done || task.status == TaskStatus.deleted)
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

  CustomDrawer _drawer() {
    return CustomDrawer();
  }
}

class CustomDrawer extends StatefulWidget {
  const CustomDrawer({super.key});

  @override
  State<CustomDrawer> createState() => _CustomDrawerState();
}

class _CustomDrawerState extends State<CustomDrawer> {
  @override
  Widget build(BuildContext context) {
    final taskBloc = context.read<TaskBloc>();
    return Drawer(
      child: SingleChildScrollView(
        child: ValueListenableBuilder(
          valueListenable: RustContext.repositories,
          builder: (context, repositories, child) {
            return Padding(
              padding: const EdgeInsets.all(10.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      const Text(
                        'Repository',
                        style: TextStyle(
                          fontWeight: FontWeight.bold,
                          fontSize: 16,
                        ),
                      ),
                      IconButton(
                        onPressed: () {
                          Navigator.of(context).push<void>(
                            MaterialPageRoute(
                              builder: (context) => const InitialRoute(),
                            ),
                          );
                        },
                        icon: const Icon(Icons.add),
                      ),
                    ],
                  ),
                  const Divider(),
                  _repositories(repositories, taskBloc),
                ],
              ),
            );
          },
        ),
      ),
    );
  }

  Widget _repositories(
    List<RepositoryDescriptor> repositories,
    TaskBloc taskBloc,
  ) {
    return StreamBuilder<Set<Output>>(
      stream: Background.stream(),
      builder: (context, asyncSnapshot) {
        return ListView.builder(
          shrinkWrap: true,
          itemCount: repositories.length,
          itemBuilder: (context, index) {
            final repository = repositories[index];
            final selected = repository.id == taskBloc.repositoryUuid;
            return Card(
              child: ListTile(
                title: Wrap(
                  children: [
                    Text(repository.name ?? 'unnamed'),
                    if (!(asyncSnapshot.data
                            ?.lookup(
                              Output(
                                name: Name(
                                  method: 'task.sync',
                                  unique: repository.id.toString(),
                                ),
                                inputData: {},
                                timestamp: DateTime.now(),
                              ),
                            )
                            ?.done ??
                        true))
                      IconButton(
                        onPressed: () {},
                        icon: const InfiniteRotationAnimation(
                          child: Icon(Icons.sync),
                        ),
                      ),
                  ],
                ),
                subtitle: Text(repository.id.toString()),
                subtitleTextStyle: const TextStyle(fontSize: 8),
                selected: selected,
                selectedColor: Colors.amber[900],
                onTap: selected
                    ? null
                    : () {
                        final settings = context.read<SettingsBloc>().settings;
                        context.read<SettingsBloc>().add(
                          SettingsUpdateEvent(
                            settings: settings.copyWith(
                              currentRepository: repository.id,
                            ),
                          ),
                        );
                      },
                onLongPress: () {
                  Navigator.of(context).push(
                    MaterialPageRoute<void>(
                      builder: (context) =>
                          RepositoryRoute(repositoryUuid: repository.id),
                    ),
                  );
                },
              ),
            );
          },
        );
      },
    );
  }
}
