import 'package:feedback/feedback.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/routes/known_hosts_route.dart';
import 'package:stride/routes/logging_routes.dart';
import 'package:stride/routes/plugin_list_route.dart';
import 'package:stride/routes/repository_list_route.dart';
import 'package:stride/routes/ssh_keys_route.dart';
import 'package:stride/widgets/settings_widget.dart';
import 'package:url_launcher/url_launcher_string.dart';

/// Shows an [AlertDialog] with the given feedback.
/// This is useful for debugging purposes.
void alertFeedbackFunction(
  BuildContext outerContext,
  UserFeedback feedback,
) {
  showDialog<void>(
    context: outerContext,
    builder: (context) {
      return AlertDialog(
        title: Text(feedback.text),
        content: SingleChildScrollView(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              if (feedback.extra != null) Text(feedback.extra!.toString()),
              Image.memory(
                feedback.screenshot,
                height: 600,
                width: 500,
                fit: BoxFit.contain,
              ),
            ],
          ),
        ),
        actions: <Widget>[
          TextButton(
            child: const Text('Close'),
            onPressed: () {
              Navigator.pop(context);
            },
          )
        ],
      );
    },
  );
}

class SettingsRoute extends StatelessWidget {
  TextStyle get headingStyle => const TextStyle(
        fontSize: 16,
        fontWeight: FontWeight.w600,
        color: Colors.red,
      );

  const SettingsRoute({super.key});

