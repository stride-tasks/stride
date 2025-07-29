import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/bridge/api/repository.dart';
import 'package:stride/routes/backend/list_route.dart';
import 'package:stride/utils/functions.dart';
import 'package:stride/widgets/settings_widget.dart';
import 'package:uuid/uuid.dart';

class RepositoryRoute extends StatelessWidget {
  final UuidValue repositoryUuid;
  TextStyle get headingStyle => const TextStyle(
    fontSize: 16,
    fontWeight: FontWeight.w600,
    color: Colors.red,
  );

  const RepositoryRoute({super.key, required this.repositoryUuid});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Repository')),
      body: BlocBuilder<SettingsBloc, SettingsState>(
        builder: (context, state) {
          final settings = state.settings;
          final repositoryIndex = settings.repositories.indexWhere(
            (element) => element.uuid == repositoryUuid,
          );
          if (repositoryIndex == -1) {
            return const Column();
          }
          final repositorySpec = settings.repositories[repositoryIndex];

          final repository = Repository.open(uuid: repositoryUuid);
          return SettingsList(
            sections: [
              SettingsSection(
                title: Text('General', style: headingStyle),
                tiles: [
                  SettingsTileText(
                    title: const Text('Name'),
                    leading: const Icon(Icons.label),
                    text: repositorySpec.name,
                    onChanged: (text) {
                      final repositories = settings.repositories
                          .map(
                            (e) => (e.uuid != repositorySpec.uuid)
                                ? e
                                : e.copyWith(name: text),
                          )
                          .toList();
                      context.read<SettingsBloc>().add(
                        SettingsUpdateEvent(
                          settings: settings.copyWith(
                            repositories: repositories,
                          ),
                        ),
                      );
                    },
                  ),
                  // TODO: Currently only work for the currently loaded repository.
                  // SettingsTile(
                  //   leading: const Icon(Icons.save_alt),
                  //   title: const Text('Export Tasks'),
                  //   onTap: _exportTasks,
                  // ),
                  // SettingsTile(
                  //   leading: const Icon(Icons.file_open),
                  //   title: const Text('Import Tasks'),
                  //   onTap: _importTasks,
                  // ),
                  SettingsTileNavigation(
                    leading: const Icon(Icons.backup),
                    title: const Text('Backends'),
                    builder: (context) =>
                        BackendListRoute(repository: repository),
                  ),
                ],
              ),
              SettingsSection(
                title: Text('Git Integration', style: headingStyle),
                tiles: [
                  SettingsTile(
                    leading: const Icon(
                      Icons.delete_forever,
                      color: Colors.red,
                    ),
                    title: const Text('Remove Repository'),
                    onTap: (context) async {
                      await showAlertDialog(
                        context: context,
                        content: const Text(
                          'Are you sure you want to delete the (local) repository, including the repository settings?',
                          style: TextStyle(fontWeight: FontWeight.bold),
                          textAlign: TextAlign.center,
                        ),
                        onConfirm: (context) async {
                          context.read<TaskBloc>().add(
                            TaskRemoveRepositoryEvent(
                              uuid: repositorySpec.uuid,
                            ),
                          );
                          final settingsBloc = context.read<SettingsBloc>();
                          final settings = settingsBloc.settings;
                          settingsBloc.add(
                            SettingsUpdateEvent(
                              settings: settings.copyWith(
                                repositories: settings.repositories.toList()
                                  ..removeWhere(
                                    (element) =>
                                        element.uuid == repositorySpec.uuid,
                                  ),
                              ),
                            ),
                          );
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

  // Future<void> _exportTasks(BuildContext context) async {
  //   final taskBloc = context.read<TaskBloc>();
  //   final logBloc = context.read<LogBloc>();

  //   await logBloc.catch_(message: 'export tasks', () async {
  //     final contents = await taskBloc.repository()?.export_() ?? '';
  //     final filepath = await FilePicker.platform.saveFile(
  //       dialogTitle: 'Export Tasks',
  //       fileName: 'tasks.json',
  //       bytes: const Utf8Encoder().convert(contents),
  //     );

  //     // On mobile `bytes` saves the file, and doing a write later can error,
  //     // due to permission errors. So return early.
  //     if (Platform.isAndroid || Platform.isIOS) {
  //       return;
  //     }

  //     // Canceled.
  //     if (filepath == null) {
  //       return;
  //     }

  //     // On Desktop `bytes` does not write, it only gives the path to the
  //     // non-existant file. So we have to write to it directly.
  //     await File(filepath).writeAsString(contents, flush: true);
  //   });
  // }

  // Future<void> _importTasks(BuildContext context) async {
  //   final taskBloc = context.read<TaskBloc>();
  //   final logBloc = context.read<LogBloc>();

  //   await logBloc.catch_(message: 'import tasks', () async {
  //     // TODO: Maybe allow importing multiple files.
  //     final result = await FilePicker.platform.pickFiles(
  //       dialogTitle: 'Import tasks',
  //     );

  //     if (result == null) {
  //       return;
  //     }

  //     final file = result.files.firstOrNull;
  //     if (file == null) {
  //       logging.error(message: 'import file not selected.');
  //       return;
  //     }

  //     final content = await file.xFile.readAsString();

  //     taskBloc.repository()?.import_(content: content);
  //     taskBloc.repository()?.addAndCommit(message: r'$IMPORT');
  //     taskBloc.add(TaskFetchEvent());
  //   });
  // }
}
