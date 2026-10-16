import 'dart:async';

import 'package:bloc/bloc.dart';
import 'package:flutter/material.dart';
import 'package:stride/api.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/context.dart';

@immutable
abstract class RepositoryEvent {}

final class RepositoryRefreshEvent extends RepositoryEvent {}

@immutable
class RepositoryState {
  final List<RepositoryDescriptor> repositories;

  const RepositoryState({required this.repositories});
}

class RepositoryBloc extends Bloc<RepositoryEvent, RepositoryState> {
  final LogBloc logBloc;
  late final StreamSubscription<SettingsState> _settingsSubscription;

  RepositoryBloc({
    required List<RepositoryDescriptor> repositories,
    required SettingsBloc settingsBloc,
    required this.logBloc,
  }) : super(RepositoryState(repositories: repositories)) {
    _settingsSubscription = settingsBloc.stream.listen(
      (_) => add(RepositoryRefreshEvent()),
    );

    on<RepositoryRefreshEvent>((event, emit) async {
      final result = await RepositoryListMethod().execute();
      emit(RepositoryState(repositories: result.repositories));
    });
  }

  @override
  void onError(Object error, StackTrace stackTrace) {
    super.onError(error, stackTrace);
    logBloc.add(
      LogErrorEvent(
        error: error,
        stackTrace: stackTrace,
        message: 'repository',
      ),
    );
  }

  @override
  Future<void> close() async {
    await _settingsSubscription.cancel();
    return super.close();
  }
}
