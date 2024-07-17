import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:path/path.dart' as path;
import 'package:path_provider/path_provider.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/routes/routes.dart';
import 'package:stride/src/rust/api/paths.dart';
import 'package:stride/src/rust/api/repository.dart';
import 'package:stride/src/rust/api/settings.dart';
import 'package:stride/src/rust/frb_generated.dart';
import 'package:stride/theme.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();

  final supportPath = await getApplicationSupportDirectory();
  final documentPath = await getApplicationDocumentsDirectory();
  final cachePath = await getApplicationCacheDirectory();

  await RustLib.init();
  await ApplicationPaths.init(
    paths: ApplicationPaths(
      supportPath: supportPath.path,
      documentPath: documentPath.path,
      cachePath: cachePath.path,
      logPath: path.joinAll([cachePath.path, 'logs', 'log.txt']),
    ),
  );
  // TODO: Better initialize settings.
  final settings = await Settings.load();

  final repository = TaskStorage(path: supportPath.path);

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
        BlocProvider<SettingsBloc>(
          create: (context) => SettingsBloc(settings: settings),
        ),
        BlocProvider<TaskBloc>(
          create: (context) => TaskBloc(
            settingsBloc: context.read<SettingsBloc>(),
            repository: repository,
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
              onGenerateRoute: Routes.onGenerateRoute,
            );
          },
        ),
      ),
    );
  }
}
