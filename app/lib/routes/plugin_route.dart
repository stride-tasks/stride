import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:intersperse/intersperse.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/blocs/plugin_bloc.dart';
import 'package:stride/bridge/api/plugin.dart';
import 'package:stride/bridge/third_party/stride_plugin_manager/manifest.dart';
import 'package:stride/widgets/settings_widget.dart';

class PluginRoute extends StatelessWidget {
  final PluginManifestPluginState plugin;
  final String? importPath;

  const PluginRoute({super.key, required this.plugin, this.importPath});

  TextStyle get headingStyle => const TextStyle(
        fontSize: 16,
        fontWeight: FontWeight.w600,
        color: Colors.red,
      );

  @override
  Widget build(BuildContext context) {
    final name = pluginInstanceManifestName(manifest: plugin);
    final events = pluginInstanceManifestEvents(manifest: plugin);
    final permissions = pluginInstanceManifestPermissions(manifest: plugin);
    return Scaffold(
      appBar: AppBar(title: const Text('Plugin')),
      body: SettingsList(
        sections: [
          SettingsSection(
            title: Text('General', style: headingStyle),
            tiles: [
              SettingsTile(
                title: Text('Name'),
                description: Text(
                  name,
                  overflow: TextOverflow.ellipsis,
                ),
              ),
            ],
          ),
          SettingsSection(
            title: Text('Events', style: headingStyle),
            tiles: [
              _manifestField('task.create', events.task.create),
              _manifestField('task.modify', events.task.modify),
              _manifestField('task.sync', events.task.sync_),
            ],
          ),
          SettingsSection(
            title: Text('Permissions', style: headingStyle),
            tiles: [
              _manifestField('task.create', permissions.task.create),
              _manifestField('task.modify', permissions.task.modify),
              _manifestField('task.sync', permissions.task.sync_),
            ],
          ),
        ],
      ),
      floatingActionButton: importPath == null
          ? null
          : FloatingActionButton(
              shape: const CircleBorder(),
              onPressed: () async {
                final logBloc = context.read<LogBloc>();
                final pluginManagerBloc = context.read<PluginManagerBloc>();
                Navigator.of(context).pop();
                await logBloc.catch_(message: 'import plugin', () async {
                  await pluginManagerBloc.import(importPath!);
                });
              },
              child: const Icon(Icons.check_circle),
            ),
    );
  }

  SettingsTile _manifestField(String name, bool value) {
    final spans = name
        .split('.')
        .map(
          (text) => TextSpan(
            text: text,
            style: const TextStyle(fontStyle: FontStyle.italic, fontSize: 16),
          ),
        )
        .intersperse(
          TextSpan(
            text: ' . ',
            style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 20),
          ),
        )
        .toList();
    return SettingsTile(
      title: RichText(text: TextSpan(children: spans)),
      trailing: Switch.adaptive(
        value: value,
        activeColor: Colors.redAccent,
        onChanged: (value) {},
      ),
    );
  }
}
