import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/api.dart';
import 'package:stride/background.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/context.dart';
import 'package:stride/routes/settings_route.dart';
import 'package:stride/routes/task_filter_route.dart';
import 'package:stride/widgets/infinite_rotation_animation.dart';

class CustomAppBar extends StatelessWidget implements PreferredSizeWidget {
  final String title;
  final Widget? leading;

  const CustomAppBar({super.key, this.leading, required this.title});

  @override
  Widget build(BuildContext context) {
    return ValueListenableBuilder(
      valueListenable: RustContext.repositories,
      builder: (context, repositories, child) {
        final hasRepositories = repositories.isNotEmpty;
        return AppBar(
          title: Text(title),
          leading: leading,
          titleSpacing: NavigationToolbar.kMiddleSpacing / 2.0,
          actions: [
            if (hasRepositories)
              BlocBuilder<TaskBloc, TaskState>(
                builder: (context, taskState) {
                  final hasFilter = taskState.query != null;
                  return InkWell(
                    child: Ink(
                      child: Icon(
                        hasFilter ? Icons.filter_alt : Icons.filter_alt_off,
                      ),
                    ),
                    onTap: () async {
                      await Navigator.of(context).push<void>(
                        MaterialPageRoute(
                          builder: (context) => const TaskFilterRoute(),
                        ),
                      );
                    },
                    onLongPress: hasFilter
                        ? () => context.read<TaskBloc>().add(TaskFilterEvent())
                        : null,
                  );
                },
              ),
            // TODO: Implemnt undo
            // if (hasRepositories)
            //   IconButton(
            //     icon: const Icon(Icons.undo),
            //     onPressed: () async {
            //       context.read<LogBloc>().catch_(
            //             message: 'undo',
            //             () async => context.read<TaskBloc>().undo(),
            //           );
            //     },
            //   ),
            if (hasRepositories)
              IconButton(
                icon: StreamBuilder<Set<Output>>(
                  stream: Background.stream(),
                  builder: (context, snapshot) {
                    final output = snapshot.data?.lookup(
                      Output(
                        name: Name(
                          method: 'repository.sync',
                          unique: context.read<TaskBloc>().repositoryUuid?.uuid,
                        ),
                        inputData: {},
                        timestamp: DateTime.now(),
                      ),
                    );

                    if (output == null) {
                      return Icon(Icons.sync);
                    } else if (output.error != null) {
                      return Icon(Icons.sync_problem);
                    } else if (output.done) {
                      context.read<TaskBloc>().add(TaskFetchEvent());
                      return Icon(Icons.sync);
                    } else {
                      return const InfiniteRotationAnimation(
                        child: Icon(Icons.sync),
                      );
                    }
                  },
                ),
                onPressed: () => context.read<TaskBloc>().add(TaskSyncEvent()),
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
