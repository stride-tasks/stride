import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/src/rust/task.dart';
import 'package:stride/src/rust/task/annotation.dart';
import 'package:stride/utils/extensions.dart';
import 'package:stride/utils/functions.dart';
import 'package:stride/widgets/icon_text_button.dart';
import 'package:uuid/uuid.dart';

class TaskRoute extends StatefulWidget {
  final Task? task;
  const TaskRoute({
    super.key,
    this.task,
  });

  @override
  State<TaskRoute> createState() => _TaskRouteState();
}

class _TaskRouteState extends State<TaskRoute> {
  String description = '';
  DateTime? due;
  List<int> tags = [];
  List<Annotation> annotations = [];
  List<UuidValue> depends = [];
  // Map<String, String> uda = {};

  final _formKey = GlobalKey<FormState>();

  @override
  void initState() {
    super.initState();

    description = widget.task?.description ?? description;
    due = widget.task?.due;
    tags = widget.task?.tags.toList() ?? tags;
    annotations = widget.task?.annotations.toList() ?? annotations;
    depends = widget.task?.depends.toList() ?? depends;
    // uda = widget.task?.uda ?? uda;
  }

  String _dueButtonText() {
    const result = 'Due';
    if (due == null) {
      return result;
    }
    return '$result - ${due!.toHumanString()}';
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Tasks')),
      body: Container(
        padding: const EdgeInsets.symmetric(
          vertical: 10,
          horizontal: 20,
        ),
        child: SingleChildScrollView(
          child: Form(
            key: _formKey,
            child: Column(
              children: [
                TextFormField(
                  initialValue: description,
                  autofocus: true,
                  decoration: const InputDecoration(hintText: 'Description'),
                  validator: (value) {
                    if (value == null || value.isEmpty) {
                      return 'Task must have a description';
                    }
                    return null;
                  },
                  onSaved: (newValue) {
                    description = newValue!;
                  },
                  textCapitalization: TextCapitalization.sentences,
                  autovalidateMode: AutovalidateMode.onUserInteraction,
                ),
                Padding(
                  padding: const EdgeInsets.all(8.0),
                  child: IconTextButton(
                    icon: const Icon(Icons.date_range),
                    text: _dueButtonText(),
                    onPressed: () async {
                      final datetime = await showPickDateTime(context: context);
                      setState(() {
                        due = datetime;
                      });
                    },
                  ),
                ),
                // TagsWidget(
                //   tags: const [],
                //   onSubmit: (tags) {
                //     _tags = tags;
                //   },
                // ),
                // const SizedBox(height: 20),
              ],
            ),
          ),
        ),
      ),
      floatingActionButton: FloatingActionButton(
        shape: const CircleBorder(),
        onPressed: () async {
          if (!_formKey.currentState!.validate()) {
            return;
          }

          _formKey.currentState!.save();

          if (context.mounted) {
            final task = Task.raw(
              uuid:
                  widget.task?.uuid ?? UuidValue.fromString(const Uuid().v7()),
              description: description,
              tags: Uint32List.fromList(tags),
              due: due,
              status: TaskStatus.pending,
              annotations: annotations,
              depends: depends,
              uda: {},
            );

            if (widget.task == null) {
              context.read<TaskBloc>().add(TaskAddEvent(task: task));
            } else {
              context.read<TaskBloc>().add(TaskUpdateEvent(task: task));
            }

            final text =
                widget.task == null ? 'New Task added' : 'Task modified';

            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(
                content: Text(text),
                behavior: SnackBarBehavior.floating,
              ),
            );
            Navigator.pop(context);
          }
        },
        child: const Icon(Icons.add_task_sharp, size: 50),
      ),
    );
  }
}
