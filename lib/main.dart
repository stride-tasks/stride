import 'package:catppuccin_flutter/catppuccin_flutter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:path_provider/path_provider.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/src/rust/api/simple.dart';
import 'package:stride/src/rust/frb_generated.dart';
import 'package:stride/routes/routes.dart';
import 'package:stride/theme.dart';

Future<void> main() async {
  await RustLib.init();

  final pathSupport = await getApplicationSupportDirectory();
  final repository = await TaskRepository.load(path: pathSupport.path);

  runApp(MyApp(repository: repository));
}

class MyApp extends StatelessWidget {
  final TaskRepository repository;
  const MyApp({super.key, required this.repository});

  @override
  Widget build(BuildContext context) {
    return MultiBlocProvider(
      providers: [
        BlocProvider<TaskBloc>(
          create: (context) => TaskBloc(repository: repository),
        ),
      ],
      child: BlocListener<TaskBloc, TaskState>(
        listener: (context, state) {
          for (int i = 0; i < state.tasks.length; i++) {
            // _scheduleNotification(state.tasks[i]);
          }
        },
        child: MaterialApp(
          title: 'Stride',
          theme: catppuccinTheme(catppuccin.frappe),
          darkTheme: catppuccinTheme(catppuccin.macchiato),
          onGenerateRoute: Routes.onGenerateRoute,
        ),
      ),
    );
  }
}
