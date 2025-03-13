import 'dart:async';

import 'package:flutter/material.dart';

Future<DateTime?> showPickDateTime({required BuildContext context}) async {
  final firstDate = DateTime.now().subtract(const Duration(days: 365 * 100));
  final lastDate = DateTime.now().add(const Duration(days: 365 * 100));
  final date = await showDatePicker(
    context: context,
    firstDate: firstDate,
    lastDate: lastDate,
  );

  if (date == null) return null;
  if (!context.mounted) return null;

  final time = await showTimePicker(
    context: context,
    initialTime: TimeOfDay.now(),
  );
  if (time == null) return null;

  return DateTime(
    date.year,
    date.month,
    date.day,
    time.hour,
    time.minute,
  );
}

Future<bool> showAlertDialog({
  required BuildContext context,
  required Widget content,
  FutureOr<bool> Function(BuildContext context)? onCancel,
  required FutureOr<bool> Function(BuildContext context) onConfirm,
}) async {
  var result = false;
  await showDialog<void>(
    context: context,
    builder: (context) => AlertDialog(
      content: SizedBox(
        width: MediaQuery.of(context).size.width * 0.95,
        child: content,
      ),
      actionsAlignment: MainAxisAlignment.center,
      actions: [
        IconButton(
          icon: const Icon(Icons.cancel),
          onPressed: () async {
            if (onCancel == null) {
              Navigator.pop(context);
              return;
            }
            await onCancel(context);
          },
        ),
        IconButton(
          icon: const Icon(Icons.check),
          onPressed: () async {
            await onConfirm(context);
            result = true;
          },
        ),
      ],
    ),
  );
  return result;
}
