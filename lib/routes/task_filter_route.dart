import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/src/rust/api/filter.dart';
import 'package:uuid/uuid.dart';

class TaskFilterRoute extends StatefulWidget {
  final Filter? filter;

  const TaskFilterRoute({
    super.key,
    this.filter,
  });

  @override
  State<TaskFilterRoute> createState() => _TaskFilterRouteState();
}

class _TaskFilterRouteState extends State<TaskFilterRoute> {
  TextEditingController nameController = TextEditingController(text: "");
  TextEditingController searchController = TextEditingController(text: "");

  @override
  void initState() {
    super.initState();

    if (widget.filter != null) {
      final filter = widget.filter!;
      nameController.text = filter.name;
      searchController.text = filter.search;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text(
          "Filters",
          style: TextStyle(fontWeight: FontWeight.bold),
        ),
      ),
      body: BlocBuilder<SettingsBloc, SettingsState>(
        builder: (context, state) {
          return Padding(
            padding: const EdgeInsets.symmetric(
              vertical: 20.0,
              horizontal: 10.0,
            ),
            child: Column(
              children: [
                TextField(
                  controller: searchController,
                  decoration: const InputDecoration(
                    border: OutlineInputBorder(),
                    labelText: "Search",
                  ),
                ),
                const SizedBox(height: 10.0),
                Row(
                  crossAxisAlignment: CrossAxisAlignment.center,
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: _actions(state, context),
                ),
              ],
            ),
          );
        },
      ),
    );
  }

  List<Widget> _actions(SettingsState state, BuildContext context) {
    return [
      ElevatedButton.icon(
        icon: const Icon(Icons.check),
        label: const Text("Apply"),
        onPressed: () {
          var filter = Filter(
            uuid: const Uuid().v4obj(),
            name: "Temporary",
            search: searchController.text,
          );
          var newSettings = state.settings.copyWith(
            selectedFilter: FilterSelection.temporary(
              filter: filter,
            ),
          );
          context
              .read<SettingsBloc>()
              .add(SettingsUpdateEvent(settings: newSettings));
          context.read<TaskBloc>().add(TaskFilterEvent(filter: filter));

          // Pop to first route.
          Navigator.popUntil(
            context,
            (route) => route.isFirst,
          );
        },
      ),
      const SizedBox(width: 20),
      ElevatedButton.icon(
        icon: const Icon(Icons.save),
        label: const Text("Save"),
        onPressed: () async {
          await showDialog(
            context: context,
            builder: (context) => AlertDialog(
              content: SizedBox(
                width: MediaQuery.of(context).size.width * 0.90,
                child: TextField(
                  controller: nameController,
                  autocorrect: false,
                  autofocus: true,
                  decoration: const InputDecoration(
                    hintText: "Filter name...",
                  ),
                ),
              ),
              actions: [
                IconButton(
                  icon: const Icon(Icons.cancel),
                  onPressed: () {
                    Navigator.pop(context);
                  },
                ),
                IconButton(
                  icon: const Icon(Icons.check),
                  onPressed: () {
                    if (nameController.text.isEmpty) {
                      return;
                    }

                    var filters = state.settings.filters.toList();
                    if (widget.filter != null) {
                      var filterIndex = filters.indexWhere(
                        (element) => element.uuid == widget.filter!.uuid,
                      );
                      var filter = widget.filter!.copyWith(
                        name: nameController.text,
                        search: searchController.text,
                      );
                      filters[filterIndex] = filter;
                    } else {
                      final hasSameName = state.settings.filters.every(
                        (element) => element.name == nameController.text,
                      );
                      var name = hasSameName
                          ? "${nameController.text} 2"
                          : nameController.text;
                      var filter = Filter(
                        uuid: const Uuid().v4obj(),
                        name: name,
                        search: searchController.text,
                      );

                      filters.add(filter);
                    }

                    context.read<SettingsBloc>().add(SettingsUpdateEvent(
                          settings: state.settings.copyWith(
                            filters: filters,
                          ),
                        ));

                    // Pop to first route.
                    Navigator.popUntil(
                      context,
                      (route) => route.isFirst,
                    );
                  },
                ),
              ],
            ),
          );
        },
      ),
      ..._deleteAction(state, context),
    ];
  }

  List<Widget> _deleteAction(SettingsState state, BuildContext context) {
    if (widget.filter == null) {
      return [];
    }

    final filter = widget.filter!;
    return [
      const SizedBox(width: 20),
      ElevatedButton.icon(
        icon: const Icon(Icons.remove_circle),
        label: const Text("Delete"),
        style: ButtonStyle(
          iconColor: WidgetStateProperty.all(Colors.redAccent),
        ),
        onPressed: () async {
          await showDialog(
            context: context,
            builder: (context) => AlertDialog(
              content: Text(
                "Are you sure you want to delete ${filter.name} filter?",
              ),
              actions: [
                IconButton(
                  icon: const Icon(Icons.cancel),
                  onPressed: () {
                    Navigator.pop(context);
                  },
                ),
                IconButton(
                  icon: const Icon(Icons.check),
                  onPressed: () {
                    var filters = state.settings.filters.toList();
                    filters.removeWhere(
                      (element) => element.uuid == filter.uuid,
                    );

                    if (state.settings.selectedFilter
                        case FilterSelection_Predefined(:final uuid)) {
                      if (uuid == filter.uuid) {
                        context
                            .read<TaskBloc>()
                            .add(TaskFilterEvent(filter: null));
                        context.read<SettingsBloc>().add(SettingsUpdateEvent(
                              settings: state.settings.copyWith(
                                selectedFilter: null,
                              ),
                            ));
                      }
                    }

                    context.read<SettingsBloc>().add(SettingsUpdateEvent(
                          settings: state.settings.copyWith(
                            filters: filters,
                          ),
                        ));

                    // Pop to first route.
                    Navigator.popUntil(
                      context,
                      (route) => route.isFirst,
                    );
                  },
                ),
              ],
            ),
          );
        },
      )
    ];
  }
}
