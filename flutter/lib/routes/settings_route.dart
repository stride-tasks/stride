import 'dart:convert';
import 'dart:io';
import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:font_awesome_flutter/font_awesome_flutter.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/bridge/api/error.dart';
import 'package:stride/bridge/api/logging.dart';
import 'package:stride/routes/commits_route.dart';
import 'package:stride/routes/encryption_key_add_route.dart';
import 'package:stride/routes/encryption_keys_route.dart';
import 'package:stride/routes/known_hosts_route.dart';
import 'package:stride/routes/logging_routes.dart';
import 'package:stride/routes/ssh_keys_route.dart';
import 'package:stride/utils/functions.dart';
import 'package:stride/widgets/settings_widget.dart';

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
      body: BlocBuilder<SettingsBloc, SettingsState>(
        builder: (context, state) {
          final settings = state.settings;
          return SettingsList(
            sections: [
              SettingsSection(
                title: Text('General', style: headingStyle),
                tiles: [
                  SettingsTileSwitch(
                    title: const Text('Theme'),
                    leading: const Icon(Icons.color_lens),
                    description: const Text('Choose the theme mode'),
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
                ],
              ),
              SettingsSection(
                title: Text('Git Integration', style: headingStyle),
                tiles: [
                  SettingsTileText(
                    title: const Text('Repository URL'),
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
                    title: const Text('Email'),
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
                    title: const Text('Author'),
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
                    title: const Text('Branch'),
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
                    title: const Text('SSH Key'),
                    builder: (context) => SshKeysRoute(
                      hasDelete: false,
                      selected: settings.repository.sshKeyUuid,
                      onTap: (key) {
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
                  SettingsTileNavigation(
                    leading: const Icon(Icons.commit),
                    title: const Text('Commits'),
                    builder: (context) => CommitsRoute(
                      repository: context.read<TaskBloc>().repository,
                    ),
                  ),
                  SettingsTileNavigation(
                    leading: const Icon(Icons.lock),
                    title: const Text('Encryption Key'),
                    builder: (context) => EncryptionKeysRoute(
                      hasDelete: false,
                      selected: settings.repository.encryptionKeyUuid,
                      onTap: (key) {
                        context.read<SettingsBloc>().add(
                              SettingsUpdateEvent(
                                settings: settings.copyWith(
                                  repository: settings.repository
                                      .copyWith(encryptionKeyUuid: key.uuid),
                                ),
                              ),
                            );
                        Navigator.of(context).pop();
                      },
                    ),
                  ),
                  SettingsTile(
                    leading: const Icon(Icons.save_alt),
                    title: const Text('Export Tasks'),
                    onTap: _exportTasks,
                  ),
                  SettingsTile(
                    leading: const Icon(Icons.file_open),
                    title: const Text('Import Tasks'),
                    onTap: _importTasks,
                  ),
                  SettingsTile(
                    leading: const Icon(Icons.delete, color: Colors.red),
                    title: const Text('Remove Repository'),
                    onTap: (context) async {
                      await showAlertDialog(
                        context: context,
                        content: const Text(
                          'Are you sure you want to delete the (local) repository?',
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
                  SettingsTile(
                    leading: const Icon(Icons.push_pin, color: Colors.red),
                    title: const Text('Force Push to Remote'),
                    onTap: (context) async {
                      await showAlertDialog(
                        context: context,
                        content: const Text(
                          'Are you sure you want to force push local branch to remote repository?',
                          style: TextStyle(fontWeight: FontWeight.bold),
                          textAlign: TextAlign.center,
                        ),
                        onConfirm: (context) async {
                          context.read<TaskBloc>().add(TaskForcePushEvent());
                          Navigator.of(context).pop();
                          return true;
                        },
                      );
                    },
                  ),
                ],
              ),
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
                  SettingsTileNavigation(
                    leading: const Icon(Icons.lock),
                    title: const Text('Encryption Keys'),
                    builder: (context) => EncryptionKeysRoute(
                      onTap: (key) {
                        Navigator.of(context).push<void>(
                          MaterialPageRoute(
                            builder: (context) => EncryptionKeyAddRoute(
                              encryptionKey: key,
                            ),
                          ),
                        );
                      },
                    ),
                  ),
                ],
              ),
              SettingsSection(
                title: Text('Misc', style: headingStyle),
                tiles: [
                  SettingsTileNavigation(
                    leading: const Icon(Icons.file_open),
                    title: const Text('Logs'),
                    builder: (context) => const LoggingRoute(),
                  ),
                  SettingsTileNavigation(
                    leading: const Icon(Icons.file_copy_outlined),
                    title: const Text('Open Source and Licence'),
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

  Future<void> _exportTasks(BuildContext context) async {
    final filepath = await FilePicker.platform.saveFile(
      dialogTitle: 'Export Tasks',
      fileName: 'tasks.json',
      allowedExtensions: const ['json'],
    );

    if (filepath == null) {
      return;
    }

    try {
      final contents = await context.read<TaskBloc>().repository.export_();

      await File(filepath).writeAsString(
        contents,
        flush: true,
      );
    } on RustError catch (error) {
      Logger.error(
        message: 'export error: ${error.toErrorString()}',
      );
    } on Exception catch (error) {
      Logger.error(message: 'export error: $error');
    }
  }

  Future<void> _importTasks(BuildContext context) async {
    try {
      final result = await FilePicker.platform.pickFiles(
        dialogTitle: 'Import tasks',
        allowedExtensions: const ['json'],
        withData: true,
      );

      if (result == null) {
        return;
      }

      if (result.files.isEmpty) {
        Logger.error(message: 'import file not selected.');
        return;
      }

      final file = result.files.first;
      // The withData flag has been passed so bytes should be available.
      final bytes = file.bytes!;
      final content = const Utf8Decoder().convert(bytes);

      context.read<TaskBloc>().repository.import_(content: content);
      context.read<TaskBloc>().repository.addAndCommit(message: r'$IMPORT');
    } on RustError catch (error) {
      Logger.error(message: 'import error: ${error.toErrorString()}');
    } on Exception catch (error) {
      Logger.error(message: 'import error: $error');
    }
  }
}
