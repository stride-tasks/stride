import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/plugin_manager_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/bridge/third_party/stride_core/event.dart';
import 'package:stride/bridge/third_party/stride_core/task.dart';
import 'package:stride/utils/extensions.dart';

class TaskItem extends StatelessWidget {
  final Task task;

  final Future<bool> Function()? onSwipeRight;
  final Color swipeRightColor;
  final Icon swipeRightIcon;
  final String? swipeRightText;
  final Future<bool> Function()? onSwipeLeft;
  final Color swipeLeftColor;
  final Icon swipeLeftIcon;
  final String? swipeLeftText;
  final void Function()? onLongPress;

  const TaskItem({
    super.key,
    required this.task,
    this.onSwipeRight,
    this.swipeRightColor = Colors.greenAccent,
    this.swipeRightIcon = const Icon(Icons.check),
    this.swipeRightText,
    this.onSwipeLeft,
    this.swipeLeftColor = Colors.redAccent,
    this.swipeLeftIcon = const Icon(Icons.delete),
    this.swipeLeftText,
    this.onLongPress,
  });

  @override
  Widget build(BuildContext context) {
    Widget tags;
    if (task.tags.isEmpty) {
      tags = const Text(
        '',
        style: TextStyle(fontSize: 12),
      );
    } else {
      tags = Wrap(
        children: task.tags
            .map(
              (tag) => Chip(
                label: Text(
                  tag.toString(),
                  style: const TextStyle(fontSize: 12),
                ),
                labelPadding: EdgeInsets.zero,
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(100),
                ),
              ),
            )
            .map(
              (chip) => Padding(
                padding: const EdgeInsets.only(right: 8),
                child: chip,
              ),
            )
            .toList(),
      );
    }

    Widget? subtitle;
    if (task.tags.isNotEmpty || task.due != null) {
      subtitle = Padding(
        padding: const EdgeInsets.only(top: 2.0),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Text(
              task.due?.toUtc().toHumanString() ?? '',
              style: const TextStyle(fontSize: 12),
            ),
            tags,
          ],
        ),
      );
    }

    Widget widget = ListTile(
      title: Text(task.title),
      selected: task.active,
      onLongPress: onLongPress,
      onTap: () {
        final current = task.copyWith(active: !task.active);
        context
            .read<TaskBloc>()
            .add(TaskUpdateEvent(current: current, previous: task));
        context.read<PluginManagerBloc>().emitHostEvent(
              HostEvent.taskModify(
                current: current,
                previous: task,
              ),
            );
      },
      subtitle: subtitle,
      trailing: Text(task.urgency().toStringAsFixed(2)),
      contentPadding: const EdgeInsets.symmetric(horizontal: 10.0),
    );

    if (task.priority != null) {
      const borderWidth = 4.0;
      const borderRadius = BorderRadius.all(Radius.circular(5));
      final decoration = switch (task.priority!) {
        TaskPriority.h => const BoxDecoration(
            borderRadius: borderRadius,
            border: Border(
              left: BorderSide(
                color: Colors.red,
                width: borderWidth,
              ),
            ),
          ),
        TaskPriority.m => const BoxDecoration(
            borderRadius: borderRadius,
            border: Border(
              left: BorderSide(
                color: Color(0xAAfd8c00),
                width: borderWidth,
              ),
            ),
          ),
        TaskPriority.l => const BoxDecoration(
            borderRadius: borderRadius,
            border: Border(
              left: BorderSide(
                color: Colors.green,
                width: borderWidth,
              ),
            ),
          ),
      };

      widget = DecoratedBox(decoration: decoration, child: widget);
    }

    if (onSwipeLeft != null || onSwipeRight != null) {
      widget = Dismissible(
        key: Key('${task.uuid}'),
        direction: switch ((onSwipeLeft != null, onSwipeRight != null)) {
          (true, true) => DismissDirection.horizontal,
          (true, false) => DismissDirection.endToStart,
          (false, true) => DismissDirection.startToEnd,
          (false, false) => throw UnimplementedError(),
        },
        confirmDismiss: (direction) async {
          if (direction == DismissDirection.startToEnd) {
            return onSwipeRight!();
          } else {
            return onSwipeLeft!();
          }
        },
        background: _slideRightBackground(),
        secondaryBackground: _slideLeftBackground(),
        child: widget,
      );
    }

    return Card(
      margin: const EdgeInsets.symmetric(vertical: 2.0),
      child: widget,
    );
  }

  Widget _slideRightBackground() {
    return Container(
      color: swipeRightColor,
      alignment: Alignment.centerLeft,
      child: Align(
        child: Row(
          children: <Widget>[
            const SizedBox(width: 16),
            swipeRightIcon,
            const SizedBox(width: 5),
            Text(
              swipeRightText == null ? 'Complete' : swipeRightText!,
              style: const TextStyle(
                color: Colors.white,
                fontWeight: FontWeight.w700,
              ),
              textAlign: TextAlign.left,
            ),
          ],
        ),
      ),
    );
  }

  Widget _slideLeftBackground() {
    return Container(
      color: swipeLeftColor,
      alignment: Alignment.centerRight,
      child: Align(
        child: Row(
          mainAxisAlignment: MainAxisAlignment.end,
          children: <Widget>[
            swipeLeftIcon,
            const SizedBox(width: 5),
            Text(
              swipeLeftText == null ? 'Delete' : swipeLeftText!,
              style: const TextStyle(
                color: Colors.white,
                fontWeight: FontWeight.w700,
              ),
              textAlign: TextAlign.right,
            ),
            const SizedBox(width: 16),
          ],
        ),
      ),
    );
  }
}
