import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/routes/routes.dart';
import 'package:stride/src/rust/api/filter.dart';
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
      appBar: const CustomAppBar(title: "Task List"),
      body: BlocBuilder<TaskBloc, TaskState>(
        builder: (context, state) => Padding(
          padding: const EdgeInsets.symmetric(vertical: 10.0, horizontal: 5.0),
          child: Column(
            children: [
              Expanded(
                child: ListView.builder(
                  itemCount: state.tasks.length + 1,
                  itemBuilder: (context, index) {
                    if (index == state.tasks.length) {
                      return const SizedBox(height: 50);
                    }

                    return Card(
                      child: TaskItemWidget(task: state.tasks[index]),
                    );
                  },
                ),
              ),
            ],
          ),
        ),
      ),
      floatingActionButton: FloatingActionButton(
        shape: const CircleBorder(),
        onPressed: () async {
          await Navigator.of(context).pushNamed(Routes.taskAdd);
        },
        child: const Icon(Icons.add_circle, size: 50),
      ),
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
                  "Filters",
                  style: TextStyle(
                    fontWeight: FontWeight.bold,
                    fontSize: 20,
                  ),
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
                            Navigator.of(context).pushNamed(
                              Routes.taskFilter,
                              arguments: filter,
                            );
                          },
                          onTap: () {
                            if (selected) {
                              var newSettings = state.settings.copyWith(
                                selectedFilter: null,
                              );
                              context.read<SettingsBloc>().add(
                                  SettingsUpdateEvent(settings: newSettings));
                              context
                                  .read<TaskBloc>()
                                  .add(TaskFilterEvent(filter: null));
                              return;
                            }

                            var newSettings = state.settings.copyWith(
                              selectedFilter: FilterSelection.predefined(
                                uuid: filter.uuid,
                              ),
                            );
                            context.read<SettingsBloc>().add(
                                SettingsUpdateEvent(settings: newSettings));
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
