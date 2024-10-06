import 'dart:convert';
import 'dart:io';
import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/bridge/api/logging.dart';
import 'package:stride/routes/repository_route.dart';
import 'package:stride/widgets/settings_widget.dart';

class RepositoryListRoute extends StatelessWidget {
  TextStyle get headingStyle => const TextStyle(
        fontSize: 16,
        fontWeight: FontWeight.w600,
        color: Colors.red,
      );

  const RepositoryListRoute({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Repository List')),
      body: BlocBuilder<SettingsBloc, SettingsState>(
        builder: (context, state) {
          final settings = state.settings;

          final repositories = settings.repositories.map((repository) {
            return SettingsTileNavigation(
              title: Text(repository.name),
              leading: const Icon(Icons.task),
              builder: (context) => RepositoryRoute(repository: repository),
            );
          }).toList();
          return SettingsList(
            sections: [
              SettingsSection(
                title: Text('Repositories', style: headingStyle),
                tiles: repositories,
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
      final contents = await taskBloc.repository.export_();
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

      taskBloc.repository.import_(content: content);
      taskBloc.repository.addAndCommit(message: r'$IMPORT');
      taskBloc.add(TaskFetchEvent());
    });
  }
}
