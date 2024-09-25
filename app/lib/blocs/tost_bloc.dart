import 'package:bloc/bloc.dart';
import 'package:flutter/material.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:stride/bridge/api/error.dart';
import 'package:stride/bridge/api/logging.dart';
import 'package:stride/utils/classes.dart';

@immutable
abstract class TostEvent {}

final class TostMessageEvent extends TostEvent {
  final String message;
  TostMessageEvent({required this.message});
}

final class TostErrorEvent extends TostEvent {
  final Object error;
  final StackTrace? stackTrace;
  TostErrorEvent({required this.error, this.stackTrace});
}

class TostState {
  final String message;
  final bool isError;
  const TostState({
    required this.message,
    this.isError = false,
  });
}

class TostBloc extends Bloc<TostEvent, TostState> {
  TostBloc() : super(TostState(message: '')) {
    on<TostMessageEvent>((event, emit) async {
      emit(TostState(message: event.message));
    });

    on<TostErrorEvent>((event, emit) async {
      final message = _errorToString(event.error, event.stackTrace);
      Logger.error(message: message);
      emit(TostState(message: message, isError: true));
    });
  }

  String _errorToString(
    Object error,
    StackTrace? stackTrace, {
    String message = '',
  }) {
    var errorString = '';
    switch (error) {
      case final RustError error:
        errorString = error.toErrorString();
      case final AnyhowException error:
        errorString = error.message;
      default:
        errorString = error.toString();
    }

    if (message.isNotEmpty) {
      message += ' ';
    }

    final value =
        '${message}error: $errorString\n\nDart Backtrace:\n$stackTrace';
    return value;
  }

  Future<Result<T, Object>> catch_<T>(Future<T> Function() f) async {
    final result = await Result.catch_<T, Object>(f);
    if (result case Err(error: (final error, final stackTrace))) {
      add(TostErrorEvent(error: error, stackTrace: stackTrace));
    }
    return result.mapErr((caughtError) => caughtError.$1);
  }
}
