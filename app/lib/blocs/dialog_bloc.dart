import 'dart:async';
import 'dart:convert';

import 'package:bloc/bloc.dart';
import 'package:flutter/material.dart';
import 'package:stride/bridge/api/error.dart';
import 'package:stride/context.dart';
import 'package:stride/bridge/api/logging.dart' as logging;

@immutable
abstract class DialogEvent {}

final class DialogAlertEvent extends DialogEvent {
  final String title;
  final String? content;
  final Future<bool> Function(BuildContext context) onConfirm;
  final Future<bool> Function(BuildContext context)? onCancel;
  DialogAlertEvent({
    required this.title,
    required this.onConfirm,
    this.content,
    this.onCancel,
  });
}

class DialogState {
  final FutureOr<Widget> Function(BuildContext context) title;
  final FutureOr<Widget> Function(BuildContext context)? content;
  final FutureOr<bool> Function(BuildContext context) onConfirm;
  final FutureOr<bool> Function(BuildContext context)? onCancel;
  const DialogState({
    required this.title,
    required this.onConfirm,
    this.content,
    this.onCancel,
  });
}

class DialogBloc extends Bloc<DialogEvent, DialogState> {
  DialogBloc()
    : super(
        DialogState(
          title: (context) => const Placeholder(),
          onConfirm: (context) async => false,
        ),
      ) {
    RustContext.stream().listen((event) {
      final map = jsonDecode(event) as Map<String, dynamic>;
      if (map['method'] == 'stride.notification.prompt') {
        final params = map['params'] as Map<String, dynamic>;
        add(
          DialogAlertEvent(
            title: params['summary'] as String,
            content: 'Rust has requested a prompt. Do you want to continue?',
            onConfirm: (context) async {
              try {
                await RustContext.execute(
                  params['target'] as String,
                  jsonEncode({'params': params['inputs']}),
                );
              } on RustError catch (e) {
                logging.error(
                  message: 'Error executing Rust method: ${e.toErrorString()}',
                );
              }
              if (context.mounted) {
                Navigator.of(context).pop();
              }
              return true;
            },
            onCancel: (context) async {
              Navigator.of(context).pop();
              return false;
            },
          ),
        );
      }
    });

    on<DialogAlertEvent>((event, emit) async {
      emit(
        DialogState(
          title: (context) => Text(
            event.title,
            style: const TextStyle(fontWeight: FontWeight.bold),
          ),
          content: event.content == null
              ? null
              : (context) => Text(event.content!, softWrap: true),
          onConfirm: event.onConfirm,
          onCancel: event.onCancel,
        ),
      );
    });
  }
}
