import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/routes/routes.dart';
import 'package:stride/src/rust/api/repository.dart';
import 'package:stride/src/rust/git/known_hosts.dart';

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
        final settings = state.settings;
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
                  print(snapshot);
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
              icon: FutureBuilder(
                future: sync,
                builder: (context, snapshot) {
                  print(snapshot);
                  if (snapshot.hasError && !seenError) {
                    if (snapshot.error! is ConnectionError_UnknownHost) {
                      final error =
                          snapshot.error! as ConnectionError_UnknownHost;

                      final snackBar = SnackBar(
                        content: Text('Unknown host ${error.hostname}'),
                        action: SnackBarAction(
                          label: "Add",
                          onPressed: () {
                            context.read<SettingsBloc>().add(
                                  SettingsAddKnownHostEvent(
                                    host: Host(
                                      hostname: error.hostname,
                                      remoteKeyType: error.keyType,
                                      remoteHostKey: error.hostKey,
                                    ),
                                  ),
                                );
                          },
                        ),
                        dismissDirection: DismissDirection.endToStart,
                        duration: const Duration(days: 1),
                      );

                      seenError = true;

                      WidgetsBinding.instance.addPostFrameCallback((_) {
                        ScaffoldMessenger.of(context)
                          ..hideCurrentSnackBar()
                          ..showSnackBar(snackBar);
                      });
                    }

                    return const Icon(Icons.perm_scan_wifi_rounded);
                  }
                  switch (snapshot.connectionState) {
                    case ConnectionState.none:
                      return const Icon(Icons.wifi_rounded);
                    case ConnectionState.waiting:
                    case ConnectionState.active:
                      return const CircularProgressIndicator();
                    case ConnectionState.done:
                      return const Icon(Icons.check);
                  }
                },
              ),
              onPressed: () async {
                if (settings.repository.sshKeyUuid == null) {
                  return await showDialog(
                    context: context,
                    builder: (context) {
                      return AlertDialog(
                        content: const Text(
                          "The repository does not have an assigned SSH key",
                        ),
                        actions: [
                          IconButton(
                            icon: const Icon(Icons.cancel),
                            onPressed: () {
                              Navigator.of(context).pop();
                            },
                          ),
                        ],
                      );
                    },
                  );
                }
                final sshKey = state.settings.keys.firstWhere(
                  (element) => element.uuid == settings.repository.sshKeyUuid,
                );

                setState(() {
                  seenError = false;
                  sync = testConnection(
                    url: settings.repository.origin,
                    publicKey: sshKey.public,
                    privateKey: sshKey.private,
                  );
                });
              },
              color: Theme.of(context).secondaryHeaderColor,
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
