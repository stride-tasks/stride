import 'package:flutter/material.dart';
import 'package:stride/routes/route_not_found_route.dart';
import 'package:stride/routes/task_add_route.dart';
import 'package:stride/routes/task_edit_route.dart';
import 'package:stride/routes/tasks_route.dart';
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
