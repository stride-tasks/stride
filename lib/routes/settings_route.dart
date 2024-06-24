import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:font_awesome_flutter/font_awesome_flutter.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/routes/known_hosts_route.dart';
import 'package:stride/routes/logging_routes.dart';
import 'package:stride/routes/ssh_keys_route.dart';
import 'package:stride/utils/functions.dart';
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
      appBar: AppBar(title: const Text("Settings")),
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
                    value: settings.darkMode,
                    onChanged: (val) =>
                        context.read<SettingsBloc>().add(SettingsToggleTheme()),
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
                  SettingsTileText(
                    leading: const FaIcon(FontAwesomeIcons.codeBranch),
                    title: const Text("Branch"),
                    text: settings.repository.branch,
                    onChanged: (text) {
                      context.read<SettingsBloc>().add(
                            SettingsUpdateEvent(
                              settings: settings.copyWith(
                                repository:
                                    settings.repository.copyWith(branch: text),
                              ),
                            ),
                          );
                      context.read<TaskBloc>().add(TaskCheckoutBranchEvent());
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
                      await showAlertDialog(
                        context: context,
                        content: const Text(
                          "Are you sure you want to delete the (local) repository?",
                          style: TextStyle(fontWeight: FontWeight.bold),
                          textAlign: TextAlign.center,
                        ),
                        onConfirm: (context) async {
                          context.read<TaskBloc>().add(TaskRemoveAllEvent());
                          Navigator.of(context).pop();
                          return true;
                        },
                      );
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
                    leading: const Icon(Icons.file_open),
                    title: const Text("Logs"),
                    builder: (context) => const LoggingRoute(),
                  ),
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
