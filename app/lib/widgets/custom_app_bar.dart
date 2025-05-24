import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/blocs/plugin_manager_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/bridge/third_party/stride_core/event.dart';
import 'package:stride/routes/settings_route.dart';
import 'package:stride/routes/task_filter_route.dart';
import 'package:stride/widgets/infinite_rotation_animation.dart';

class CustomAppBar extends StatelessWidget implements PreferredSizeWidget {
  final String title;
  final Widget? leading;

  const CustomAppBar({
    super.key,
    this.leading,
    required this.title,
  });

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<SettingsBloc, SettingsState>(
      builder: (context, state) {
        final hasRepositories = state.settings.repositories.isNotEmpty;
        final hasFilter = state.settings.selectedFilter != null;
        return AppBar(
          title: Text(title),
          leading: leading,
          titleSpacing: NavigationToolbar.kMiddleSpacing / 2.0,
          actions: [
            if (hasRepositories)
              InkWell(
                child: Ink(
                  child: hasFilter
                      ? const Icon(Icons.filter_alt)
                      : const Icon(Icons.filter_alt_off),
                ),
                onTap: () async {
                  await Navigator.of(context).push<void>(
                    MaterialPageRoute(
                      builder: (context) => const TaskFilterRoute(),
                    ),
                  );
                },
                onLongPress: () {
                  if (!hasFilter) {
                    return;
                  }
                  final newSettings =
                      state.settings.copyWith(selectedFilter: null);
                  context
                      .read<SettingsBloc>()
                      .add(SettingsUpdateEvent(settings: newSettings));
                  context.read<TaskBloc>().add(TaskFilterEvent());
                },
              ),
            if (hasRepositories)
              IconButton(
                icon: const Icon(Icons.undo),
                onPressed: () async {
                  context.read<LogBloc>().catch_(
                        message: 'undo',
                        () async => context.read<TaskBloc>().undo(),
                      );
                },
              ),
            if (hasRepositories)
              IconButton(
                icon: BlocBuilder<TaskBloc, TaskState>(
                  builder: (context, state) {
                    if (state.syncing) {
                      return const InfiniteRotationAnimation(
                        child: Icon(Icons.sync),
                      );
                    }
                    return state.syncingError == null
                        ? const Icon(Icons.sync)
                        : const Icon(Icons.sync_problem);
                  },
                ),
                onPressed: () {
                  context.read<TaskBloc>().add(TaskSyncEvent());
                  context
                      .read<PluginManagerBloc>()
                      .emitHostEvent(HostEvent.taskSync());
                },
              ),
            IconButton(
              icon: const Icon(Icons.settings),
              onPressed: () {
                Navigator.push<void>(
                  context,
                  MaterialPageRoute(
                    builder: (context) => const SettingsRoute(),
                  ),
                );
              },
            ),
          ],
        );
      },
    );
  }

  @override
  Size get preferredSize => const Size.fromHeight(kToolbarHeight);
}
