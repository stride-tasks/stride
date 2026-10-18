import 'dart:async';

import 'package:flutter/material.dart';

import 'package:stride/api.dart';
import 'package:stride/context.dart';
import 'package:stride/routes/logging_routes.dart';
import 'package:stride/utils/classes.dart';

final class LogNotification {
  final String message;
  final bool isError;
  final bool show;

  const LogNotification({
    required this.message,
    this.isError = false,
    this.show = true,
  });
}

final class Log {
  static final _notifications = StreamController<LogNotification>.broadcast();

  static Stream<LogNotification> get notifications => _notifications.stream;

  static Future<String> getLogs() async {
    return LogGetMethod().execute().then((result) => result.content);
  }

  static void notify(String message, {bool isError = false, bool show = true}) {
    _notifications.add(
      LogNotification(message: message, isError: isError, show: show),
    );
  }

  static void reportError(
    Object error, {
    StackTrace? stackTrace,
    String message = '',
    bool show = true,
  }) {
    if (message.isNotEmpty) {
      message += ' ';
    }

    final formattedMessage =
        '${message}error: $error\n\nDart Backtrace:\n$stackTrace';
    unawaited(Log.error(formattedMessage));
    notify(formattedMessage, isError: true, show: show);
  }

  static Future<Result<T, Object>> catch_<T>(
    Future<T> Function() f, {
    String message = '',
  }) async {
    final result = await Result.catch_<T, Object>(f);
    if (result case Err(error: (final error, final stackTrace))) {
      reportError(error, stackTrace: stackTrace, message: message);
    }
    return result.mapErr((caughtError) => caughtError.$1);
  }

  static Future<void> event({
    required LogLevel level,
    required String message,
  }) async {
    await LogEventMethod(level: level, message: message).execute();
  }

  static Future<void> debug(String message) async {
    await event(level: LogLevel.debug, message: message);
  }

  static Future<void> trace(String message) async {
    await event(level: LogLevel.trace, message: message);
  }

  static Future<void> info(String message) async {
    await event(level: LogLevel.info, message: message);
  }

  static Future<void> warn(String message) async {
    await event(level: LogLevel.warn, message: message);
  }

  static Future<void> error(String message) async {
    await event(level: LogLevel.error, message: message);
  }
}

class LogNotificationListener extends StatefulWidget {
  const LogNotificationListener({super.key, required this.child});

  final Widget child;

  @override
  State<LogNotificationListener> createState() =>
      _LogNotificationListenerState();
}

class _LogNotificationListenerState extends State<LogNotificationListener> {
  late final StreamSubscription<LogNotification> _subscription;

  @override
  void initState() {
    super.initState();
    _subscription = Log.notifications.listen(_showNotification);
  }

  void _showNotification(LogNotification notification) {
    if (!notification.show || !mounted) return;

    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(notification.message.split('\n')[0]),
        behavior: SnackBarBehavior.floating,
        duration: const Duration(seconds: 10),
        backgroundColor: notification.isError ? Colors.red[300] : null,
        action: SnackBarAction(
          label: 'Go to Logs',
          onPressed: () async {
            // TODO: Maybe don't push if already there on top.
            await Navigator.of(context).push<void>(
              MaterialPageRoute(builder: (context) => const LoggingRoute()),
            );
          },
        ),
      ),
    );
  }

  @override
  void dispose() {
    unawaited(_subscription.cancel());
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => widget.child;
}
