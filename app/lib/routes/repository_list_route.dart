import 'package:flutter/material.dart';
import 'package:stride/api.dart';
import 'package:stride/context.dart';
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
      body: ValueListenableBuilder(
        valueListenable: RustContext.repositories,
        builder: (context, repositories, child) {
          final tiles = repositories.map((repository) {
            return SettingsTileNavigation(
              title: Text(repository.name ?? 'unnamed'),
              leading: const Icon(Icons.task),
              builder: (context) =>
                  RepositoryRoute(repositoryUuid: repository.id),
            );
          }).toList();
          return SettingsList(
            sections: [
              SettingsSection(
                title: Text('Repositories', style: headingStyle),
                tiles: tiles,
              ),
            ],
          );
        },
      ),
    );
  }
}
