import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/routes/routes.dart';
import 'package:stride/widgets/custom_app_bar.dart';
import 'package:stride/widgets/task_item_widget.dart';

class TasksRoute extends StatefulWidget {
  const TasksRoute({super.key});

  @override
  State<TasksRoute> createState() => _TasksRouteState();
}

class _TasksRouteState extends State<TasksRoute> {
  @override
  void initState() {
    context.read<TaskBloc>().add(TaskFetchEvent());
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: const CustomAppBar(title: "Task List"),
      body: BlocBuilder<TaskBloc, TaskState>(
        builder: (context, state) => Padding(
          padding: const EdgeInsets.symmetric(vertical: 20.0, horizontal: 10.0),
          child: Column(
            children: [
              TextField(
                autofocus: true,
                decoration: const InputDecoration(
                  border: OutlineInputBorder(),
                  labelText: "Search",
                ),
                onChanged: (text) {
                  context.read<TaskBloc>().add(TaskSearchEvent(text: text));
                },
              ),
              const SizedBox(height: 10.0),
              ElevatedButton(
                onPressed: () {
                  context.read<TaskBloc>().add(TaskLoadDeletedEvent());
                },
                child: const Icon(Icons.switch_access_shortcut),
              ),
              const SizedBox(height: 10.0),
              Expanded(
                child: ListView.builder(
                  itemCount: state.tasks.length + 1,
                  itemBuilder: (context, index) {
                    if (index == state.tasks.length) {
                      return const SizedBox(height: 50);
                    }

                    return Card(
                      child: TaskItemWidget(task: state.tasks[index]),
                    );
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
          await Navigator.of(context).pushNamed(Routes.taskAdd);
        },
        child: const Icon(Icons.add_circle, size: 50),
      ),
    );
  }
}
