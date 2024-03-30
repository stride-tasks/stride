import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/src/rust/task.dart';
import 'package:stride/utils/extensions.dart';
import 'package:stride/utils/functions.dart';
import 'package:stride/widgets/custom_app_bar.dart';
import 'package:stride/widgets/icon_text_button.dart';
import 'package:stride/widgets/tags_widget.dart';

class TaskEditRoute extends StatefulWidget {
  final Task task;
  const TaskEditRoute({super.key, required this.task});

  @override
  State<TaskEditRoute> createState() => _TaskEditRouteState();
}

class _TaskEditRouteState extends State<TaskEditRoute> {
  late TextEditingController description;
  DateTime? _selectedDay;
  List<String> _tags = [];

  String _dueButtonText() {
    String result = "Due";
    if (_selectedDay == null) {
      return result;
    }
    return "$result - ${_selectedDay!.toHumanString()}";
  }

  @override
  void initState() {
    super.initState();

    description = TextEditingController(text: widget.task.description);
    _selectedDay = widget.task.due;
    _tags = widget.task.tags.toList();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: const CustomAppBar(title: "Tasks"),
      body: Container(
        padding: const EdgeInsets.symmetric(
          vertical: 10,
          horizontal: 20,
        ),
        child: SingleChildScrollView(
          child: Column(
            children: [
              const SizedBox(height: 20),
              TextField(
                controller: description,
                autofocus: true,
                decoration: const InputDecoration(
                  hintText: "Description",
                ),
              ),
              Padding(
                padding: const EdgeInsets.all(8.0),
                child: IconTextButton(
                  icon: const Icon(Icons.date_range),
                  text: _dueButtonText(),
                  onPressed: () async {
                    var datetime = await showPickDateTime(context: context);
                    setState(() {
                      _selectedDay = datetime;
                    });
                  },
                ),
              ),
              // Padding(
              //   padding: const EdgeInsets.symmetric(
              //     vertical: 5,
              //     horizontal: 10,
              //   ),
              //   child: TagsWidget(
              //     tags: const [],
              //     onSubmit: (tags) {
              //       _tags = tags;
              //     },
              //   ),
              // ),
            ],
          ),
        ),
      ),
      floatingActionButton: FloatingActionButton(
        shape: const CircleBorder(),
        onPressed: () async {
          if (description.value.text.isEmpty) {
            ScaffoldMessenger.of(context).showSnackBar(
              const SnackBar(
                content: Text('Cannot set task without a description'),
                behavior: SnackBarBehavior.floating,
              ),
            );
            return;
          }

          if (context.mounted) {
            context.read<TaskBloc>().add(
                  TaskUpdateEvent(
                    task: Task.raw(
                      uuid: widget.task.uuid,
                      entry: widget.task.entry,
                      description: description.text,
                      tags: _tags,
                      due: _selectedDay,
                      status: TaskStatus.pending,
                      annotations: widget.task.annotations,
                      depends: widget.task.depends,
                      uda: widget.task.uda,
                      modified: DateTime.now(),
                    ),
                  ),
                );
            Navigator.pop(context);
          }
        },
        child: Icon(
          color: Theme.of(context).primaryColor,
          Icons.check_circle,
          size: 50,
        ),
      ),
    );
  }
}
