import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/src/rust/task.dart';
import 'package:stride/utils/functions.dart';
import 'package:stride/widgets/custom_app_bar.dart';
import 'package:stride/widgets/icon_text_button.dart';
import 'package:stride/widgets/tags_widget.dart';
import 'package:uuid/uuid.dart';

class TaskAddRoute extends StatefulWidget {
  const TaskAddRoute({super.key});

  @override
  State<TaskAddRoute> createState() => _TaskAddRouteState();
}

class _TaskAddRouteState extends State<TaskAddRoute> {
  String title = "";
  DateTime? _selectedDay;
  List<String> _tags = [];

  final _formKey = GlobalKey<FormState>();

  String _dueButtonText() {
    String result = "Due";
    if (_selectedDay == null) {
      return result;
    }
    return "$result - ${_selectedDay!.toIso8601String()}";
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
          child: Form(
            key: _formKey,
            child: Column(
              children: [
                TextFormField(
                  autofocus: true,
                  decoration: const InputDecoration(
                    hintText: "Description",
                  ),
                  validator: (value) {
                    if (value == null || value.isEmpty) {
                      return "Cannot add task without a description";
                    }
                    return null;
                  },
                  onSaved: (newValue) {
                    title = newValue!;
                  },
                  autovalidateMode: AutovalidateMode.onUserInteraction,
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
            context.read<TaskBloc>().add(
                  TaskAddEvent(
                    task: Task.raw(
                      uuid: UuidValue.fromString(const Uuid().v4()),
                      entry: DateTime.now(),
                      description: title,
                      tags: _tags,
                      due: _selectedDay,
                      status: TaskStatus.pending,
                      annotations: [],
                      depends: [],
                      uda: {},
                    ),
                  ),
                );
            ScaffoldMessenger.of(context).showSnackBar(
              const SnackBar(
                content: Text('New Task added'),
                behavior: SnackBarBehavior.floating,
              ),
            );
            Navigator.pop(context);
          }
        },
        child: Icon(
          color: Theme.of(context).primaryColor,
          Icons.add_task_sharp,
          size: 50,
        ),
      ),
    );
  }
}
