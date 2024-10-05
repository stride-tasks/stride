import 'dart:async';

import 'package:bloc/bloc.dart';
import 'package:flutter/material.dart';

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
