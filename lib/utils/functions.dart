import 'package:flutter/material.dart';

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
