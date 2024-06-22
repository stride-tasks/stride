import 'package:flutter/material.dart';
import 'package:stride/routes/logging_routes.dart';
import 'package:stride/routes/route_not_found_route.dart';
import 'package:stride/routes/settings_route.dart';
import 'package:stride/routes/ssh_key_add_route.dart';
import 'package:stride/routes/ssh_keys_route.dart';
import 'package:stride/routes/task_filter_route.dart';
import 'package:stride/routes/task_route.dart';
import 'package:stride/routes/tasks_route.dart';
import 'package:stride/src/rust/api/filter.dart';
import 'package:stride/src/rust/task.dart';

class Routes {
  static const start = '/';
  static const taskAdd = '/taskAdd/';
  static const taskEdit = '/taskEdit/';
  static const taskFilter = '/taskFilter/';
  static const settings = '/settings/';
  static const sshKeys = '/sshKeys/';
  static const sshKeysAdd = '/sshKeysAdd/';
  static const logging = '/logging/';

  static Route onGenerateRoute(RouteSettings routeSettings) {
    switch (routeSettings.name) {
      case start:
        return MaterialPageRoute(builder: (context) => const TasksRoute());
      case taskAdd:
        return MaterialPageRoute(builder: (context) => const TaskRoute());
      case taskEdit:
        return MaterialPageRoute(
          builder: (context) => TaskRoute(
            task: routeSettings.arguments as Task,
          ),
        );
      case taskFilter:
        return MaterialPageRoute(
          builder: (context) => TaskFilterRoute(
            filter: routeSettings.arguments == null
                ? null
                : routeSettings.arguments as Filter,
          ),
        );
      case settings:
        return MaterialPageRoute(builder: (context) => const SettingsRoute());
      case sshKeys:
        return MaterialPageRoute(builder: (context) => const SshKeysRoute());
      case sshKeysAdd:
        return MaterialPageRoute(builder: (context) => const SshKeyAddRoute());
      case logging:
        return MaterialPageRoute(builder: (context) => const LoggingRoute());
      default:
        return MaterialPageRoute(
          builder: (context) => const RouteNotFoundRoute(),
        );
    }
  }
}
