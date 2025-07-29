import 'dart:async';

import 'package:flutter/material.dart';

Future<DateTime?> showPickDateTime({required BuildContext context}) async {
  final now = DateTime.now();
  final firstDate = now.subtract(const Duration(days: 365 * 100));
  final lastDate = now.add(const Duration(days: 365 * 100));
  final date = await showDatePicker(
    context: context,
    firstDate: firstDate,
    lastDate: lastDate,
  );

  if (date == null) return null;
  if (!context.mounted) return null;

  final time = await showTimePicker(
    context: context,
    initialTime: TimeOfDay.fromDateTime(now),
  );
  if (time == null) return null;

  // NOTE: Dates are stored in UTC.
  return DateTime(
    date.year,
    date.month,
    date.day,
    time.hour,
    time.minute,
  ).toUtc();
}

Future<T?> showAlertDialog<T>({
  required BuildContext context,
  required Widget content,
  FutureOr<T> Function(BuildContext context)? onCancel,
  required FutureOr<T> Function(BuildContext context) onConfirm,
  bool popRoute = true,
}) async {
  return showDialog<T?>(
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
            final result = await onCancel(context);

            if (context.mounted && popRoute) Navigator.pop(context, result);
          },
        ),
        IconButton(
          icon: const Icon(Icons.check),
          onPressed: () async {
            final result = await onConfirm(context);
            if (context.mounted && popRoute) Navigator.pop(context, result);
          },
        ),
      ],
    ),
  );
}
