import 'package:flutter/material.dart';
import 'package:stride/src/rust/routes/route_not_found_route.dart';
import 'package:stride/src/rust/routes/task_add_route.dart';
import 'package:stride/src/rust/routes/task_edit_route.dart';
import 'package:stride/src/rust/routes/tasks_route.dart';
import 'package:stride/src/rust/task.dart';

class Routes {
  static const start = '/';
  static const taskAdd = '/taskAdd/';
  static const taskEdit = '/taskEdit/';

  static Route onGenerateRoute(RouteSettings routeSettings) {
    switch (routeSettings.name) {
      case start:
        return MaterialPageRoute(builder: (context) => const TasksRoute());
      case taskAdd:
        return MaterialPageRoute(builder: (context) => const TaskAddRoute());
      case taskEdit:
        return MaterialPageRoute(
          builder: (context) => TaskEditRoute(
            task: routeSettings.arguments as Task,
          ),
        );
      default:
        return MaterialPageRoute(
          builder: (context) => const RouteNotFoundRoute(),
        );
    }
  }
}
