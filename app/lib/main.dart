import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:path/path.dart' as path;
import 'package:path_provider/path_provider.dart';
import 'package:stride/blocs/dialog_bloc.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/blocs/plugin_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/bridge/api/plugin_manager.dart' as pm;
import 'package:stride/bridge/api/settings.dart';
import 'package:stride/bridge/frb_generated.dart';
import 'package:stride/bridge/third_party/stride_plugin_manager/manager.dart';
import 'package:stride/routes/initial_route.dart';
import 'package:stride/routes/logging_routes.dart';
import 'package:stride/routes/tasks_route.dart';
import 'package:stride/theme.dart';
import 'package:stride/utils/functions.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();

  final supportPath = await getApplicationSupportDirectory();
  final documentPath = await getApplicationDocumentsDirectory();
  final cachePath = await getApplicationCacheDirectory();

  await RustLib.init();
  final settings = await Settings.load(
    paths: ApplicationPaths(
      supportPath: supportPath.path,
      documentPath: documentPath.path,
      cachePath: cachePath.path,
      logPath: path.joinAll([cachePath.path, 'logs', 'log.txt']),
    ),
  );

  final pluginPath = path.join(supportPath.path, 'plugins');
  await pm.load(pluginPath: pluginPath);
  final plugins = await pm.pluginManifests();
  final pluginManagerState = PluginManagerState(plugins: plugins);

  runApp(
    MyApp(
      settings: settings,
      pluginManagerState: pluginManagerState,
    ),
  );
}

class MyApp extends StatelessWidget {
  final Settings settings;
  final PluginManagerState pluginManagerState;
  const MyApp({
    super.key,
    required this.settings,
    required this.pluginManagerState,
  });

  @override
  Widget build(BuildContext context) {
    return MultiBlocProvider(
      providers: [
        BlocProvider<DialogBloc>(create: (context) => DialogBloc()),
        BlocProvider<LogBloc>(create: (context) => LogBloc()),
        BlocProvider<SettingsBloc>(
          create: (context) => SettingsBloc(
            settings: settings,
            logBloc: context.read<LogBloc>(),
          ),
        ),
        BlocProvider<PluginManagerBloc>(
          create: (context) => PluginManagerBloc(
            logBloc: context.read<LogBloc>(),
            state: pluginManagerState,
          ),
        ),
        BlocProvider<TaskBloc>(
          create: (context) => TaskBloc(
            settingsBloc: context.read<SettingsBloc>(),
            logBloc: context.read<LogBloc>(),
            dialogBloc: context.read<DialogBloc>(),
            pluginManagerBloc: context.read<PluginManagerBloc>(),
          ),
        ),
      ],
      child: BlocListener<TaskBloc, TaskState>(
        listener: (context, state) {
          for (var i = 0; i < state.tasks.length; i++) {
            // _scheduleNotification(state.tasks[i]);
          }
        },
        child: BlocBuilder<SettingsBloc, SettingsState>(
          builder: (context, state) {
            return MaterialApp(
              title: 'Stride',
              theme: generateTheme(darkMode: false),
              darkTheme: generateTheme(darkMode: true),
              themeMode:
                  state.settings.darkMode ? ThemeMode.dark : ThemeMode.light,
              home: BlocListener<LogBloc, LogState>(
                listener: (context, state) {
                  if (!state.show) {
                    return;
                  }

                  ScaffoldMessenger.of(context).showSnackBar(
                    SnackBar(
                      content: Text(state.message.split('\n')[0]),
                      behavior: SnackBarBehavior.floating,
                      duration: const Duration(seconds: 10),
                      backgroundColor: state.isError ? Colors.red[300] : null,
                      action: SnackBarAction(
                        label: 'Go to Logs',
                        onPressed: () async {
                          // TODO: Maybe don't push if already there on top.
                          await Navigator.of(context).push<void>(
                            MaterialPageRoute(
                              builder: (context) => const LoggingRoute(),
                            ),
                          );
                        },
                      ),
                    ),
                  );
                },
                child: BlocListener<DialogBloc, DialogState>(
                  listener: (context, state) async {
                    final title = await state.title(context);
                    if (!context.mounted) return;
                    final description = await state.content?.call(context);
                    if (!context.mounted) return;
                    showAlertDialog(
                      context: context,
                      content: description == null
                          ? title
                          : Column(
                              mainAxisSize: MainAxisSize.min,
                              children: [title, description],
                            ),
                      onConfirm: state.onConfirm,
                      onCancel: state.onCancel,
                    );
                  },
                  child: settings.repositories.isEmpty
                      ? const InitialRoute()
                      : const TasksRoute(),
                ),
              ),
            );
          },
        ),
      ),
    );
  }
}
