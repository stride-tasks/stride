import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/bridge/api/filter.dart';
import 'package:stride/bridge/task.dart';
import 'package:stride/utils/functions.dart';
import 'package:uuid/uuid.dart';

class TaskFilterRoute extends StatefulWidget {
  final Filter? filter;

  const TaskFilterRoute({
    super.key,
    this.filter,
  });

  @override
  State<TaskFilterRoute> createState() => _TaskFilterRouteState();
}

class _TaskFilterRouteState extends State<TaskFilterRoute> {
  TextEditingController nameController = TextEditingController(text: '');
  TextEditingController searchController = TextEditingController(text: '');
  Set<TaskStatus> status = <TaskStatus>{TaskStatus.pending};

  @override
  void initState() {
    super.initState();

    if (widget.filter != null) {
      final filter = widget.filter!;
      nameController.text = filter.name;
      searchController.text = filter.search;
      status = filter.status.toSet();
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Filters')),
      body: BlocBuilder<SettingsBloc, SettingsState>(
        builder: (context, state) {
          final isMobilePlatform = Platform.isAndroid || Platform.isIOS;
          return Padding(
            padding: const EdgeInsets.symmetric(
              vertical: 20.0,
              horizontal: 10.0,
            ),
            child: Column(
              children: [
                TextField(
                  controller: searchController,
                  autofocus: true,
                  decoration: const InputDecoration(
                    border: OutlineInputBorder(),
                    labelText: 'Search',
                  ),
                ),
                const SizedBox(height: 8.0),
                SegmentedButton<TaskStatus>(
                  segments: <ButtonSegment<TaskStatus>>[
                    ButtonSegment<TaskStatus>(
                      value: TaskStatus.pending,
                      icon: const Icon(Icons.calendar_month),
                      label: isMobilePlatform ? null : const Text('Pending'),
                      tooltip: 'Pending',
                    ),
                    ButtonSegment<TaskStatus>(
                      value: TaskStatus.complete,
                      icon: const Icon(Icons.check_box),
                      label: isMobilePlatform ? null : const Text('Completed'),
                      tooltip: 'Completed',
                    ),
                    ButtonSegment<TaskStatus>(
                      value: TaskStatus.deleted,
                      icon: const Icon(Icons.delete),
                      label: isMobilePlatform ? null : const Text('Deleted'),
                      tooltip: 'Deleted',
                    ),
                    ButtonSegment<TaskStatus>(
                      value: TaskStatus.waiting,
                      icon: const Icon(Icons.alarm),
                      label: isMobilePlatform ? null : const Text('Waiting'),
                      tooltip: 'Waiting',
                    ),
                    ButtonSegment<TaskStatus>(
                      value: TaskStatus.recurring,
                      icon: const Icon(Icons.repeat_on),
                      label: isMobilePlatform ? null : const Text('Recurring'),
                      tooltip: 'Recurring',
                    ),
                  ],
                  selected: status,
                  onSelectionChanged: (newSelection) {
                    setState(() {
                      status = newSelection;
                    });
                  },
                  multiSelectionEnabled: true,
                  showSelectedIcon: !isMobilePlatform,
                  selectedIcon: const Icon(Icons.check),
                ),
                const SizedBox(height: 10.0),
                Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: _actions(state, context),
                ),
              ],
            ),
          );
        },
      ),
    );
  }

  List<Widget> _actions(SettingsState state, BuildContext context) {
    return [
      ElevatedButton.icon(
        icon: const Icon(Icons.check),
        label: const Text('Apply'),
        onPressed: () {
          final filter = Filter(
            uuid: const Uuid().v4obj(),
            status: status,
            name: 'Temporary',
            search: searchController.text,
          );
          final newSettings = state.settings.copyWith(
            selectedFilter: FilterSelection.temporary(
              filter: filter,
            ),
          );
          context
              .read<SettingsBloc>()
              .add(SettingsUpdateEvent(settings: newSettings));
          context.read<TaskBloc>().add(TaskFilterEvent(filter: filter));

          // Pop to first route.
          Navigator.popUntil(
            context,
            (route) => route.isFirst,
          );
        },
      ),
      const SizedBox(width: 20),
      ElevatedButton.icon(
        icon: const Icon(Icons.save),
        label: const Text('Save'),
        onPressed: () async {
          await showAlertDialog(
            context: context,
            content: TextField(
              controller: nameController,
              autocorrect: false,
              autofocus: true,
              decoration: const InputDecoration(
                hintText: 'Filter name...',
              ),
            ),
            onConfirm: (context) async {
              if (nameController.text.isEmpty) {
                return false;
              }

              final filters = state.settings.filters.toList();
              if (widget.filter != null) {
                final filterIndex = filters.indexWhere(
                  (element) => element.uuid == widget.filter!.uuid,
                );
                final filter = widget.filter!.copyWith(
                  name: nameController.text,
                  search: searchController.text,
                );
                filters[filterIndex] = filter;
              } else {
                final hasSameName = state.settings.filters.every(
                  (element) => element.name == nameController.text,
                );
                final name = hasSameName
                    ? '${nameController.text} 2'
                    : nameController.text;
                final filter = Filter(
                  uuid: const Uuid().v4obj(),
                  status: status,
                  name: name,
                  search: searchController.text,
                );

                filters.add(filter);
              }

              context.read<SettingsBloc>().add(
                    SettingsUpdateEvent(
                      settings: state.settings.copyWith(
                        filters: filters,
                      ),
                    ),
                  );

              // Pop to first route.
              Navigator.popUntil(context, (route) => route.isFirst);
              return false;
            },
          );
        },
      ),
      ..._deleteAction(state, context),
    ];
  }

  List<Widget> _deleteAction(SettingsState state, BuildContext context) {
    if (widget.filter == null) {
      return [];
    }

    final filter = widget.filter!;
    return [
      const SizedBox(width: 20),
      ElevatedButton.icon(
        icon: const Icon(Icons.remove_circle),
        label: const Text('Delete'),
        style: ButtonStyle(
          iconColor: WidgetStateProperty.all(Colors.redAccent),
        ),
        onPressed: () async {
          await showAlertDialog(
            context: context,
            content: Text(
              'Are you sure you want to delete ${filter.name} filter?',
              style: const TextStyle(fontWeight: FontWeight.bold),
              textAlign: TextAlign.center,
            ),
            onConfirm: (context) {
              final filters = state.settings.filters.toList()
                ..removeWhere(
                  (element) => element.uuid == filter.uuid,
                );

              if (state.settings.selectedFilter
                  case FilterSelection_Predefined(:final uuid)) {
                if (uuid == filter.uuid) {
                  context.read<TaskBloc>().add(TaskFilterEvent());
                  context.read<SettingsBloc>().add(
                        SettingsUpdateEvent(
                          settings:
                              state.settings.copyWith(selectedFilter: null),
                        ),
                      );
                }
              }

              context.read<SettingsBloc>().add(
                    SettingsUpdateEvent(
                      settings: state.settings.copyWith(filters: filters),
                    ),
                  );

              // Pop to first route.
              Navigator.popUntil(context, (route) => route.isFirst);
              return Future.value(true);
            },
          );
        },
      ),
    ];
  }
}
