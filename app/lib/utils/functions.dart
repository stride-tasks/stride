import 'package:flutter/material.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:stride/bridge/api/error.dart';
import 'package:stride/bridge/api/logging.dart';

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
  Future<bool> Function(BuildContext context)? onCancel,
  required Future<bool> Function(BuildContext context) onConfirm,
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

void logException<T>(T error, StackTrace? stackTrace) {
  switch (error) {
    case final RustError error:
      Logger.error(
        message:
            'tasks error: ${error.toErrorString()}\n\nDart Backtrace:\n$stackTrace',
      );
    case final AnyhowException error:
      Logger.error(
        message: 'task error: ${error.message}\n\nDart Backtrace:\n$stackTrace',
      );
    case final Exception error:
      Logger.error(
        message: 'task error: $error\n\nDart Backtrace:\n$stackTrace',
      );
    default:
      Logger.error(
        message: 'task error: $error\n\nDart Backtrace:\n$stackTrace',
      );
  }
}

T? catchLogException<T>(T Function() f) {
  try {
    return f();
    // ignore: avoid_catches_without_on_clauses
  } catch (error, stackTrace) {
    logException(error, stackTrace);
  }
  return null;
}
