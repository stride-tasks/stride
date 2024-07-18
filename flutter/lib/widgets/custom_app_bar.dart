import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/routes/settings_route.dart';
import 'package:stride/routes/task_filter_route.dart';
import 'package:stride/src/rust/api/repository.dart';
import 'package:stride/src/rust/git/known_hosts.dart';
import 'package:stride/utils/functions.dart';
import 'package:stride/widgets/infinite_rotation_animation.dart';

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
          title: Text(widget.title),
          leading: widget.leading,
          actions: [
            BlocListener<TaskBloc, TaskState>(
              listenWhen: (previous, current) =>
                  current.error is ConnectionError_UnknownHost,
              listener: (context, taskState) async {
                final ConnectionError_UnknownHost(
                  :hostname,
                  :keyType,
                  :hostKey
                ) = taskState.error! as ConnectionError_UnknownHost;
                await showAlertDialog(
                  context: context,
                  content: Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Text(
                        'Accept Unknown Host: $hostname',
                        style: const TextStyle(fontWeight: FontWeight.bold),
                      ),
                      Text(
                        'Host Key: ${keyType.name} $hostKey',
                        softWrap: true,
                      ),
                    ],
                  ),
                  onConfirm: (context) async {
                    context.read<SettingsBloc>().add(
                          SettingsUpdateEvent(
                            settings: state.settings.copyWith(
                              knownHosts: state.settings.knownHosts.copyWith(
                                hosts: state.settings.knownHosts.hosts.toList()
                                  ..add(
                                    Host(
                                      hostname: hostname,
                                      remoteKeyType: keyType,
                                      remoteHostKey: hostKey,
                                    ),
                                  ),
                              ),
                            ),
                          ),
                        );
                    Navigator.pop(context);
                    return true;
                  },
                );
              },
              child: InkWell(
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
            ),
            IconButton(
              icon: BlocBuilder<TaskBloc, TaskState>(
                builder: (context, state) {
                  if (state.syncing) {
                    return const InfiniteRotationAnimation(
                      child: Icon(Icons.sync),
                    );
                  }
                  return state.error == null
                      ? const Icon(Icons.sync)
                      : const Icon(Icons.sync_problem);
                },
              ),
              onPressed: () {
                context.read<TaskBloc>().add(TaskSyncEvent());
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
}
