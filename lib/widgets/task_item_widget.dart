import 'package:flutter/material.dart';
import 'package:stride/src/rust/task.dart';
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
  final Function()? onLongPress;

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
        "",
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
            .map((chip) => Padding(
                  padding: const EdgeInsets.only(right: 8),
                  child: chip,
                ))
            .toList(),
      );
    }

    Widget? subtitle;
    if (task.tags.isNotEmpty || task.due != null) {
      subtitle = Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          tags,
          Text(
            task.due?.toUtc().toHumanString() ?? "",
            style: const TextStyle(fontSize: 12),
          ),
        ],
      );
    }

    Widget widget = ListTile(
      title: Text(task.description),
      onLongPress: onLongPress,
      subtitle: subtitle,
    );

    if (onSwipeLeft != null || onSwipeRight != null) {
      widget = Dismissible(
        key: Key("${task.uuid}"),
        direction: switch ((onSwipeLeft != null, onSwipeRight != null)) {
          (true, true) => DismissDirection.horizontal,
          (true, false) => DismissDirection.endToStart,
          (false, true) => DismissDirection.startToEnd,
          (false, false) => throw UnimplementedError(),
        },
        confirmDismiss: (direction) async {
          if (direction == DismissDirection.startToEnd) {
            return await onSwipeRight!();
          } else {
            return await onSwipeLeft!();
          }
        },
        background: _slideRightBackground(),
        secondaryBackground: _slideLeftBackground(),
        child: widget,
      );
    }

    return widget;
  }

  Widget _slideRightBackground() {
    return Container(
      color: swipeRightColor,
      alignment: Alignment.centerLeft,
      child: Align(
        child: Row(
          mainAxisAlignment: MainAxisAlignment.start,
          children: <Widget>[
            const SizedBox(width: 16),
            swipeRightIcon,
            const SizedBox(width: 5),
            Text(
              swipeRightText == null ? "Complete" : swipeRightText!,
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
              swipeLeftText == null ? "Delete" : swipeLeftText!,
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
