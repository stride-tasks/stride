import 'dart:async';
import 'dart:io';

import 'package:connectivity_plus/connectivity_plus.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter_foreground_task/flutter_foreground_task.dart';
import 'package:stride/api/generated.dart';
import 'package:stride/bridge.dart';
import 'package:stride/context.dart';
import 'package:stride/logging.dart';
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
  static const minimumSyncIntervalSeconds = 5;
  static final _statuses = <String, RepositorySyncStatus>{};
  static final _statusController =
      StreamController<Map<String, RepositorySyncStatus>>.broadcast();
  static var _syncEnabled = false;
  static var _syncLoopRunning = false;
  static var _syncIntervalSeconds = minimumSyncIntervalSeconds;
  static var _hasNetworkConnectivity = false;
  static List<UuidValue> _repositoryIds = const [];

  static bool get _isSupported => Platform.isAndroid || Platform.isIOS;

  static Stream<Map<String, RepositorySyncStatus>> stream() =>
      _statusController.stream;

  static Future<void> init() async {
    Connectivity().onConnectivityChanged.listen(
      _onConnectivityChanged,
      onError: (Object error, StackTrace stackTrace) {
        Log.error(
          'Error occurred while listening to connectivity changes: $error\n$stackTrace',
        );
      },
    );
    try {
      _hasNetworkConnectivity = _isConnected(
        await Connectivity().checkConnectivity(),
      );
      Log.debug(
        'Initial network connectivity: ${_hasNetworkConnectivity ? 'available' : 'unavailable'}',
      );
    } catch (error, stackTrace) {
      Log.reportError(error, stackTrace: stackTrace);
      rethrow;
    }

    if (_isSupported) {
      FlutterForegroundTask.initCommunicationPort();
      FlutterForegroundTask.addTaskDataCallback(_onReceiveTaskData);
      _configureForegroundTask(RustContext.setting.value.periodicSyncInterval);
    } else {
      configurePeriodicSync(
        enabled: _syncEnabled,
        repositoryIds: _repositoryIds,
        intervalSeconds: _syncIntervalSeconds,
      );
    }
  }

  static bool _isConnected(List<ConnectivityResult> results) =>
      results.any((result) => result != ConnectivityResult.none);

  static void _onConnectivityChanged(List<ConnectivityResult> results) {
    final hasConnectivity = _isConnected(results);
    if (_hasNetworkConnectivity == hasConnectivity) return;

    _hasNetworkConnectivity = hasConnectivity;
    Log.debug(
      'Network connectivity changed: ${hasConnectivity ? 'available' : 'unavailable'}',
    );

    try {
      configurePeriodicSync(
        enabled: _hasNetworkConnectivity && _syncEnabled,
        repositoryIds: _repositoryIds,
        intervalSeconds: _syncIntervalSeconds,
      );
    } catch (error, stackTrace) {
      Log.reportError(error, stackTrace: stackTrace);
    }
  }

  static void _configureForegroundTask(int intervalSeconds) {
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
      foregroundTaskOptions: _foregroundTaskOptions(intervalSeconds),
    );
  }

  static void setSyncIntervalSeconds(int seconds) {
    if (seconds < minimumSyncIntervalSeconds) {
      throw ArgumentError.value(
        seconds,
        'seconds',
        'Sync interval must be at least $minimumSyncIntervalSeconds seconds.',
      );
    }

    RustContext.setting.value = RustContext.setting.value.copyWith(
      periodicSyncInterval: seconds,
    );
  }

  static Future<void> configurePeriodicSync({
    required bool enabled,
    required List<UuidValue> repositoryIds,
    required int intervalSeconds,
  }) async {
    _syncEnabled = _hasNetworkConnectivity && enabled;
    _syncIntervalSeconds = intervalSeconds;
    _repositoryIds = List.of(repositoryIds, growable: false);

    if (!_isSupported) {
      if (_syncEnabled && _repositoryIds.isNotEmpty) {
        unawaited(_runDesktopSyncLoop());
      }
      return;
    }

    final serviceIsRunning = await FlutterForegroundTask.isRunningService;
    if (!_syncEnabled || _repositoryIds.isEmpty) {
      if (serviceIsRunning) {
        await stop();
      } else {
        _configureForegroundTask(_syncIntervalSeconds);
      }
      return;
    }

    if (serviceIsRunning) {
      final result = await FlutterForegroundTask.updateService(
        foregroundTaskOptions: _foregroundTaskOptions(_syncIntervalSeconds),
      );
      if (result is ServiceRequestFailure) {
        // ignore: only_throw_errors
        throw result.error;
      }
      FlutterForegroundTask.sendDataToTask({
        'repositoryIds': _repositoryIds.map((id) => id.uuid).toList(),
      });
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
        Log.notify(
          'Cannot start periodic repository sync without notification permission.',
        );
        return;
      }
    }

    _configureForegroundTask(_syncIntervalSeconds);
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

    FlutterForegroundTask.sendDataToTask({
      'repositoryIds': _repositoryIds.map((id) => id.uuid).toList(),
    });
  }

  static Future<void> _runDesktopSyncLoop() async {
    if (_syncLoopRunning) return;

    _syncLoopRunning = true;

    try {
      outerLoop:
      while (_syncLoopRunning && _repositoryIds.isNotEmpty) {
        Log.debug('Starting repository sync loop');
        for (final repositoryId in _repositoryIds) {
          Log.debug('Syncing repository $repositoryId');
          try {
            if (!_syncEnabled) {
              Log.debug(
                'Syncing is disabled, skipping repository $repositoryId',
              );
              break outerLoop;
            }
            await syncRepository(repositoryId);
            // ignore: avoid_catches_without_on_clauses
          } catch (error, stackTrace) {
            Log.error(
              'Repository sync failed for $repositoryId: $error\n$stackTrace',
            );
          }
        }

        Log.trace(
          'Repository sync loop completed, waiting for next interval ($_syncIntervalSeconds)...',
        );
        await Future<void>.delayed(Duration(seconds: _syncIntervalSeconds));
      }
    } finally {
      _syncLoopRunning = false;
    }
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
      Log.error('Ignoring invalid repository sync update: $data');
      return;
    }
    _publish(RepositorySyncStatus.fromMap(Map<String, dynamic>.from(data)));
  }

  static void _publish(RepositorySyncStatus status) {
    _statuses[status.repositoryId] = status;
    _statusController.add(Map.unmodifiable(_statuses));
  }

  static ForegroundTaskOptions _foregroundTaskOptions(int seconds) =>
      ForegroundTaskOptions(
        eventAction: ForegroundTaskEventAction.repeat(
          Duration(seconds: seconds).inMilliseconds,
        ),
        allowWakeLock: true,
        allowWifiLock: true,
      );
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
    RustWorker.connect();

    final repositoryIds = data is Map ? data['repositoryIds'] : null;
    if (repositoryIds is! List ||
        repositoryIds.any((repositoryId) => repositoryId is! String)) {
      Log.error('Ignoring invalid repository list sent to sync service.');
      return;
    }

    _repositoryIds = List<String>.from(repositoryIds);
    unawaited(_syncRepositories());
  }

  Future<void> _syncRepositories() async {
    RustWorker.connect();

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
          Log.error('Repository sync failed for $repositoryId: $error');
        }
      }
    } finally {
      _isSyncing = false;
    }
  }
}
