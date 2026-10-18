import 'dart:io';

import 'package:flutter/material.dart';
import 'package:stride/api.dart';
import 'package:stride/controllers/task_controller.dart';

class TaskFilterRoute extends StatefulWidget {
  const TaskFilterRoute({super.key});

  @override
  State<TaskFilterRoute> createState() => _TaskFilterRouteState();
}

class _TaskFilterRouteState extends State<TaskFilterRoute> {
  final TextEditingController searchController = TextEditingController();
  Set<TaskStatus> status = <TaskStatus>{TaskStatus.pending};

  @override
  void initState() {
    super.initState();

    final query = context.taskController.activeQuery;
    if (query case TaskQuery_Filter(:final search, :final statuses)) {
      searchController.text = search;
      status = statuses.toSet();
    }
  }

  @override
  void dispose() {
    searchController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final isMobilePlatform = Platform.isAndroid || Platform.isIOS;
    return Scaffold(
      appBar: AppBar(title: const Text('Filters')),
      body: Padding(
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
                      value: TaskStatus.done,
                      icon: const Icon(Icons.check_box),
                      label: isMobilePlatform ? null : const Text('Done'),
                      tooltip: 'Done',
                    ),
                    ButtonSegment<TaskStatus>(
                      value: TaskStatus.deleted,
                      icon: const Icon(Icons.delete),
                      label: isMobilePlatform ? null : const Text('Deleted'),
                      tooltip: 'Deleted',
                    ),
                    // TODO: Uncomment when waiting and recurring tasks are implemented.
                    // ButtonSegment<TaskStatus>(
                    //   value: TaskStatus.waiting,
                    //   icon: const Icon(Icons.alarm),
                    //   label: isMobilePlatform ? null : const Text('Waiting'),
                    //   tooltip: 'Waiting',
                    // ),
                    // ButtonSegment<TaskStatus>(
                    //   value: TaskStatus.recurring,
                    //   icon: const Icon(Icons.repeat_on),
                    //   label: isMobilePlatform ? null : const Text('Recurring'),
                    //   tooltip: 'Recurring',
                    // ),
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
                  children: _actions(context),
                ),
              ],
            ),
      ),
    );
  }

  List<Widget> _actions(BuildContext context) {
    return [
      ElevatedButton.icon(
        icon: const Icon(Icons.check),
        label: const Text('Apply'),
        onPressed: () {
          context.taskController.filter(
            TaskQuery.filter(
              search: searchController.text,
              statuses: status.toList(),
            ),
          );
          Navigator.popUntil(context, (route) => route.isFirst);
        },
      ),
    ];
  }
}
