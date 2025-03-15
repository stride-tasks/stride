import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/blocs/plugin_manager_bloc.dart';
import 'package:stride/bridge/api/logging.dart' as logging;
import 'package:stride/bridge/api/plugin.dart';
import 'package:stride/bridge/api/plugin_manager.dart' as pm;
import 'package:stride/bridge/third_party/stride_plugin_manager/manifest.dart';
import 'package:stride/routes/plugin_route.dart';
import 'package:stride/utils/functions.dart';
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
          final plugins =
              state.plugins.map((plugin) => _plugin(context, plugin)).toList();
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
          await logBloc.catch_(message: 'import plugin', () async {
            final result = await FilePicker.platform.pickFiles(
              dialogTitle: 'Import Plugin',
            );

            if (result == null) {
              return;
            }

            final file = result.files.firstOrNull;
            if (file == null) {
              logging.error(message: 'plugin file not selected.');
              return;
            }

            final filepath = file.xFile.path;
            final manifest = await pm.parsePlugin(filepath: filepath);
            if (context.mounted) {
              Navigator.of(context).push<void>(
                MaterialPageRoute(
                  builder: (context) => PluginRoute(
                    plugin: manifest,
                    importPath: file.xFile.path,
                  ),
                ),
              );
            }
          });
        },
        child: const Icon(Icons.add),
      ),
    );
  }

  SettingsTile _plugin(BuildContext context, PluginManifestPluginState plugin) {
    final name = pluginInstanceManifestName(manifest: plugin);
    final enabled = pluginInstanceManifestEnabled(manifest: plugin);
    final reason = pluginInstanceManifestDisabledReason(manifest: plugin);
    return SettingsTile(
      title: Text(name),
      leading: enabled
          ? const Icon(Icons.check_circle)
          : _deleteButton(context, name),
      description: !enabled && reason != null
          ? RichText(
              text: TextSpan(
                style: TextStyle(fontSize: 12),
                children: [
                  TextSpan(
                    text: 'Disable: ',
                    style: const TextStyle(fontWeight: FontWeight.bold),
                  ),
                  TextSpan(text: reason),
                ],
              ),
            )
          : null,
      trailing: Switch.adaptive(
        value: enabled,
        activeColor: Colors.redAccent,
        onChanged: (value) async {
          await context.read<PluginManagerBloc>().toggle(name);
        },
      ),
      onTap: (context) {
        Navigator.of(context).push<void>(
          MaterialPageRoute(
            builder: (context) => PluginRoute(plugin: plugin),
          ),
        );
      },
    );
  }

  IconButton _deleteButton(BuildContext context, String name) {
    return IconButton(
      onPressed: () async {
        await showAlertDialog(
          context: context,
          content: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(
                'Are you sure you want to delete $name? (forever)',
                style: const TextStyle(fontWeight: FontWeight.bold),
                textAlign: TextAlign.center,
              ),
            ],
          ),
          onConfirm: (context) async {
            context.read<LogBloc>().catch_(
                  () async => context.read<PluginManagerBloc>().remove(name),
                );
            Navigator.of(context).pop();
            logging.trace(message: 'Plugin $name deleted');
            return true;
          },
        );
      },
      icon: const Icon(Icons.delete_forever),
    );
  }
}
