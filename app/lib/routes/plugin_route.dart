import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:intersperse/intersperse.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/blocs/plugin_manager_bloc.dart';
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
    final event = pluginInstanceManifestEvent(manifest: plugin);
    final permission = pluginInstanceManifestPermission(manifest: plugin);
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
            title: Text('Event', style: headingStyle),
            tiles: [
              if (event.task?.create ?? false)
                _manifestBoolField('task.create', event.task?.create),
              if (event.task?.modify ?? false)
                _manifestBoolField('task.modify', event.task?.modify),
              if (event.task?.sync_ ?? false)
                _manifestBoolField('task.sync', event.task?.sync_),
            ],
          ),
          SettingsSection(
            title: Text('Permission', style: headingStyle),
            tiles: [
              if (permission.task?.create ?? false)
                _manifestBoolField('task.create', permission.task?.create),
              if (permission.task?.modify ?? false)
                _manifestBoolField('task.modify', permission.task?.modify),
              if (permission.task?.sync_ ?? false)
                _manifestBoolField('task.sync', permission.task?.sync_),
              if (permission.task?.query ?? false)
                _manifestBoolField('task.query', permission.task?.query),
              if (permission.storage != null)
                _manifestStorageField(permission.storage!),
              if (permission.network != null)
                _manifestNetworkField(permission.network!),
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

  SettingsTile _manifestBoolField(String name, bool? value) {
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
        value: value ?? false,
        activeColor: Colors.redAccent,
        onChanged: (value) {},
      ),
    );
  }

  SettingsTile _manifestStorageField(ManifestPermissionStorage storage) {
    var size = storage.maxSize.toString();
    var unit = 'KB';
    if (storage.maxSize > 1024) {
      size = (storage.maxSize / 1024).toStringAsFixed(2);
      unit = 'MB';
    }
    final children = [
      ListTile(
        title: RichText(
          text: TextSpan(
            text: 'max-size',
            style: const TextStyle(fontStyle: FontStyle.italic, fontSize: 18),
          ),
        ),
        subtitle: Text('$size $unit'),
      ),
    ];
    return SettingsTile(
      title: ExpansionTile(
        initiallyExpanded: true,
        tilePadding: EdgeInsets.all(0),
        title: Text(
          'Storage',
          style: const TextStyle(fontStyle: FontStyle.italic, fontSize: 18),
        ),
        children: children,
      ),
      trailing: const SizedBox(),
    );
  }

  SettingsTile _manifestNetworkField(ManifestPermissionNetwork network) {
    final urls = network.urls
        .map(
          (url) => ListTile(
            title: RichText(
              text: TextSpan(
                text: 'GET $url',
                style:
                    const TextStyle(fontStyle: FontStyle.italic, fontSize: 16),
              ),
            ),
          ),
        )
        .toList();
    return SettingsTile(
      title: ExpansionTile(
        initiallyExpanded: true,
        tilePadding: EdgeInsets.all(0),
        title: Text(
          'Network (${network.urls.length})',
          style: const TextStyle(fontStyle: FontStyle.italic, fontSize: 18),
        ),
        children: urls,
      ),
      trailing: const SizedBox(),
    );
  }
}
