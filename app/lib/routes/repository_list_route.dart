import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
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
              builder: (context) => RepositoryRoute(
                repositoryUuid: repository.uuid,
              ),
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
}
