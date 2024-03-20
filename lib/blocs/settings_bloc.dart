import 'package:bloc/bloc.dart';
import 'package:flutter/material.dart';
import 'package:stride/src/rust/api/settings.dart';
import 'package:stride/src/rust/git/known_hosts.dart';
import 'package:uuid/uuid.dart';

@immutable
abstract class SettingsEvent {}

final class SettingsFetchEvent extends SettingsEvent {}

final class SettingsUpdateEvent extends SettingsEvent {
  final Settings settings;
  SettingsUpdateEvent({required this.settings});
}

final class SettingsRemoveSshKeyEvent extends SettingsEvent {
  final UuidValue uuid;
  SettingsRemoveSshKeyEvent({required this.uuid});
}

final class SettingsRemoveKnownHostEvent extends SettingsEvent {
  final Host host;
  SettingsRemoveKnownHostEvent({required this.host});
}

final class SettingsAddKnownHostEvent extends SettingsEvent {
  final Host host;
  SettingsAddKnownHostEvent({required this.host});
}

final class SettingsAddSshKeyEvent extends SettingsEvent {
  final SshKey key;
  SettingsAddSshKeyEvent({required this.key});
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
    on<SettingsFetchEvent>((event, emit) async {
      settings = await Settings.load();
      emit(SettingsState(settings: settings));
    });

    on<SettingsUpdateEvent>((event, emit) async {
      settings = event.settings;
      await Settings.save(settings: settings);
      emit(SettingsState(settings: settings));
    });

    on<SettingsRemoveSshKeyEvent>((event, emit) async {
      settings = settings.copyWith(
        keys: settings.keys.toList()
          ..removeWhere((element) => element.uuid == event.uuid),
      );
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
                  element.remoteKeyType == event.host.remoteKeyType,
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

    on<SettingsAddSshKeyEvent>((event, emit) async {
      settings = settings.copyWith(
        keys: settings.keys.toList()..add(event.key),
      );
      await Settings.save(settings: settings);
      emit(SettingsState(settings: settings));
    });
  }
}
