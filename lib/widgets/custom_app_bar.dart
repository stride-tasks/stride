import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/routes/routes.dart';
import 'package:stride/src/rust/api/repository.dart';

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
              icon: FutureBuilder(
                future: syncTasksFuture,
                builder: (context, snapshot) {
                  if (snapshot.connectionState == ConnectionState.done) {
                    return const Icon(Icons.sync);
                  }
                  return const CircularProgressIndicator();
                },
              ),
              onPressed: () {
                setState(() {
                  syncTasksFuture = syncTasks();
                });
                if (context.mounted) {
                  context.read<TaskBloc>().add(TaskFetchEvent());
                }
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
