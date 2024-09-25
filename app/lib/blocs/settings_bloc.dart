import 'dart:async';

import 'package:bloc/bloc.dart';
import 'package:flutter/material.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/bridge/api/logging.dart';
import 'package:stride/bridge/api/settings.dart';
import 'package:stride/bridge/git/known_hosts.dart';

@immutable
abstract class SettingsEvent {}

final class SettingsRefreshEvent extends SettingsEvent {
  final Settings settings;
  SettingsRefreshEvent({required this.settings});
}

final class SettingsUpdateEvent extends SettingsEvent {
  final Settings settings;
  SettingsUpdateEvent({required this.settings});
}

final class SettingsRemoveKnownHostEvent extends SettingsEvent {
  final Host host;
  SettingsRemoveKnownHostEvent({required this.host});
}

final class SettingsAddKnownHostEvent extends SettingsEvent {
  final Host host;
  SettingsAddKnownHostEvent({required this.host});
}

final class SettingsAddEncryptionKeyEvent extends SettingsEvent {
  final EncryptionKey key;
  SettingsAddEncryptionKeyEvent({required this.key});
}

final class SettingsToggleTheme extends SettingsEvent {
  SettingsToggleTheme();
}

class SettingsState {
  final Settings settings;
  const SettingsState({required this.settings});
}

class SettingsBloc extends Bloc<SettingsEvent, SettingsState> {
  LogBloc logBloc;
  Settings settings;
  late Stream<Settings> _stream;
  late StreamSubscription<Settings> _streamSubscription;

  SettingsBloc({
    required this.settings,
    required this.logBloc,
  }) : super(SettingsState(settings: settings)) {
    _stream = Settings.createStream();
    _streamSubscription = _stream.listen(
      (event) => add(SettingsRefreshEvent(settings: event)),
      onError: (Object error, StackTrace stackTrace) {
        Logger.error(
          message:
              'ERROR: settings stream error: $error\n\nDart Backtrace:\n$stackTrace',
        );
      },
    );

    on<SettingsRefreshEvent>((event, emit) async {
      emit(SettingsState(settings: event.settings));
    });

    on<SettingsUpdateEvent>((event, emit) async {
      await Settings.save(settings: event.settings);
    });

    on<SettingsRemoveKnownHostEvent>((event, emit) async {
      settings = settings.copyWith(
        knownHosts: settings.knownHosts.copyWith(
          hosts: settings.knownHosts.hosts.toList()
            ..removeWhere(
              (element) =>
                  element.hostname == event.host.hostname &&
                  element.keyType == event.host.keyType,
            ),
        ),
      );
      await Settings.save(settings: settings);
    });

    on<SettingsAddKnownHostEvent>((event, emit) async {
      settings = settings.copyWith(
        knownHosts: settings.knownHosts.copyWith(
          hosts: settings.knownHosts.hosts.toList()..add(event.host),
        ),
      );
      await Settings.save(settings: settings);
    });

    on<SettingsAddEncryptionKeyEvent>((event, emit) async {
      settings = settings.copyWith(
        encryptionKeys: settings.encryptionKeys.toList()..add(event.key),
      );
      await Settings.save(settings: settings);
    });

    on<SettingsToggleTheme>((event, emit) async {
      settings = settings.copyWith(darkMode: !settings.darkMode);
      await Settings.save(settings: settings);
    });
  }

  @override
  void onError(Object error, StackTrace stackTrace) {
    super.onError(error, stackTrace);
    logBloc.add(LogErrorEvent(error: error, stackTrace: stackTrace));
  }

  @override
  Future<void> close() {
    _streamSubscription.cancel();
    return super.close();
  }
}
