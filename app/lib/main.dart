import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:path/path.dart' as path;
import 'package:path_provider/path_provider.dart';
import 'package:stride/api.dart';
import 'package:stride/background.dart';
import 'package:stride/bridge.dart' show NativeNotification, RustException;
import 'package:stride/context.dart';
import 'package:stride/controllers/task_controller.dart';
import 'package:stride/logging.dart';
import 'package:stride/notifications.dart';
import 'package:stride/routes/initial_route.dart';
import 'package:stride/routes/tasks_route.dart';
import 'package:stride/theme.dart';
import 'package:stride/utils/functions.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();

  String supportPath;
  String cachePath;
  if (Platform.isLinux || Platform.isMacOS || Platform.isWindows) {
    final environment = Platform.environment['STRIDE_HOME'];
    if (environment != null) {
      supportPath = environment;
      cachePath = path.join(environment, 'cache');
    } else {
      supportPath = (await getApplicationSupportDirectory()).path;
      cachePath = (await getApplicationCacheDirectory()).path;
    }
  } else {
    supportPath = (await getApplicationSupportDirectory()).path;
    cachePath = (await getApplicationCacheDirectory()).path;
  }

  await RustContext.init(supportPath, cachePath);
  await Background.init();
  await NotificationService.init();

  await RustContext.refreshRepositories();

  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return TaskControllerProvider(
      child: ValueListenableBuilder(
        valueListenable: RustContext.setting,
        builder: (context, setting, _) {
          return MaterialApp(
            title: 'Stride',
            theme: generateTheme(darkMode: false),
            darkTheme: generateTheme(darkMode: true),
            themeMode: setting.darkMode ? ThemeMode.dark : ThemeMode.light,
            home: LogNotificationListener(
              child: _UserPromptListener(
                child: ValueListenableBuilder(
                  valueListenable: RustContext.repositories,
                  builder: (context, repositories, child) =>
                      repositories.isEmpty
                      ? const InitialRoute()
                      : const TasksRoute(),
                ),
              ),
            ),
          );
        },
      ),
    );
  }
}

class _UserPromptListener extends StatefulWidget {
  const _UserPromptListener({required this.child});

  final Widget child;

  @override
  State<_UserPromptListener> createState() => _UserPromptListenerState();
}

class _UserPromptListenerState extends State<_UserPromptListener> {
  late final StreamSubscription<NativeNotification> _subscription;

  @override
  void initState() {
    super.initState();
    _subscription = RustContext.stream().listen(_onNotification);
  }

  void _onNotification(NativeNotification event) {
    if (event.name != 'user.prompt') return;

    final params = jsonDecode(event.params) as Map<String, dynamic>;
    final notification = UserPromptNotification.fromJson(params);
    if (!mounted) return;

    final title = Text(
      notification.summary,
      style: const TextStyle(fontWeight: FontWeight.bold),
    );
    final description = notification.description;
    showAlertDialog(
      context: context,
      popRoute: false,
      content: description == null
          ? title
          : Column(
              mainAxisSize: MainAxisSize.min,
              children: [title, Text(description, softWrap: true)],
            ),
      onConfirm: (context) async {
        try {
          await RustContext.executeErased(
            notification.target.method,
            jsonEncode({'params': notification.target.params}),
          );
        } on RustException catch (error) {
          Log.error('Error executing Rust method: $error');
        }
        if (context.mounted) {
          Navigator.of(context).pop();
        }
        return true;
      },
      onCancel: (context) async {
        Navigator.of(context).pop();
        return false;
      },
    );
  }

  @override
  void dispose() {
    unawaited(_subscription.cancel());
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => widget.child;
}
