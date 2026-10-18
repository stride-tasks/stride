import 'package:flutter/material.dart';
import 'package:stride/background.dart';
import 'package:stride/context.dart';
import 'package:stride/controllers/task_controller.dart';
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
              ValueListenableBuilder(
                valueListenable: context.taskController.state,
                builder: (context, taskState, _) {
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
                        ? () => context.taskController.filter(null)
                        : null,
                  );
                },
              ),
            if (hasRepositories)
              IconButton(
                icon: StreamBuilder<Set<Output>>(
                  stream: Background.stream(),
                  builder: (context, snapshot) {
                    final output = snapshot.data?.lookup(
                      Output(
                        name: Name(
                          method: 'repository.sync',
                          unique: context.taskController.repositoryUuid?.uuid,
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
                      context.taskController.fetch();
                      return Icon(Icons.sync);
                    } else {
                      return const InfiniteRotationAnimation(
                        child: Icon(Icons.sync),
                      );
                    }
                  },
                ),
                onPressed: () => context.taskController.sync(),
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
