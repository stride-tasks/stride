import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/blocs/plugin_bloc.dart';
import 'package:stride/bridge/api/logging.dart';
import 'package:stride/bridge/api/plugin.dart';
import 'package:stride/widgets/settings_widget.dart';

class PluginListRoute extends StatelessWidget {
  const PluginListRoute({super.key});

  TextStyle get headingStyle => const TextStyle(
        fontSize: 16,
        fontWeight: FontWeight.w600,
        color: Colors.red,
      );

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Plugin List')),
      body: BlocBuilder<PluginManagerBloc, PluginManagerState>(
        builder: (context, state) {
          final plugins = state.plugins.map((plugin) {
            return SettingsTile(
              title: Text(pluginInstanceManifestName(manifest: plugin)),
              leading: const Icon(Icons.task),
            );
          }).toList();
          return SettingsList(
            sections: [
              SettingsSection(
                title: Text('Plugins', style: headingStyle),
                tiles: plugins,
              ),
            ],
          );
        },
      ),
      floatingActionButton: FloatingActionButton(
        shape: const CircleBorder(),
        onPressed: () async {
          final logBloc = context.read<LogBloc>();
          final pluginManagerBloc = context.read<PluginManagerBloc>();
          await logBloc.catch_(message: 'import plugin', () async {
            final result = await FilePicker.platform.pickFiles(
              dialogTitle: 'Import Plugin',
            );

            if (result == null) {
              return;
            }

            final file = result.files.firstOrNull;
            if (file == null) {
              Logger.error(message: 'plugin file not selected.');
              return;
            }

            await pluginManagerBloc.import(file.xFile.path);
          });
        },
        child: const Icon(Icons.add),
      ),
    );
  }
}
