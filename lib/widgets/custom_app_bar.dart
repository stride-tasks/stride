import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/routes/routes.dart';

class CustomAppBar extends StatefulWidget implements PreferredSizeWidget {
  final String title;

  const CustomAppBar({
    super.key,
    required this.title,
  });

  @override
  State<CustomAppBar> createState() => _CustomAppBarState();

  @override
  Size get preferredSize => const Size.fromHeight(kToolbarHeight);
}

class _CustomAppBarState extends State<CustomAppBar> {
  Future<void>? sync;
  bool seenError = false;

  Future<void> syncTasksFuture = Future.value();

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<SettingsBloc, SettingsState>(
      builder: (context, state) {
        return AppBar(
          title: Text(
            widget.title,
            style: const TextStyle(fontWeight: FontWeight.bold),
          ),
          actions: [
            IconButton(
              color: Theme.of(context).secondaryHeaderColor,
              icon: BlocBuilder<TaskBloc, TaskState>(
                builder: (context, state) {
                  if (state.syncing) {
                    return const CircularProgressIndicator();
                  }
                  return const Icon(Icons.sync);
                },
              ),
              onPressed: () {
                context.read<TaskBloc>().add(TaskSyncEvent());
              },
            ),
            IconButton(
              icon: const Icon(Icons.settings),
              onPressed: () {
                Navigator.pushNamed(context, Routes.settings);
              },
            ),
          ],
        );
      },
    );
  }
}
