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
        spacing: 6.0,
        runSpacing: 6.0,
        children: [
          if (task.due != null)
            _metadataChip(
              context,
              icon: Icons.event,
              label: task.due!.toUtc().toHumanString(),
            ),
          if (task.tags.isNotEmpty)
            ...task.tags.map(
              (tag) => _metadataChip(context, icon: Icons.sell, label: tag),
            ),
        ],
      );
    }

    void Function()? onTap;
    if (task.annotations.isNotEmpty) {
      try {
        final uri = Uri.parse(task.annotations.first.text);
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
      title: Text(task.title ?? '<missing title>'),
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
                const TextSpan(text: ' '),
                TextSpan(text: annotation.text),
              ],
            ),
          ),
        ),
      );
      widget = ExpansionTile(
        tilePadding: EdgeInsets.zero,
        title: widget,
        children: children.toList(),
      );
    }

    if (task.priority != null) {
      const borderWidth = 4.0;
      const borderRadius = BorderRadius.all(Radius.circular(5));
      final decoration = switch (task.priority!) {
        TaskPriority.h => BoxDecoration(
          borderRadius: borderRadius,
          border: const Border(
            left: BorderSide(color: Colors.red, width: borderWidth),
          ),
        ),
        TaskPriority.m => BoxDecoration(
          borderRadius: borderRadius,
          border: const Border(
            left: BorderSide(color: Color(0xAAfd8c00), width: borderWidth),
          ),
        ),
        TaskPriority.l => BoxDecoration(
          borderRadius: borderRadius,
          border: const Border(
            left: BorderSide(color: Colors.green, width: borderWidth),
          ),
        ),
      };

      widget = DecoratedBox(decoration: decoration, child: widget);
    }

    if (onSwipeLeft != null || onSwipeRight != null) {
      widget = Dismissible(
        key: Key('${task.id}-${task.status?.index ?? -1}'),
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

  Widget _metadataChip(
    BuildContext context, {
    required IconData icon,
    required String label,
  }) {
    final theme = Theme.of(context);
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8.0, vertical: 5.0),
      decoration: BoxDecoration(
        color: theme.colorScheme.surfaceContainerHighest,
        borderRadius: BorderRadius.circular(999),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(icon, size: 12, color: theme.colorScheme.onSurfaceVariant),
          const SizedBox(width: 4),
          Text(
            label,
            style:
                theme.textTheme.labelSmall?.copyWith(
                  color: theme.colorScheme.onSurfaceVariant,
                ) ??
                const TextStyle(fontSize: 11),
          ),
        ],
      ),
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
              swipeRightText == null ? 'Done' : swipeRightText!,
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
