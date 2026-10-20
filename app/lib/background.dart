import 'dart:async';
import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter_foreground_task/flutter_foreground_task.dart';
import 'package:stride/api/generated.dart';
import 'package:stride/context.dart';
import 'package:uuid/uuid.dart';

@immutable
class RepositorySyncStatus {
  final String repositoryId;
  final bool syncing;
  final String? error;

  const RepositorySyncStatus({
    required this.repositoryId,
    required this.syncing,
    this.error,
  });

  Map<String, dynamic> toMap() => {
    'repositoryId': repositoryId,
    'syncing': syncing,
    'error': error,
  };

  factory RepositorySyncStatus.fromMap(Map<String, dynamic> map) =>
      RepositorySyncStatus(
        repositoryId: map['repositoryId'] as String,
        syncing: map['syncing'] as bool,
        error: map['error'] as String?,
      );
}

class Background {
  static const _syncInterval = Duration(minutes: 5);
  static final _statuses = <String, RepositorySyncStatus>{};
  static final _statusController =
      StreamController<Map<String, RepositorySyncStatus>>.broadcast();

  static bool get _isSupported => Platform.isAndroid || Platform.isIOS;

  static Stream<Map<String, RepositorySyncStatus>> stream() =>
      _statusController.stream;

  static void init() {
    if (!_isSupported) return;

    FlutterForegroundTask.initCommunicationPort();
    FlutterForegroundTask.addTaskDataCallback(_onReceiveTaskData);
    FlutterForegroundTask.init(
      androidNotificationOptions: AndroidNotificationOptions(
        channelId: 'stride.repository.sync',
        channelName: 'Stride repository sync',
        channelDescription: 'Keeps repositories synchronized.',
        onlyAlertOnce: true,
      ),
      iosNotificationOptions: const IOSNotificationOptions(
        showNotification: false,
        playSound: false,
      ),
      foregroundTaskOptions: ForegroundTaskOptions(
        eventAction: ForegroundTaskEventAction.repeat(
          _syncInterval.inMilliseconds,
        ),
        allowWakeLock: true,
        allowWifiLock: true,
      ),
    );
  }

  static Future<void> configurePeriodicSync({
    required bool enabled,
    required List<UuidValue> repositoryIds,
  }) async {
    if (!_isSupported) {
      if (enabled) {
        debugPrint(
          'Periodic repository sync is unavailable outside Android and iOS.',
        );
      }
      return;
    }

    if (!enabled || repositoryIds.isEmpty) {
      await stop();
      return;
    }

    final ids = repositoryIds.map((id) => id.uuid).toList(growable: false);

    if (await FlutterForegroundTask.isRunningService) {
      FlutterForegroundTask.sendDataToTask({'repositoryIds': ids});
      return;
    }

    if (Platform.isAndroid) {
      var permission =
          await FlutterForegroundTask.checkNotificationPermission();
      if (permission != NotificationPermission.granted) {
        await FlutterForegroundTask.requestNotificationPermission();
        permission = await FlutterForegroundTask.checkNotificationPermission();
      }
      if (permission != NotificationPermission.granted) {
        debugPrint(
          'Cannot start periodic repository sync without notification permission.',
        );
        return;
      }
    }

    final result = await FlutterForegroundTask.startService(
      serviceTypes: Platform.isAndroid
          ? const [ForegroundServiceTypes.dataSync]
          : null,
      notificationTitle: 'Stride repository sync',
      notificationText: 'Periodically syncing repositories',
      callback: _entryPoint,
    );
    if (result is ServiceRequestFailure) {
      // ignore: only_throw_errors
      throw result.error;
    }

    FlutterForegroundTask.sendDataToTask({'repositoryIds': ids});
  }

  static Future<void> stop() async {
    if (!_isSupported || !await FlutterForegroundTask.isRunningService) return;

    final result = await FlutterForegroundTask.stopService();
    if (result is ServiceRequestFailure) {
      // ignore: only_throw_errors
      throw result.error;
    }
  }

  static Future<void> syncRepository(
    UuidValue repositoryId, {
    void Function(RepositorySyncStatus status)? onStatus,
  }) async {
    void publish({required bool syncing, String? error}) {
      final status = RepositorySyncStatus(
        repositoryId: repositoryId.uuid,
        syncing: syncing,
        error: error,
      );
      if (onStatus == null) {
        _publish(status);
      } else {
        onStatus(status);
      }
    }

    publish(syncing: true);

    try {
      await RustContext.execute(RepositorySyncMethod(id: repositoryId));
      publish(syncing: false);
    } catch (error) {
      publish(syncing: false, error: error.toString());
      rethrow;
    }
  }

  static void _onReceiveTaskData(Object data) {
    if (data is! Map) {
      debugPrint('Ignoring invalid repository sync update: $data');
      return;
    }
    _publish(RepositorySyncStatus.fromMap(Map<String, dynamic>.from(data)));
  }

  static void _publish(RepositorySyncStatus status) {
    _statuses[status.repositoryId] = status;
    _statusController.add(Map.unmodifiable(_statuses));
  }
}

@pragma('vm:entry-point')
void _entryPoint() {
  FlutterForegroundTask.setTaskHandler(_RepositorySyncTaskHandler());
}

class _RepositorySyncTaskHandler extends TaskHandler {
  List<String> _repositoryIds = [];
  var _isSyncing = false;

  @override
  Future<void> onStart(DateTime timestamp, TaskStarter starter) async {
    // The app sends repository IDs immediately after starting the service.
  }

  @override
  void onRepeatEvent(DateTime timestamp) {
    unawaited(_syncRepositories());
  }

  @override
  Future<void> onDestroy(DateTime timestamp, bool isTimeout) async {}

  @override
  void onReceiveData(Object data) {
    final repositoryIds = data is Map ? data['repositoryIds'] : null;
    if (repositoryIds is! List ||
        repositoryIds.any((repositoryId) => repositoryId is! String)) {
      debugPrint('Ignoring invalid repository list sent to sync service.');
      return;
    }

    _repositoryIds = List<String>.from(repositoryIds);
    unawaited(_syncRepositories());
  }

  Future<void> _syncRepositories() async {
    if (_isSyncing || _repositoryIds.isEmpty) return;

    _isSyncing = true;
    try {
      for (final repositoryId in _repositoryIds) {
        try {
          await Background.syncRepository(
            UuidValue.fromString(repositoryId),
            onStatus: (status) =>
                FlutterForegroundTask.sendDataToMain(status.toMap()),
          );
          // ignore: avoid_catches_without_on_clauses
        } catch (error) {
          debugPrint('Repository sync failed for $repositoryId: $error');
        }
      }
    } finally {
      _isSyncing = false;
    }
  }
}
