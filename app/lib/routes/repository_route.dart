import 'dart:convert';

import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:stride/api.dart';
import 'package:stride/context.dart';
import 'package:stride/controllers/task_controller.dart';
import 'package:stride/logging.dart';
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
      body: ValueListenableBuilder(
        valueListenable: RustContext.repositories,
        builder: (context, repositories, child) {
          final repositoryDescriptor = repositories
              .where((element) => element.id == repositoryUuid)
              .firstOrNull;
          if (repositoryDescriptor == null) {
            return const Column();
          }

          return SettingsList(
            sections: [
              SettingsSection(
                title: Text('General', style: headingStyle),
                tiles: [
                  SettingsTileText(
                    title: const Text('Name'),
                    leading: const Icon(Icons.label),
                    text: repositoryDescriptor.name ?? 'unnamed',
                    onChanged: (text) async {
                      await RepositoryUpdateMethod(
                        descriptor: repositoryDescriptor.copyWith(name: text),
                      ).execute();
                      await RustContext.refreshRepositories();
                    },
                  ),
                  SettingsTile(
                    leading: const Icon(Icons.save_alt),
                    title: const Text('Export Tasks'),
                    onTap: (context) =>
                        _exportTasks(context, repositoryDescriptor.id),
                  ),
                  SettingsTile(
                    leading: const Icon(Icons.file_open),
                    title: const Text('Import Tasks'),
                    onTap: (context) =>
                        _importTasks(context, repositoryDescriptor.id),
                  ),
                  SettingsTileNavigation(
                    leading: const Icon(Icons.backup),
                    title: const Text('Backends'),
                    builder: (context) => BackendListRoute(
                      repositoryUuid: repositoryDescriptor.id,
                    ),
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
                      final repositories = RustContext.repositories.value
                          .where(
                            (element) => element.id != repositoryDescriptor.id,
                          )
                          .toList();
                      final currentRepository =
                          RustContext.setting.value.currentRepository;
                      await showAlertDialog(
                        context: context,
                        content: const Text(
                          'Are you sure you want to delete the (local) repository, including the repository settings?',
                          style: TextStyle(fontWeight: FontWeight.bold),
                          textAlign: TextAlign.center,
                        ),
                        onConfirm: (context) async {
                          context.taskController.removeRepository(
                            repositoryDescriptor.id,
                          );
                          if (currentRepository == repositoryDescriptor.id) {
                            RustContext.setting.value = RustContext
                                .setting
                                .value
                                .copyWith(
                                  currentRepository:
                                      repositories.firstOrNull?.id,
                                );
                          }
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

  Future<void> _exportTasks(
    BuildContext context,
    UuidValue repositoryId,
  ) async {
    await Log.catch_(message: 'export tasks', () async {
      final result = await RepositoryExportMethod(
        repositoryId: repositoryId,
      ).execute();
      final contents = const JsonEncoder.withIndent(' ').convert(
        result.tasks.map((task) => task.serde.serialize(task)).toList(),
      );
      final bytes = utf8.encode(contents);
      final filepath = await FilePicker.saveFile(
        dialogTitle: 'Export Tasks',
        fileName: 'tasks.json',
        type: FileType.custom,
        allowedExtensions: const ['json'],
        bytes: bytes,
      );

      if (filepath == null) return;

      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Exported ${result.tasks.length} tasks.')),
        );
      }
    });
  }

  Future<void> _importTasks(
    BuildContext context,
    UuidValue repositoryId,
  ) async {
    final taskController = context.taskController;
    await Log.catch_(message: 'import tasks', () async {
      final result = await FilePicker.pickFiles(
        dialogTitle: 'Import Tasks',
        type: FileType.custom,
        allowedExtensions: const ['json'],
      );
      final file = result?.files.firstOrNull;
      if (file == null) return;

      final decoded = jsonDecode(await file.xFile.readAsString());
      if (decoded is! List) {
        throw const FormatException('Task import must contain a JSON array.');
      }

      final tasks = decoded.map((value) {
        if (value is! Map<String, dynamic>) {
          throw const FormatException(
            'Each imported task must be a JSON object.',
          );
        }
        return Task.fromJson(value);
      }).toList();

      final imported = await RepositoryImportMethod(
        repositoryId: repositoryId,
        tasks: tasks,
      ).execute();
      await taskController.fetch();

      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Imported ${imported.imported} tasks.')),
        );
      }
    });
  }
}
