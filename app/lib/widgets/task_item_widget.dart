import 'package:flutter/material.dart';
import 'package:stride/bridge/third_party/stride_core/task.dart';
import 'package:stride/utils/extensions.dart';
import 'package:url_launcher/url_launcher.dart';

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
    Widget? subtitle;
    if (task.due != null || task.tags.isNotEmpty) {
      subtitle = Wrap(
        crossAxisAlignment: WrapCrossAlignment.center,
        children: [
          if (task.due != null)
            Padding(
              padding: const EdgeInsets.only(top: 2.0),
              child: Text(
                task.due?.toUtc().toHumanString() ?? '',
                style: const TextStyle(fontSize: 12),
              ),
            ),
          if (task.tags.isNotEmpty) SizedBox(width: 8),
          if (task.tags.isNotEmpty)
            ...task.tags.map(
              (tag) => Padding(
                padding: const EdgeInsets.only(right: 4),
                child: InputChip(
                  label: Text(
                    tag,
                    style: const TextStyle(fontSize: 10),
                  ),
                  labelPadding: EdgeInsets.zero,
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(10),
                  ),
                ),
              ),
            ),
        ],
      );
    }

    void Function()? onTap;
    if (task.annotations.isNotEmpty) {
      final description = task.annotations.first.description;
      try {
        final uri = Uri.parse(description);
        // TODO: Maybe allow other link types, example: email?
        if (uri.isScheme('HTTP') || uri.isScheme('HTTPS')) {
          onTap = () async {
            launchUrl(uri);
          };
        }
        // ignore: avoid_catches_without_on_clauses, empty_catches
      } catch (_) {}
    }

    Widget widget = ListTile(
      title: Text(task.title),
      selected: task.active,
      onLongPress: onLongPress,
      subtitle: subtitle,
      onTap: onTap,
      trailing: Text(task.urgency().toStringAsFixed(2)),
      contentPadding: const EdgeInsets.symmetric(horizontal: 10.0),
      leading: onTap == null ? null : const Icon(Icons.open_in_new),
    );

    if (task.annotations.isNotEmpty) {
      final children = task.annotations.map(
        (annotation) => ListTile(
          title: RichText(
            text: TextSpan(
              children: [
                TextSpan(
                  text: annotation.entry.toHumanString(),
                  style: const TextStyle(fontWeight: FontWeight.bold),
                ),
                TextSpan(text: ' '),
                TextSpan(text: annotation.description),
              ],
            ),
          ),
        ),
      );
      widget = ExpansionTile(
        tilePadding: EdgeInsets.all(0),
        title: widget,
        children: children.toList(),
      );
    }

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
        key: Key('${task.uuid}-${task.status.index}'),
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
