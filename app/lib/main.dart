import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:path/path.dart' as path;
import 'package:path_provider/path_provider.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/blocs/tost_bloc.dart';
import 'package:stride/bridge/api/repository.dart';
import 'package:stride/bridge/api/settings.dart';
import 'package:stride/bridge/frb_generated.dart';
import 'package:stride/routes/tasks_route.dart';
import 'package:stride/theme.dart';

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

  final repository = TaskStorage(
    path: path.join(supportPath.path, 'repository'),
    settings: settings,
  );

  runApp(
    MyApp(
      repository: repository,
      settings: settings,
    ),
  );
}

class MyApp extends StatelessWidget {
  final TaskStorage repository;
  final Settings settings;
  const MyApp({super.key, required this.repository, required this.settings});

  @override
  Widget build(BuildContext context) {
    return MultiBlocProvider(
      providers: [
        BlocProvider<TostBloc>(
          create: (context) => TostBloc(),
        ),
        BlocProvider<SettingsBloc>(
          create: (context) => SettingsBloc(
            settings: settings,
            tostBloc: context.read<TostBloc>(),
          ),
        ),
        BlocProvider<TaskBloc>(
          create: (context) => TaskBloc(
            repository: repository,
            settingsBloc: context.read<SettingsBloc>(),
            tostBloc: context.read<TostBloc>(),
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
              home: BlocListener<TostBloc, TostState>(
                listener: (context, state) {
                  ScaffoldMessenger.of(context).showSnackBar(
                    SnackBar(
                      content: Text(state.message.split('\n')[0]),
                      behavior: SnackBarBehavior.floating,
                      duration: Duration(seconds: 10),
                      backgroundColor: state.isError ? Colors.red[300] : null,
                    ),
                  );
                },
                child: const TasksRoute(),
              ),
            );
          },
        ),
      ),
    );
  }
}