  @override
  Widget build(BuildContext context_) {
    return Scaffold(
      appBar: AppBar(title: const Text('Settings')),
      body: BlocBuilder<SettingsBloc, SettingsState>(
        builder: (context, state) {
          final settings = state.settings;
          return SettingsList(
            sections: [
              SettingsSection(
                title: Text('General', style: headingStyle),
                tiles: [
                  SettingsTileSwitch(
                    title: const Text('Dark Theme'),
                    leading: const Icon(Icons.color_lens),
                    description: const Text('Enable dark theme mode'),
                    value: settings.darkMode,
                    onChanged: (val) =>
                        context.read<SettingsBloc>().add(SettingsToggleTheme()),
                  ),
                  SettingsTileSwitch(
                    title: const Text('Periodic Sync'),
                    leading: const Icon(Icons.timer),
                    description:
                        const Text('Periodically sync every 5 minutes'),
                    value: settings.periodicSync,
                    onChanged: (val) => context.read<SettingsBloc>().add(
                          SettingsUpdateEvent(
                            settings: settings.copyWith(periodicSync: val),
                          ),
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
                  SettingsTileSwitch(
                    title: Text("FeedBack"),
                    value: false,
                    onChanged: (value) {
                      BetterFeedback.of(context_).show(
                        (feedback) async {
                          // upload to server, share whatever
                          // for example purposes just show it to the user
                          alertFeedbackFunction(
                            context,
                            feedback,
                          );
                        },
                      );
                    },
                  )
                ],
              ),
              // SettingsSection(
              //   title: Text('Git Integration', style: headingStyle),
              //   tiles: [
              //     SettingsTileText(
              //       title: const Text('Repository URL'),
              //       leading: const Icon(Icons.code),
              //       text: settings.repositories[0].origin,
              //       onChanged: (text) {
              //         context.read<SettingsBloc>().add(
              //               SettingsUpdateEvent(
              //                 settings: settings.copyWith(
              //                   repository:
              //                       settings.repository.copyWith(origin: text),
              //                 ),
              //               ),
              //             );
              //       },
              //     ),
              //     SettingsTileText(
              //       leading: const Icon(Icons.mail),
              //       title: const Text('Email'),
              //       text: settings.repository.email,
              //       onChanged: (text) {
              //         context.read<SettingsBloc>().add(
              //               SettingsUpdateEvent(
              //                 settings: settings.copyWith(
              //                   repository:
              //                       settings.repository.copyWith(email: text),
              //                 ),
              //               ),
              //             );
              //       },
              //     ),
              //     SettingsTileText(
              //       leading: const Icon(Icons.person),
              //       title: const Text('Author'),
              //       text: settings.repository.author,
              //       onChanged: (text) {
              //         context.read<SettingsBloc>().add(
              //               SettingsUpdateEvent(
              //                 settings: settings.copyWith(
              //                   repository:
              //                       settings.repository.copyWith(author: text),
              //                 ),
              //               ),
              //             );
              //       },
              //     ),
              //     SettingsTileText(
              //       leading: const FaIcon(FontAwesomeIcons.codeBranch),
              //       title: const Text('Branch'),
              //       text: settings.repository.branch,
              //       onChanged: (text) {
              //         context.read<SettingsBloc>().add(
              //               SettingsUpdateEvent(
              //                 settings: settings.copyWith(
              //                   repository:
              //                       settings.repository.copyWith(branch: text),
              //                 ),
              //               ),
              //             );
              //         context.read<TaskBloc>().add(TaskCheckoutBranchEvent());
              //       },
              //     ),
              //     SettingsTileNavigation(
              //       leading: const Icon(Icons.key),
              //       title: const Text('SSH Key'),
              //       builder: (context) => SshKeysRoute(
              //         hasDelete: false,
              //         selected: settings.repository.sshKeyUuid,
              //         onTap: (key) {
              //           context.read<SettingsBloc>().add(
              //                 SettingsUpdateEvent(
              //                   settings: settings.copyWith(
              //                     repository: settings.repository
              //                         .copyWith(sshKeyUuid: key.uuid),
              //                   ),
              //                 ),
              //               );
              //           Navigator.of(context).pop();
              //         },
              //       ),
              //     ),
              //     SettingsTileNavigation(
              //       leading: const Icon(Icons.commit),
              //       title: const Text('Commits'),
              //       builder: (context) => CommitsRoute(
              //         repository: context.read<TaskBloc>().repository,
              //       ),
              //     ),
              //     SettingsTileNavigation(
              //       leading: const Icon(Icons.lock),
              //       title: const Text('Encryption'),
              //       builder: (context) => EncryptionKeyRoute(
              //         encryption: settings.repository.encryption,
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
              //     SettingsTile(
              //       leading: const Icon(Icons.push_pin, color: Colors.red),
              //       title: const Text('Force Push to Remote'),
              //       onTap: (context) async {
              //         await showAlertDialog(
              //           context: context,
              //           content: const Text(
              //             'Are you sure you want to force push local branch to remote repository?',
              //             style: TextStyle(fontWeight: FontWeight.bold),
              //             textAlign: TextAlign.center,
              //           ),
              //           onConfirm: (context) async {
              //             context.read<TaskBloc>().add(TaskForcePushEvent());
              //             Navigator.of(context).pop();
              //             return true;
              //           },
              //         );
              //       },
              //     ),
              //   ],
              // ),
              SettingsSection(
                title: Text('Security', style: headingStyle),
                tiles: [
                  SettingsTileNavigation(
                    leading: const Icon(Icons.key_sharp),
                    title: const Text('SSH Keys'),
                    builder: (context) => SshKeysRoute(
                      onTap: (key) async {
                        await Clipboard.setData(
                          ClipboardData(
                            text: key.publicKey,
                          ),
                        );
                        if (context.mounted) {
                          ScaffoldMessenger.of(context).showSnackBar(
                            const SnackBar(
                              content: Text(
                                'Copied to clipbard!',
                              ),
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
                      style: TextStyle(
                        fontSize: 12.0,
                        color: Colors.grey,
                      ),
                    ),
                  ),
                  // TODO: Deduplicate string by using the SettingsTileText
                  //       with null onChanged callback.
                  SettingsTile(
                    leading: const Icon(Icons.bug_report),
                    description: const Text(
                      'https://github.com/stride-tasks/stride/issues',
                      overflow: TextOverflow.ellipsis,
                      style: TextStyle(
                        fontSize: 12.0,
                        color: Colors.grey,
                      ),
                    ),
                    title: const Text('Issue Tracker'),
                    onTap: (context) async {
                      const url =
                          'https://github.com/stride-tasks/stride/issues';
                      await context
                          .read<LogBloc>()
                          .catch_(() async => launchUrlString(url));
                    },
                  ),
                  SettingsTile(
                    leading: const Icon(Icons.code),
                    title: const Text('Source Code'),
                    description: const Text(
                      'https://github.com/stride-tasks/stride',
                      overflow: TextOverflow.ellipsis,
                      style: TextStyle(
                        fontSize: 12.0,
                        color: Colors.grey,
                      ),
                    ),
                    onTap: (context) async {
                      const url = 'https://github.com/stride-tasks/stride';
                      await context
                          .read<LogBloc>()
                          .catch_(() async => launchUrlString(url));
                    },
                  ),
                  SettingsTile(
                    leading: const Icon(Icons.copyright),
                    title: const Text('License'),
                    description: const Text(
                      'AGPL-3.0-or-later',
                      overflow: TextOverflow.ellipsis,
                      style: TextStyle(
                        fontSize: 12.0,
                        color: Colors.grey,
                      ),
                    ),
                    onTap: (context) async {
                      const url =
                          'https://www.gnu.org/licenses/agpl-3.0.en.html';
                      await context
                          .read<LogBloc>()
                          .catch_(() async => launchUrlString(url));
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
}
