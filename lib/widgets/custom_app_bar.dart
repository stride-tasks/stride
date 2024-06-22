import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/routes/routes.dart';

class CustomAppBar extends StatefulWidget implements PreferredSizeWidget {
  final String title;
  final Widget? leading;

  const CustomAppBar({
    super.key,
    this.leading,
    required this.title,
  });

  @override
  State<CustomAppBar> createState() => _CustomAppBarState();

  @override
  Size get preferredSize => const Size.fromHeight(kToolbarHeight);
}

class _CustomAppBarState extends State<CustomAppBar> {
  Future<void>? sync;

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<SettingsBloc, SettingsState>(
      builder: (context, state) {
        final hasFilter = state.settings.selectedFilter != null;
        return AppBar(
          title: Text(
            widget.title,
            style: const TextStyle(fontWeight: FontWeight.bold),
          ),
          leading: widget.leading,
          actions: [
            InkWell(
              child: Ink(
                child: hasFilter
                    ? const Icon(Icons.filter_alt_off)
                    : const Icon(Icons.filter_alt),
              ),
              onTap: () async {
                await Navigator.of(context).pushNamed(Routes.taskFilter);
              },
              onLongPress: () {
                if (!hasFilter) {
                  return;
                }
                var newSettings = state.settings.copyWith(selectedFilter: null);
                context
                    .read<SettingsBloc>()
                    .add(SettingsUpdateEvent(settings: newSettings));
                context.read<TaskBloc>().add(TaskFilterEvent(filter: null));
              },
            ),
            // IconButton(
            //   icon: hasFilter
            //       ? const Icon(Icons.filter_alt_off)
            //       : const Icon(Icons.filter_alt),
            //   onPressed: () async {
            //     if (hasFilter) {
            //       return;
            //     }
            //     await Navigator.of(context).pushNamed(Routes.taskFilter);
            //   },
            // ),
            IconButton(
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
