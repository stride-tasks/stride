import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:stride/api.dart';
import 'package:stride/context.dart';
import 'package:stride/logging.dart';
import 'package:stride/routes/plugin_route.dart';
import 'package:stride/utils/functions.dart';
import 'package:stride/widgets/settings_widget.dart';

class PluginListRoute extends StatefulWidget {
  const PluginListRoute({super.key});

  @override
  State<PluginListRoute> createState() => _PluginListRouteState();
}

class _PluginListRouteState extends State<PluginListRoute> {
  Future<List<PluginManifestInstance>>? _plugins;

  @override
  void initState() {
    super.initState();

    _reloadPlugins();
  }

  void _reloadPlugins() {
    setState(() {
      _plugins = PluginListMethod().execute().then((result) => result.plugins);
    });
  }

  TextStyle get headingStyle => const TextStyle(
    fontSize: 16,
    fontWeight: FontWeight.w600,
    color: Colors.red,
  );

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Plugin List')),
      body: FutureBuilder(
        future: _plugins,
        builder: (context, state) {
          if (state.connectionState == ConnectionState.waiting) {
            return const Center(child: CircularProgressIndicator());
          } else if (state.hasError) {
            return Center(child: Text('Error: ${state.error}'));
          } else if (!state.hasData || state.data!.isEmpty) {
            return const Center(child: Text('No plugins found.'));
          }

          final plugins = state.data!
              .map((plugin) => _plugin(context, plugin))
              .toList();
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
          await Log.catch_(message: 'import plugin', () async {
            final result = await FilePicker.pickFiles(
              dialogTitle: 'Import Plugin',
            );

            if (result == null) {
              return;
            }

            final file = result.files.firstOrNull;
            if (file == null) {
              Log.error('plugin file not selected.');
              return;
            }

            final filepath = file.xFile.path;
            final manifest = (await PluginValidateMethod(
              filePath: filepath,
            ).execute()).plugin;
            if (context.mounted) {
              await Navigator.of(context).push<void>(
                MaterialPageRoute(
                  builder: (context) => PluginRoute(
                    plugin: manifest,
                    importPath: file.xFile.path,
                  ),
                ),
              );

              _reloadPlugins();
            }
          });
        },
        child: const Icon(Icons.add),
      ),
    );
  }

  SettingsTile _plugin(BuildContext context, PluginManifestInstance plugin) {
    final name = plugin.manifest.name;
    final enabled = plugin.state is PluginState_Enable;
    final reason = switch (plugin.state) {
      PluginState_Disable(:final reason) => reason,
      PluginState_Enable() => null,
    };
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
          await PluginToggleMethod(pluginName: name).execute();
          _reloadPlugins();
        },
      ),
      onTap: (context) {
        Navigator.of(context).push<void>(
          MaterialPageRoute(builder: (context) => PluginRoute(plugin: plugin)),
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
            await Log.catch_(
              () async => PluginRemoveMethod(pluginName: name).execute(),
            );
            await Log.trace('Plugin $name deleted');
            _reloadPlugins();
            return true;
          },
        );
      },
      icon: const Icon(Icons.delete_forever),
    );
  }
}
