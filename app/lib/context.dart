import 'dart:convert';

import 'package:flutter/foundation.dart';
import 'package:stride/api.dart';
import 'package:stride/bridge.dart';
import 'package:stride/notifications.dart';
import 'package:uuid/uuid.dart';

@immutable
class RepositoryStatus {
  final bool syncing;
  final String? error;
  const RepositoryStatus({required this.syncing, this.error});
}

class RustContext {
  static final ValueNotifier<List<RepositoryDescriptor>> repositories =
      ValueNotifier([]);
  static final ValueNotifier<Map<UuidValue, RepositoryStatus>>
  syncingRepositories = ValueNotifier({});

  static late ValueNotifier<Setting> setting;

  // Every isolate attaches to the already running worker on first use.
  static RustWorker get _worker => RustWorker.connect();

  static Stream<NativeNotification> stream() => _worker.notifications;

  static Future<void> refreshRepositories() async {
    final result = await RepositoryListMethod().execute();
    repositories.value = result.repositories;
  }

  static Future<void> init(String supportPath, String cachePath) async {
    await RustWorker.init(supportPath: supportPath, cachePath: cachePath);
    RustWorker.connect();

    stream().listen((event) async {
      final params = jsonDecode(event.params) as Map<String, dynamic>;
      print('${event.name}; $params');
      if (event.name == 'repository.changed') {
        final notification = RepositoryChangedNotification.fromJson(params);
        for (final change in notification.changes) {
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
              title = current;

              if (previous == null) {
                continue;
              }
            }

            body ??= '';

            // ignore: use_string_buffers
            body += '$type: ${previous ?? 'none'} -> ${current ?? 'none'}\n';
          }

          await NotificationService.show(
            '${isNewTask ? "New task" : "Task change"}: $title',
            body,
          );
        }
      } else if (event.name == 'repository.sync.started') {
        final notification = RepositorySyncStartedNotification.fromJson(params);
        syncingRepositories.value = {
          ...syncingRepositories.value,
          notification.repositoryId: const RepositoryStatus(syncing: true),
        };
      } else if (event.name == 'repository.sync.stopped') {
        final notification = RepositorySyncStoppedNotification.fromJson(params);
        syncingRepositories.value = {
          ...syncingRepositories.value,
          notification.repositoryId: RepositoryStatus(
            syncing: false,
            error: notification.error,
          ),
        };
      }
    });

    final result = await SettingGetMethod().execute();
    setting = ValueNotifier(result.setting);

    setting.addListener(() async {
      await SettingSetMethod(setting: setting.value).execute();
    });
  }

  static Future<String> executeErased(String method, String args) async {
    return _worker.execute(method: method, args: args);
  }

  static Future<Result> execute<Result>(Method<Result> method) async {
    final factory =
        SerdeRegistry.getByType(method.runtimeType)
            as SerdeFactory<Method<Result>>;

    final result = await _worker.execute(
      method: method.getName(),
      args: jsonEncode({'params': factory.serialize(method)}),
    );

    final json = jsonDecode(result);
    return SerdeRegistry.get<Result>().deserialize(json);
  }
}

extension RustContextExtension<Result> on Method<Result> {
  Future<Result> execute() async {
    return RustContext.execute(this);
  }
}
