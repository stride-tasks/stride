import 'package:flutter/foundation.dart';
import 'package:stride/api.dart';
import 'package:stride/context.dart';
import 'package:uuid/uuid.dart';

/// The application settings, stored in the global storage.
@immutable
class Setting {
  static const currentRepositoryKey = 'core:current-repository';
  static const periodicSyncKey = 'app:periodic-sync';
  static const periodicSyncIntervalKey = 'app:periodic-sync-interval';
  static const darkModeKey = 'app:dark-mode';

  static const keys = [
    currentRepositoryKey,
    periodicSyncKey,
    periodicSyncIntervalKey,
    darkModeKey,
  ];

  static const _unset = Object();

  final UuidValue? currentRepository;
  final bool periodicSync;
  final int periodicSyncInterval;
  final bool darkMode;

  const Setting({
    this.currentRepository,
    this.periodicSync = false,
    this.periodicSyncInterval = 300,
    this.darkMode = true,
  });

  /// Passing `null` as [currentRepository] clears it.
  Setting copyWith({
    Object? currentRepository = _unset,
    bool? periodicSync,
    int? periodicSyncInterval,
    bool? darkMode,
  }) {
    return Setting(
      currentRepository: identical(currentRepository, _unset)
          ? this.currentRepository
          : currentRepository as UuidValue?,
      periodicSync: periodicSync ?? this.periodicSync,
      periodicSyncInterval: periodicSyncInterval ?? this.periodicSyncInterval,
      darkMode: darkMode ?? this.darkMode,
    );
  }

  /// Applies a storage change, a `null` [value] resets the key to its default.
  Setting applyStorage(String key, ConfigValue? value) {
    switch ((key, value)) {
      case (currentRepositoryKey, ConfigValue_Uuid(:final value)):
        return copyWith(currentRepository: value);
      case (currentRepositoryKey, null):
        return copyWith(currentRepository: null);
      case (periodicSyncKey, ConfigValue_Boolean(:final value)):
        return copyWith(periodicSync: value);
      case (periodicSyncKey, null):
        return copyWith(periodicSync: const Setting().periodicSync);
      case (periodicSyncIntervalKey, ConfigValue_Integer(:final value)):
        return copyWith(periodicSyncInterval: value);
      case (periodicSyncIntervalKey, null):
        return copyWith(
          periodicSyncInterval: const Setting().periodicSyncInterval,
        );
      case (darkModeKey, ConfigValue_Boolean(:final value)):
        return copyWith(darkMode: value);
      case (darkModeKey, null):
        return copyWith(darkMode: const Setting().darkMode);
      default:
        return this;
    }
  }

  /// The storage values of this setting, a `null` value means the key is removed.
  Map<String, ConfigValue?> toStorage() => {
    currentRepositoryKey: currentRepository == null
        ? null
        : ConfigValue.uuid(value: currentRepository!),
    periodicSyncKey: ConfigValue.boolean(value: periodicSync),
    periodicSyncIntervalKey: ConfigValue.integer(value: periodicSyncInterval),
    darkModeKey: ConfigValue.boolean(value: darkMode),
  };

  @override
  bool operator ==(Object other) =>
      other is Setting &&
      other.currentRepository == currentRepository &&
      other.periodicSync == periodicSync &&
      other.periodicSyncInterval == periodicSyncInterval &&
      other.darkMode == darkMode;

  @override
  int get hashCode => Object.hash(
    currentRepository,
    periodicSync,
    periodicSyncInterval,
    darkMode,
  );
}

/// Keeps a [ValueNotifier] of [Setting] in sync with the global storage.
class SettingStorage {
  final ValueNotifier<Setting> notifier;

  // What the storage is known to contain.
  Setting _persisted;

  // Our own writes whose change notification has not arrived yet.
  final List<(String, ConfigValue?)> _echoes = [];

  SettingStorage._(this.notifier) : _persisted = notifier.value;

  static Future<SettingStorage> load() async {
    final result = await StorageGetMethod(items: Setting.keys).execute();

    var setting = const Setting();
    for (final item in result.items) {
      setting = setting.applyStorage(item.key, item.value);
    }

    final storage = SettingStorage._(ValueNotifier(setting));
    storage.notifier.addListener(storage._persist);
    return storage;
  }

  Future<void> _persist() async {
    final previous = _persisted.toStorage();
    final current = notifier.value.toStorage();
    _persisted = notifier.value;

    final removed = <String>[];
    for (final MapEntry(:key, :value) in current.entries) {
      if (previous[key] == value) continue;

      _echoes.add((key, value));
      if (value == null) {
        removed.add(key);
      } else {
        await StorageSetMethod(key: key, value: value).execute();
      }
    }
    if (removed.isNotEmpty) {
      await StorageRemoveMethod(items: removed).execute();
    }
  }

  /// Handles a global storage notification, a `null` [value] means the key was removed.
  void onChanged(String key, ConfigValue? value) {
    if (_echoes.remove((key, value))) return;

    _persisted = _persisted.applyStorage(key, value);
    notifier.value = notifier.value.applyStorage(key, value);
  }
}
