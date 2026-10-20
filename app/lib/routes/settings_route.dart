import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:stride/background.dart';
import 'package:stride/context.dart';
import 'package:stride/logging.dart';
import 'package:stride/routes/known_hosts_route.dart';
import 'package:stride/routes/logging_routes.dart';
import 'package:stride/routes/plugin_list_route.dart';
import 'package:stride/routes/repository_list_route.dart';
import 'package:stride/routes/ssh_keys_route.dart';
import 'package:stride/utils/functions.dart';
import 'package:stride/widgets/settings_widget.dart';
import 'package:url_launcher/url_launcher_string.dart';

class SettingsRoute extends StatelessWidget {
  TextStyle get headingStyle => const TextStyle(
    fontSize: 16,
    fontWeight: FontWeight.w600,
    color: Colors.red,
  );

  const SettingsRoute({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Settings')),
      body: ValueListenableBuilder(
        valueListenable: RustContext.setting,
        builder: (context, setting, _) {
          return SettingsList(
            sections: [
              SettingsSection(
                title: Text('General', style: headingStyle),
                tiles: [
                  SettingsTileSwitch(
                    title: const Text('Dark Theme'),
                    leading: const Icon(Icons.color_lens),
                    description: const Text('Enable dark theme mode'),
                    value: setting.darkMode,
                    onChanged: (val) {
                      RustContext.setting.value = setting.copyWith(
                        darkMode: val,
                      );
                    },
                  ),
                  SettingsTileSwitch(
                    title: const Text('Periodic Sync'),
                    leading: const Icon(Icons.timer),
                    description: Text(
                      'Sync every ${setting.periodicSyncInterval} seconds. '
                      'Hold to change (minimum 5 seconds).',
                    ),
                    value: setting.periodicSync,
                    onChanged: (val) {
                      RustContext.setting.value = setting.copyWith(
                        periodicSync: val,
                      );
                    },
                    onLongPress: (context) => _editSyncInterval(
                      context,
                      setting.periodicSyncInterval,
                    ),
                  ),
                  SettingsTileNavigation(
                    leading: const Icon(Icons.file_open),
                    title: const Text('Logs'),
                    builder: (context) => const LoggingRoute(),
                  ),
                  SettingsTileNavigation(
                    leading: const Icon(Icons.storage),
                    title: const Text('Repositories'),
                    builder: (context) => const RepositoryListRoute(),
                  ),
                  SettingsTileNavigation(
                    leading: const Icon(Icons.electrical_services),
                    title: const Text('Plugins'),
                    builder: (context) => const PluginListRoute(),
                  ),
                ],
              ),
              //     SettingsTileNavigation(
              //       leading: const Icon(Icons.commit),
              //       title: const Text('Commits'),
              //       builder: (context) => CommitsRoute(
              //         repository: context.read<TaskBloc>().repository,
              //       ),
              //     ),
              //     SettingsTile(
              //       leading: const Icon(Icons.save_alt),
              //       title: const Text('Export Tasks'),
              //       onTap: _exportTasks,
              //     ),
              //     SettingsTile(
              //       leading: const Icon(Icons.file_open),
              //       title: const Text('Import Tasks'),
              //       onTap: _importTasks,
              //     ),
              //     SettingsTile(
              //       leading: const Icon(Icons.delete, color: Colors.red),
              //       title: const Text('Remove Repository'),
              //       onTap: (context) async {
              //         await showAlertDialog(
              //           context: context,
              //           content: const Text(
              //             'Are you sure you want to delete the (local) repository?',
              //             style: TextStyle(fontWeight: FontWeight.bold),
              //             textAlign: TextAlign.center,
              //           ),
              //           onConfirm: (context) async {
              //             context.read<TaskBloc>().add(TaskRemoveAllEvent());
              //             Navigator.of(context).pop();
              //             return true;
              //           },
              //         );
              //       },
              //     ),
              SettingsSection(
                title: Text('Security', style: headingStyle),
                tiles: [
                  SettingsTileNavigation(
                    leading: const Icon(Icons.key_sharp),
                    title: const Text('SSH Keys'),
                    builder: (context) => SshKeysRoute(
                      onTap: (key) async {
                        await Clipboard.setData(
                          ClipboardData(text: key.publicKey),
                        );
                        if (context.mounted) {
                          ScaffoldMessenger.of(context).showSnackBar(
                            const SnackBar(
                              content: Text('Copied to clipbard!'),
                            ),
                          );
                        }
                      },
                    ),
                  ),
                  SettingsTileNavigation(
                    leading: const Icon(Icons.key_sharp),
                    title: const Text('SSH Known Hosts'),
                    builder: (context) => const KnownHostsRoute(),
                  ),
                ],
              ),
              SettingsSection(
                title: Text('Project', style: headingStyle),
                tiles: [
                  const SettingsTile(
                    leading: Icon(Icons.pin),
                    title: Text('Version'),
                    description: Text(
                      // NOTE: Change when pubspec.yaml version is changed.
                      // maybe use package_info_plus to get the version.
                      '0.0.1+1',
                      overflow: TextOverflow.ellipsis,
                      style: TextStyle(fontSize: 12.0, color: Colors.grey),
                    ),
                  ),
                  // TODO: Deduplicate string by using the SettingsTileText
                  //       with null onChanged callback.
                  SettingsTile(
                    leading: const Icon(Icons.bug_report),
                    description: const Text(
                      'https://github.com/stride-tasks/stride/issues',
                      overflow: TextOverflow.ellipsis,
                      style: TextStyle(fontSize: 12.0, color: Colors.grey),
                    ),
                    title: const Text('Issue Tracker'),
                    onTap: (context) async {
                      const url =
                          'https://github.com/stride-tasks/stride/issues';
                      await Log.catch_(
                        () async => launchUrlString(url),
                      );
                    },
                  ),
                  SettingsTile(
                    leading: const Icon(Icons.code),
                    title: const Text('Source Code'),
                    description: const Text(
                      'https://github.com/stride-tasks/stride',
                      overflow: TextOverflow.ellipsis,
                      style: TextStyle(fontSize: 12.0, color: Colors.grey),
                    ),
                    onTap: (context) async {
                      const url = 'https://github.com/stride-tasks/stride';
                      await Log.catch_(
                        () async => launchUrlString(url),
                      );
                    },
                  ),
                  SettingsTile(
                    leading: const Icon(Icons.copyright),
                    title: const Text('License'),
                    description: const Text(
                      'AGPL-3.0-or-later',
                      overflow: TextOverflow.ellipsis,
                      style: TextStyle(fontSize: 12.0, color: Colors.grey),
                    ),
                    onTap: (context) async {
                      const url =
                          'https://www.gnu.org/licenses/agpl-3.0.en.html';
                      await Log.catch_(
                        () async => launchUrlString(url),
                      );
                    },
                  ),
                ],
              ),
            ],
          );
        },
      ),
    );
  }

