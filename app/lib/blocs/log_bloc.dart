import 'package:bloc/bloc.dart';
import 'package:flutter/material.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:stride/bridge/api/error.dart';
import 'package:stride/bridge/api/logging.dart';
import 'package:stride/utils/classes.dart';

@immutable
abstract class LogEvent {}

final class LogMessageEvent extends LogEvent {
  final String message;
  final bool show;
  LogMessageEvent({required this.message, this.show = true});
}

final class LogErrorEvent extends LogEvent {
  final Object error;
  final StackTrace? stackTrace;
  final String message;
  final bool show;
  LogErrorEvent({
    required this.error,
    this.stackTrace,
    this.message = '',
    this.show = true,
  });
}

class LogState {
  final String message;
  final bool isError;
  final bool show;
  const LogState({
    required this.message,
    this.show = true,
    this.isError = false,
  });
}

class LogBloc extends Bloc<LogEvent, LogState> {
  LogBloc() : super(const LogState(message: '')) {
    on<LogMessageEvent>((event, emit) async {
      emit(LogState(message: event.message, show: event.show));
    });

    on<LogErrorEvent>((event, emit) async {
      final message = _errorToString(
        event.error,
        event.stackTrace,
        message: event.message,
      );
      Logger.error(message: message);
      emit(
        LogState(
          message: message,
          show: event.show,
          isError: true,
        ),
      );
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

    return '${message}error: $errorString\n\nDart Backtrace:\n$stackTrace';
  }

  Future<Result<T, Object>> catch_<T>(
    Future<T> Function() f, {
    String message = '',
  }) async {
    final result = await Result.catch_<T, Object>(f);
    if (result case Err(error: (final error, final stackTrace))) {
      add(
        LogErrorEvent(
          error: error,
          stackTrace: stackTrace,
          message: message,
        ),
      );
    }
    return result.mapErr((caughtError) => caughtError.$1);
  }
}
