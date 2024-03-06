import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/src/rust/task.dart';
import 'package:stride/widgets/custom_app_bar.dart';
import 'package:stride/widgets/tags_widget.dart';
import 'package:table_calendar/table_calendar.dart';

class TaskEditRoute extends StatefulWidget {
  final Task task;
  const TaskEditRoute({super.key, required this.task});

  @override
  State<TaskEditRoute> createState() => _TaskEditRouteState();
}

class _TaskEditRouteState extends State<TaskEditRoute> {
  late TextEditingController description;
  CalendarFormat _calendarFormat = CalendarFormat.month;
  final DateTime _focusedDate = DateTime.now();
  DateTime? _selectedDay;
  List<String> _tags = [];

  final DateTime _firstDay =
      DateTime.now().subtract(const Duration(days: 365 * 100));
  final DateTime _lastDay = DateTime.now().add(const Duration(days: 365 * 100));

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
                  border: OutlineInputBorder(),
                  labelText: "Description",
                ),
              ),
              TableCalendar(
                firstDay: _firstDay,
                lastDay: _lastDay,
                focusedDay: _focusedDate,
                selectedDayPredicate: (day) {
                  return _selectedDay == day;
                },
                calendarFormat: _calendarFormat,
                onDaySelected: (selectedDay, focusedDay) {
                  setState(() {
                    _selectedDay = selectedDay;
                  });
                },
                onFormatChanged: (format) {
                  setState(() {
                    _calendarFormat = format;
                  });
                },
              ),
              Padding(
                padding: const EdgeInsets.symmetric(
                  vertical: 5,
                  horizontal: 10,
                ),
                child: TagsWidget(
                  tags: const [],
                  onSubmit: (tags) {
                    _tags = tags;
                  },
                ),
              ),
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

          // if (context.mounted) {
          //   context.read<TaskBloc>().add(
          //         TaskUpdateEvent(
          //           task: widget.task.copyWith(
          //             description: description.text,
          //             tags: _tags,
          //             due: _selectedDay,
          //             status: TaskStatus.pending,
          //           ),
          //         ),
          //       );
          //   Navigator.pop(context);
          // }

          // FIXME: this
          throw Exception("not implemented");
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
