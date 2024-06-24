import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';

Future<DateTime?> showPickDateTime({required BuildContext context}) async {
  final firstDate = DateTime.now().subtract(const Duration(days: 365 * 100));
  final lastDate = DateTime.now().add(const Duration(days: 365 * 100));
  DateTime? date = await showDatePicker(
    context: context,
    firstDate: firstDate,
    lastDate: lastDate,
  );

  if (date == null) return null;
  if (!context.mounted) return null;

  TimeOfDay? time = await showTimePicker(
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
  Future<bool> Function(BuildContext context)? onCancel,
  required Future<bool> Function(BuildContext context) onConfirm,
}) async {
  var result = false;
  await showDialog(
    context: context,
    builder: (context) => AlertDialog(
      content: SizedBox(
        width: MediaQuery.of(context).size.width * 0.95,
        child: content,
      ),
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
