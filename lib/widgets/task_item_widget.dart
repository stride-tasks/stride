import 'package:flutter/material.dart';
import 'package:stride/src/rust/task.dart';
import 'package:stride/utils/extensions.dart';

class TaskItem extends StatelessWidget {
  final Task task;

  final Future<bool> Function()? onSwipeRight;
  final Color swipeRightColor;
  final Future<bool> Function()? onSwipeLeft;
  final Color swipeLeftColor;
  final Function()? onLongPress;

  const TaskItem({
    super.key,
    required this.task,
    this.onSwipeRight,
    this.swipeRightColor = Colors.greenAccent,
    this.onSwipeLeft,
    this.swipeLeftColor = Colors.redAccent,
    this.onLongPress,
  });

  @override
  Widget build(BuildContext context) {
    Widget tags;
    if (task.tags.isEmpty) {
      tags = const Text("");
    } else {
      tags = Wrap(
        children: task.tags
            .map(
              (tag) => Chip(
                label: Text(tag),
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

    Widget widget = ListTile(
      title: Text(task.description),
      onLongPress: onLongPress,
      subtitle: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          tags,
          Text(task.due?.toUtc().toHumanString() ?? ""),
        ],
      ),
    );

    if (onSwipeLeft != null && onSwipeRight != null) {
      widget = Dismissible(
        key: Key("${task.uuid}left"),
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
      child: const Align(
        child: Row(
          mainAxisAlignment: MainAxisAlignment.start,
          children: <Widget>[
            SizedBox(
              width: 20,
            ),
            Icon(Icons.check, color: Colors.white),
            Text(
              " Complete",
              style: TextStyle(
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
      child: const Align(
        child: Row(
          mainAxisAlignment: MainAxisAlignment.end,
          children: <Widget>[
            Icon(Icons.delete, color: Colors.white),
            Text(
              " Delete",
              style: TextStyle(
                color: Colors.white,
                fontWeight: FontWeight.w700,
              ),
              textAlign: TextAlign.right,
            ),
            SizedBox(
              width: 20,
            ),
          ],
        ),
      ),
    );
  }
}
