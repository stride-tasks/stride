import 'dart:convert';
import 'dart:io';
import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:font_awesome_flutter/font_awesome_flutter.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/bridge/api/logging.dart';
import 'package:stride/bridge/api/repository/git.dart';
import 'package:stride/bridge/api/settings.dart';
import 'package:stride/routes/commits_route.dart';
import 'package:stride/routes/encryption_key_route.dart';
import 'package:stride/routes/ssh_keys_route.dart';
import 'package:stride/utils/functions.dart';
import 'package:stride/widgets/settings_widget.dart';

class RepositoryRoute extends StatelessWidget {
  final Repository repository;
  TextStyle get headingStyle => const TextStyle(
        fontSize: 16,
        fontWeight: FontWeight.w600,
        color: Colors.red,
      );

  const RepositoryRoute({super.key, required this.repository});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Repository')),
      body: BlocBuilder<SettingsBloc, SettingsState>(
        builder: (context, state) {
          final settings = state.settings;
          return SettingsList(
            sections: [
              SettingsSection(
                title: Text('Git Integration', style: headingStyle),
                tiles: [
                  SettingsTileText(
                    title: const Text('Repository URL'),
                    leading: const Icon(Icons.code),
                    text: settings.repositories[0].origin,
                    onChanged: (text) {
                      final repositories = settings.repositories.toList()
                        ..removeWhere((e) => e.uuid == repository.uuid)
                        ..add(repository.copyWith(origin: text));
                      context.read<SettingsBloc>().add(
                            SettingsUpdateEvent(
                              settings:
                                  settings.copyWith(repositories: repositories),
                            ),
                          );
                    },
                  ),
                  SettingsTileText(
                    leading: const Icon(Icons.mail),
                    title: const Text('Email'),
                    text: repository.email,
                    onChanged: (text) {
                      final repositories = settings.repositories.toList()
                        ..removeWhere((e) => e.uuid == repository.uuid)
                        ..add(repository.copyWith(email: text));
                      context.read<SettingsBloc>().add(
                            SettingsUpdateEvent(
                              settings:
                                  settings.copyWith(repositories: repositories),
                            ),
                          );
                    },
                  ),
                  SettingsTileText(
                    leading: const Icon(Icons.person),
                    title: const Text('Author'),
                    text: repository.author,
                    onChanged: (text) {
                      final repositories = settings.repositories.toList()
                        ..removeWhere((e) => e.uuid == repository.uuid)
                        ..add(repository.copyWith(author: text));
                      context.read<SettingsBloc>().add(
                            SettingsUpdateEvent(
                              settings:
                                  settings.copyWith(repositories: repositories),
                            ),
                          );
                    },
                  ),
                  SettingsTileText(
                    leading: const FaIcon(FontAwesomeIcons.codeBranch),
                    title: const Text('Branch'),
                    text: repository.branch,
                    onChanged: (text) {
                      final repositories = settings.repositories.toList()
                        ..removeWhere((e) => e.uuid == repository.uuid)
                        ..add(repository.copyWith(branch: text));
                      context.read<SettingsBloc>().add(
                            SettingsUpdateEvent(
                              settings:
                                  settings.copyWith(repositories: repositories),
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
                      selected: repository.sshKeyUuid,
                      onTap: (key) {
                        final repositories = settings.repositories.toList()
                          ..removeWhere((e) => e.uuid == repository.uuid)
                          ..add(repository.copyWith(sshKeyUuid: key.uuid));
                        context.read<SettingsBloc>().add(
                              SettingsUpdateEvent(
                                settings: settings.copyWith(
                                  repositories: repositories,
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
                      repository:
                          context.read<TaskBloc>().repository() as TaskStorage,
                    ),
                  ),
                  SettingsTileNavigation(
                    leading: const Icon(Icons.lock),
                    title: const Text('Encryption'),
                    builder: (context) => EncryptionKeyRoute(
                      repository: repository,
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
            ],
          );
        },
      ),
    );
  }

  Future<void> _exportTasks(BuildContext context) async {
    final taskBloc = context.read<TaskBloc>();
    final logBloc = context.read<LogBloc>();

    await logBloc.catch_(message: 'export tasks', () async {
      final contents = await taskBloc.repository()?.export_() ?? '';
      final filepath = await FilePicker.platform.saveFile(
        dialogTitle: 'Export Tasks',
        fileName: 'tasks.json',
        bytes: const Utf8Encoder().convert(contents),
      );

      // On mobile `bytes` saves the file, and doing a write later can error,
      // due to permission errors. So return early.
      if (Platform.isAndroid || Platform.isIOS) {
        return;
      }

      // Canceled.
      if (filepath == null) {
        return;
      }

      // On Desktop `bytes` does not write, it only gives the path to the
      // non-existant file. So we have to write to it directly.
      await File(filepath).writeAsString(contents, flush: true);
    });
  }

  Future<void> _importTasks(BuildContext context) async {
    final taskBloc = context.read<TaskBloc>();
    final logBloc = context.read<LogBloc>();

    await logBloc.catch_(message: 'import tasks', () async {
      // TODO: Maybe allow importing multiple files.
      final result = await FilePicker.platform.pickFiles(
        dialogTitle: 'Import tasks',
      );

      if (result == null) {
        return;
      }

      final file = result.files.firstOrNull;
      if (file == null) {
        Logger.error(message: 'import file not selected.');
        return;
      }

      final content = await file.xFile.readAsString();

      taskBloc.repository()?.import_(content: content);
      taskBloc.repository()?.addAndCommit(message: r'$IMPORT');
      taskBloc.add(TaskFetchEvent());
    });
  }
}
