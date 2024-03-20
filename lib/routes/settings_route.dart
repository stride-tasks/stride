import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/routes/known_hosts_route.dart';
import 'package:stride/routes/ssh_keys_route.dart';
import 'package:stride/src/rust/api/repository.dart';
import 'package:stride/widgets/custom_app_bar.dart';
import 'package:stride/widgets/settings_widget.dart';

class SettingsRoute extends StatelessWidget {
  final TextStyle headingStyle = const TextStyle(
    fontSize: 16,
    fontWeight: FontWeight.w600,
    color: Colors.red,
  );

  const SettingsRoute({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: const CustomAppBar(title: "Settings"),
      body: BlocBuilder<SettingsBloc, SettingsState>(
        builder: (context, state) {
          final settings = state.settings;
          return SettingsList(
            sections: [
              SettingsSection(
                title: Text("General", style: headingStyle),
                tiles: [
                  SettingsTileSwitch(
                    title: const Text("Theme"),
                    leading: const Icon(Icons.color_lens),
                    description: const Text("Choose the theme mode"),
                    value: true,
                    onChanged: (val) {
                      context.read<SettingsBloc>().add(
                            SettingsUpdateEvent(
                              // TODO: Add theme to Settings
                              settings: settings.copyWith(),
                            ),
                          );
                    },
                  ),
                ],
              ),
              SettingsSection(
                title: Text("Git Integration", style: headingStyle),
                tiles: [
                  SettingsTileText(
                    title: const Text("Repository URL"),
                    leading: const Icon(Icons.code),
                    text: settings.repository.origin,
                    onChanged: (text) {
                      context.read<SettingsBloc>().add(
                            SettingsUpdateEvent(
                              settings: settings.copyWith(
                                repository:
                                    settings.repository.copyWith(origin: text),
                              ),
                            ),
                          );
                    },
                  ),
                  SettingsTileText(
                    leading: const Icon(Icons.mail),
                    title: const Text("Email"),
                    text: settings.repository.email,
                    onChanged: (text) {
                      context.read<SettingsBloc>().add(
                            SettingsUpdateEvent(
                              settings: settings.copyWith(
                                repository:
                                    settings.repository.copyWith(email: text),
                              ),
                            ),
                          );
                    },
                  ),
                  SettingsTileText(
                    leading: const Icon(Icons.person),
                    title: const Text("Author"),
                    text: settings.repository.author,
                    onChanged: (text) {
                      context.read<SettingsBloc>().add(
                            SettingsUpdateEvent(
                              settings: settings.copyWith(
                                repository:
                                    settings.repository.copyWith(author: text),
                              ),
                            ),
                          );
                    },
                  ),
                  SettingsTileNavigation(
                    leading: const Icon(Icons.key),
                    title: const Text("SSH Key"),
                    builder: (context) => SshKeysRoute(
                      hasDelete: false,
                      onTap: (key) {
                        // print(key);
                        context.read<SettingsBloc>().add(
                              SettingsUpdateEvent(
                                settings: settings.copyWith(
                                  repository: settings.repository
                                      .copyWith(sshKeyUuid: key.uuid),
                                ),
                              ),
                            );
                        Navigator.of(context).pop();
                      },
                    ),
                  ),
                  SettingsTile(
                    leading: const Icon(Icons.delete),
                    title: const Text("Remove Repository"),
                    onTap: (context) async {
                      await removeRepository();
                      if (context.mounted) {
                        context.read<TaskBloc>().add(TaskFetchEvent());
                      }
                    },
                  ),
                ],
              ),
              SettingsSection(
                title: Text("Security", style: headingStyle),
                tiles: [
                  SettingsTileNavigation(
                    leading: const Icon(Icons.key_sharp),
                    title: const Text("SSH Keys"),
                    builder: (context) => SshKeysRoute(
                      onTap: (key) async {
                        await Clipboard.setData(
                          ClipboardData(
                            text: key.public,
                          ),
                        );
                        if (context.mounted) {
                          ScaffoldMessenger.of(context).showSnackBar(
                            const SnackBar(
                              content: Text(
                                "Copied to clipbard!",
                              ),
                            ),
                          );
                        }
                      },
                    ),
                  ),
                  SettingsTileNavigation(
                    leading: const Icon(Icons.key_sharp),
                    title: const Text("SSH Known Hosts"),
                    builder: (context) => const KnownHostsRoute(),
                  ),
                ],
              ),
              SettingsSection(
                title: Text("Misc", style: headingStyle),
                tiles: [
                  SettingsTileNavigation(
                    leading: const Icon(Icons.file_copy_outlined),
                    title: const Text("Open Source and Licence"),
                    builder: (context) => const SshKeysRoute(),
                  ),
                ],
              ),
            ],
          );
        },
      ),
    );
  }
}
