import 'dart:async';

import 'package:bloc/bloc.dart';
import 'package:flutter/material.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/bridge/api/logging.dart';
import 'package:stride/bridge/api/settings.dart';

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
