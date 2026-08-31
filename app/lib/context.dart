import 'dart:convert';

import 'package:stride/bridge/api/context.dart' as context;
import 'package:stride/notifications.dart';

class RustContext {
  static late Stream<String> _stream;

  static Stream<String> stream() => _stream;

  static Future<void> init() async {
    _stream = context.createContext().asBroadcastStream();

    _stream.listen((event) async {
      final json = jsonDecode(event) as Map<String, dynamic>;
      if (json['method'] == 'stride.notification.repository.changed') {
        final params = json['params'] as Map<String, dynamic>;
        final changes = params['changes'] as List<dynamic>;
        for (final change in changes) {
          final taskId = change['task-id'] as String;
          var title = change['title'] as String?;

          String? body;

          var isNewTask = false;

          final fields = change['fields'] as List<dynamic>;
          for (final field in fields) {
            final type = field['type'] as String;
            final current = field['current'] as String?;
            final previous = field['previous'] as String?;

            if (type == 'status' && previous == null && current == 'pending') {
              isNewTask = true;
              continue;
            }

            if (type == 'title') {
              title ??= current;

              if (previous == null) {
                continue;
              }
            }

            body ??= '';

            // ignore: use_string_buffers
            body += '$type: ${previous ?? 'none'} -> ${current ?? 'none'}\n';
          }

          await NotificationService.show(
            '${isNewTask ? "New task" : "Task change"}: ${title ?? "Task($taskId)"}',
            body,
          );
        }
      }
    });
  }

  static Future<void> execute(String method, String args) async {
    return context.execute(method: method, args: args);
  }
}
