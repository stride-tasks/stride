import 'dart:convert';

import 'package:stride/api.dart';
import 'package:stride/bridge/api/context.dart' as context;
import 'package:stride/notifications.dart';

class RustContext {
  static late Stream<String> _stream;

  static Stream<String> stream() => _stream;

  static Future<void> init() async {
    _stream = context.createContext().asBroadcastStream();

    _stream.listen((event) async {
      final json = jsonDecode(event) as Map<String, dynamic>;
      final method = json['method'] as String;
      final params = json['params'] as Map<String, dynamic>;
      if (method == 'stride.repository.changed') {
        final notification = RepositoryChangedNotification.fromJson(params);
        for (final change in notification.changes) {
          final taskId = change.taskId;
          var title = change.title;

          String? body;

          var isNewTask = false;

          for (final field in change.fields) {
            final type = field.type;
            final current = field.current;
            final previous = field.previous;

            if (type == 'status' && previous == null && current == 'pending') {
              isNewTask = true;
              continue;
            }

            if (type == 'title' && current != null) {
              title ??= current!;

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

  static Future<String> executeErased(String method, String args) async {
    return context.execute(method: method, args: args);
  }

  static Future<Result> execute<Result>(Method<Result> method) async {
    final factory =
        SerdeRegistry.getByType(method.runtimeType)
            as SerdeFactory<Method<Result>>;

    final result = await context.execute(
      method: method.getName(),
      args: jsonEncode({'params': factory.serialize(method)}),
    );

    final json = jsonDecode(result);
    return SerdeRegistry.get<Result>().deserialize(json);
  }
}