  Future<void> _editSyncInterval(
    BuildContext context,
    int currentIntervalSeconds,
  ) async {
    final controller = TextEditingController(
      text: currentIntervalSeconds.toString(),
    );

    try {
      await showAlertDialog<bool>(
        context: context,
        popRoute: false,
        content: StatefulBuilder(
          builder: (context, setState) => TextFormField(
            controller: controller,
            autofocus: true,
            keyboardType: TextInputType.number,
            inputFormatters: [FilteringTextInputFormatter.digitsOnly],
            decoration: InputDecoration(
              labelText: 'Sync interval in seconds',
              errorText: _syncIntervalError(controller.text),
            ),
            onChanged: (_) => setState(() {}),
          ),
        ),
        onConfirm: (dialogContext) async {
          final seconds = int.tryParse(controller.text);
          if (seconds == null ||
              seconds < Background.minimumSyncIntervalSeconds) {
            return false;
          }

          Background.setSyncIntervalSeconds(seconds);
          Navigator.of(dialogContext).pop(true);
          return true;
        },
      );
    } finally {
      controller.dispose();
    }
  }

  String? _syncIntervalError(String value) {
    final seconds = int.tryParse(value);
    if (seconds == null) return 'Enter a valid number of seconds.';
    if (seconds < Background.minimumSyncIntervalSeconds) {
      return 'Minimum interval is 5 seconds.';
    }
    return null;
  }
}
