import 'package:bloc/bloc.dart';
import 'package:flutter/material.dart';
import 'package:stride/bridge/api/settings.dart';
import 'package:stride/bridge/git/known_hosts.dart';

@immutable
abstract class SettingsEvent {}

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

final class SettingsToggleTheme extends SettingsEvent {
  SettingsToggleTheme();
}

class SettingsState {
  final Settings settings;
  const SettingsState({required this.settings});
}

class SettingsBloc extends Bloc<SettingsEvent, SettingsState> {
  Settings settings;

  SettingsBloc({
    required this.settings,
  }) : super(SettingsState(settings: settings)) {
    on<SettingsUpdateEvent>((event, emit) async {
      settings = event.settings;
      await Settings.save(settings: settings);
      emit(SettingsState(settings: settings));
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
      emit(SettingsState(settings: settings));
    });

    on<SettingsAddKnownHostEvent>((event, emit) async {
      settings = settings.copyWith(
        knownHosts: settings.knownHosts.copyWith(
          hosts: settings.knownHosts.hosts.toList()..add(event.host),
        ),
      );
      await Settings.save(settings: settings);
      emit(SettingsState(settings: settings));
    });

    on<SettingsToggleTheme>((event, emit) async {
      settings = settings.copyWith(darkMode: !settings.darkMode);
      await Settings.save(settings: settings);
      emit(SettingsState(settings: settings));
    });
  }
}
