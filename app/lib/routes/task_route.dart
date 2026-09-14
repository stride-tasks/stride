import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/plugin_manager_bloc.dart';
import 'package:stride/blocs/tasks_bloc.dart';
import 'package:stride/bridge/third_party/stride_core/event.dart';
import 'package:stride/bridge/third_party/stride_core/task.dart';
import 'package:stride/bridge/third_party/stride_core/task/annotation.dart';
import 'package:stride/bridge/third_party/stride_core/task/uda.dart';
import 'package:stride/context.dart';
import 'package:stride/utils/extensions.dart';
import 'package:stride/utils/functions.dart';
import 'package:stride/widgets/tags_widget.dart';
import 'package:uuid/uuid.dart';

class TaskRoute extends StatefulWidget {
  final Task? task;
  const TaskRoute({super.key, this.task});

  @override
  State<TaskRoute> createState() => _TaskRouteState();
}

class _TaskRouteState extends State<TaskRoute> {
  String title = '';
  DateTime? entry;
  DateTime? due;
  String? project;
  Set<String> _tags = {};
  List<(UuidValue, DateTime, TextEditingController)> annotations = [];
  List<UuidValue> depends = [];
  TaskPriority? priority;
  List<Uda> udas = [];

  Future<Set<String>>? availableTags;
  Future<Set<String>>? availableProjects;

  final _formKey = GlobalKey<FormState>();

  @override
  void initState() {
    super.initState();

    title = widget.task?.title ?? title;
    entry = widget.task?.entry;
    due = widget.task?.due;
    project = widget.task?.project;
    _tags = widget.task?.tags.toSet() ?? _tags;
    annotations =
        widget.task?.annotations
            .map(
              (annotation) => (
                annotation.id,
                annotation.entry,
                TextEditingController(text: annotation.text),
              ),
            )
            .toList() ??
        annotations;
    depends = widget.task?.depends.toList() ?? depends;
    priority = widget.task?.priority;
    udas = widget.task?.udas ?? udas;

    availableTags = _getAvailableTags();
    availableProjects = _getAvailableProjects();
  }

  Future<Set<String>> _getAvailableTags() async {
    final taskBloc = context.read<TaskBloc>();
    final repositoryUuid =
        taskBloc.repositoryUuid ??
        taskBloc.settingsBloc.settings.currentRepository;
    if (repositoryUuid == null) {
      return const {};
    }

    try {
      final response = await RustContext.execute(
        'stride.repository.tag.list',
        jsonEncode({
          'params': {'id': repositoryUuid.toString()},
        }),
      );
      final data = jsonDecode(response) as Map<String, dynamic>;
      return (data['tags'] as List? ?? const [])
          .map((entry) => entry['id'] as String?)
          .whereType<String>()
          .toSet();
    } catch (_) {
      return const {};
    }
  }

  Future<Set<String>> _getAvailableProjects() async {
    final taskBloc = context.read<TaskBloc>();
    final repositoryUuid =
        taskBloc.repositoryUuid ??
        taskBloc.settingsBloc.settings.currentRepository;
    if (repositoryUuid == null) {
      return const {};
    }

    try {
      final response = await RustContext.execute(
        'stride.repository.project.list',
        jsonEncode({
          'params': {'id': repositoryUuid.toString()},
        }),
      );
      final data = jsonDecode(response) as Map<String, dynamic>;
      return (data['projects'] as List? ?? const [])
          .map((entry) => entry['id'] as String?)
          .whereType<String>()
          .toSet();
    } catch (_) {
      return const {};
    }
  }

  String _dueButtonText() {
    if (due == null) {
      return 'Select';
    } else {
      return due!.toHumanString();
    }
  }

  @override
  Widget build(BuildContext context) {
    final isEditing = widget.task != null;

    return Scaffold(
      appBar: AppBar(title: Text(isEditing ? 'Edit task' : 'New task')),
      body: SafeArea(child: _buildForm()),
      floatingActionButton: FloatingActionButton(
        onPressed: _saveTask,
        shape: const CircleBorder(),
        child: isEditing
            ? const Icon(Icons.check_rounded)
            : const Icon(Icons.add_rounded),
      ),
    );
  }

  Widget _buildForm() {
    return Center(
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 760),
        child: Padding(
          padding: const .all(12),
          child: SingleChildScrollView(
            child: Form(
              key: _formKey,
              child: Column(
                crossAxisAlignment: .stretch,
                children: [
                  _buildTitleField(),
                  const SizedBox(height: 10),
                  _buildDueSection(),
                  const SizedBox(height: 10),
                  _buildPrioritySection(),
                  const SizedBox(height: 10),
                  _buildProjectSection(),
                  const SizedBox(height: 10),
                  _buildTagsSection(),
                  const SizedBox(height: 10),
                  _buildAnnotations(),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildTitleField() {
    return _sectionCard(
      child: TextFormField(
        initialValue: title,
        autofocus: true,
        decoration: const InputDecoration(
          border: .none,
          hintText: 'What needs to be done?',
          prefixIcon: Icon(Icons.task_alt_outlined),
          contentPadding: .symmetric(vertical: 4),
        ),
        validator: (value) {
          if (value == null || value.isEmpty) {
            return 'Task must have a description';
          }
          return null;
        },
        onSaved: (newValue) {
          title = newValue!;
        },
        textCapitalization: .sentences,
        autovalidateMode: .onUserInteraction,
      ),
    );
  }

  Widget _buildDueSection() {
    final theme = Theme.of(context);

    return _sectionCard(
      child: Row(
        children: [
          Icon(
            Icons.event_available_rounded,
            size: 18,
            color: theme.colorScheme.primary,
          ),
          const SizedBox(width: 8),
          Expanded(
            child: Row(
              children: [
                Expanded(
                  child: Text(
                    'Due',
                    style: theme.textTheme.titleSmall?.copyWith(
                      fontWeight: .w600,
                    ),
                  ),
                ),
                Flexible(
                  child: TextButton(
                    onPressed: _pickDueDate,
                    child: Text(_dueButtonText()),
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  List<String> _projectSuggestions(String raw, Set<String> availableProjects) {
    final normalized = raw.trim().toLowerCase();
    final available =
        availableProjects
            .where((value) => value.trim().isNotEmpty)
            .where(
              (value) =>
                  normalized.isEmpty ||
                  value.toLowerCase().contains(normalized),
            )
            .toList()
          ..sort();
    return available;
  }

  Widget _buildProjectSection() {
    final theme = Theme.of(context);

    return _sectionCard(
      child: FutureBuilder<Set<String>>(
        future: availableProjects,
        builder: (context, snapshot) {
          final availableProjects = snapshot.data ?? const <String>{};
          return Row(
            children: [
              Tooltip(
                message: 'Project',
                child: Icon(
                  Icons.folder_open_rounded,
                  size: 18,
                  color: theme.colorScheme.primary,
                ),
              ),
              const SizedBox(width: 8),
              Expanded(
                child: Autocomplete<String>(
                  initialValue: TextEditingValue(text: project ?? ''),
                  optionsBuilder: (value) =>
                      _projectSuggestions(value.text, availableProjects),
                  onSelected: (value) {
                    setState(() {
                      project = value;
                    });
                  },
                  fieldViewBuilder:
                      (context, controller, focusNode, onFieldSubmitted) {
                        final currentValue = project ?? '';
                        if (controller.text != currentValue) {
                          controller.text = currentValue;
                        }
                        return TextFormField(
                          controller: controller,
                          focusNode: focusNode,
                          decoration: const InputDecoration(
                            border: .none,
                            hintText: 'Project',
                            contentPadding: .symmetric(vertical: 4),
                          ),
                          onChanged: (newValue) {
                            project = newValue.trim().isEmpty
                                ? null
                                : newValue.trim();
                          },
                          onSaved: (newValue) {
                            project =
                                newValue == null || newValue.trim().isEmpty
                                ? null
                                : newValue.trim();
                          },
                        );
                      },
                ),
              ),
            ],
          );
        },
      ),
    );
  }

  Widget _buildPrioritySection() {
    final theme = Theme.of(context);

    return _sectionCard(
      child: Row(
        children: [
          Icon(Icons.flag_rounded, size: 18, color: theme.colorScheme.primary),
          const SizedBox(width: 8),
          Expanded(
            child: SegmentedButton<TaskPriority>(
              segments: const <ButtonSegment<TaskPriority>>[
                ButtonSegment<TaskPriority>(
                  value: TaskPriority.h,
                  icon: Icon(Icons.priority_high),
                  label: Text('High'),
                ),
                ButtonSegment<TaskPriority>(
                  value: TaskPriority.m,
                  icon: Icon(Icons.density_medium),
                  label: Text('Medium'),
                ),
                ButtonSegment<TaskPriority>(
                  value: TaskPriority.l,
                  icon: Icon(Icons.low_priority),
                  label: Text('Low'),
                ),
              ],
              selected: priority == null ? {} : {priority!},
              onSelectionChanged: (newSelection) {
                setState(() {
                  priority = newSelection.isEmpty ? null : newSelection.first;
                });
              },
              emptySelectionAllowed: true,
              selectedIcon: const Icon(Icons.check),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildTagsSection() {
    final theme = Theme.of(context);

    return _sectionCard(
      child: FutureBuilder<Set<String>>(
        future: availableTags,
        builder: (context, snapshot) {
          final availableTags = snapshot.data ?? const <String>{};
          return Column(
            crossAxisAlignment: .start,
            children: [
              Row(
                children: [
                  Icon(
                    Icons.sell_rounded,
                    size: 18,
                    color: theme.colorScheme.primary,
                  ),
                  const SizedBox(width: 8),
                  Text(
                    'Tags',
                    style: theme.textTheme.titleSmall?.copyWith(
                      fontWeight: .w600,
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 10),
              TagsWidget(
                tags: _tags,
                availableTags: availableTags,
                onSubmit: (tags) => _tags = tags,
              ),
            ],
          );
        },
      ),
    );
  }

  Future<void> _pickDueDate() async {
    final datetime = await showPickDateTime(context: context);
    setState(() {
      due = datetime;
    });
  }

  Future<void> _saveTask() async {
    if (!_formKey.currentState!.validate()) {
      return;
    }

    _formKey.currentState!.save();

    if (!context.mounted) {
      return;
    }

    final task = Task.raw(
      id: widget.task?.id ?? UuidValue.fromString(const Uuid().v7()),
      entry: entry ?? DateTime.now().toUtc(),
      title: title,
      project: project,
      tags: _tags.toList(),
      due: due,
      status: .pending,
      annotations: annotations
          .map<Annotation>(
            (annotation) => Annotation(
              id: annotation.$1,
              entry: annotation.$2,
              text: annotation.$3.text,
            ),
          )
          .toList(),
      depends: depends,
      priority: priority,
      udas: udas,
    );

    if (widget.task == null) {
      context.read<TaskBloc>().add(TaskAddEvent(task: task));
      context.read<PluginManagerBloc>().emitHostEvent(
        HostEvent.taskCreate(task: task),
      );
    } else {
      context.read<TaskBloc>().add(
        TaskUpdateEvent(current: task, previous: widget.task),
      );
      context.read<PluginManagerBloc>().emitHostEvent(
        HostEvent.taskModify(current: task, previous: widget.task),
      );
    }

    if (context.mounted) {
      Navigator.pop(context);
    }
  }

  Widget _sectionCard({required Widget child}) {
    return Container(
      padding: const .symmetric(horizontal: 8, vertical: 6),
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.surfaceContainerLow,
        borderRadius: .circular(8),
        border: .all(
          color: Theme.of(context).colorScheme.outlineVariant,
          width: 1.0,
        ),
      ),
      child: child,
    );
  }

  Widget _buildAnnotations() {
    final theme = Theme.of(context);

    return _sectionCard(
      child: Column(
        crossAxisAlignment: .start,
        children: [
          Row(
            children: [
              Icon(
                Icons.note_alt_outlined,
                size: 18,
                color: theme.colorScheme.primary,
              ),
              const SizedBox(width: 8),
              Text(
                'Annotations',
                style: theme.textTheme.titleSmall?.copyWith(fontWeight: .w600),
              ),
              const Spacer(),
              IconButton(
                onPressed: () => setState(() {
                  annotations.add((
                    UuidValue.fromString(Uuid().v7()),
                    DateTime.now().toUtc(),
                    TextEditingController(),
                  ));
                }),
                icon: const Icon(Icons.add_circle_outline_rounded),
                tooltip: 'Add annotation',
                visualDensity: .compact,
              ),
            ],
          ),
          const SizedBox(height: 8),
          if (annotations.isNotEmpty) ...annotations.map(_buildAnnotationRow),
        ],
      ),
    );
  }

  Widget _buildAnnotationRow(
    (UuidValue, DateTime, TextEditingController) annotation,
  ) {
    final theme = Theme.of(context);

    return Padding(
      padding: const .only(top: 8),
      child: Row(
        children: [
          Expanded(
            child: Container(
              padding: const .symmetric(horizontal: 10, vertical: 8),
              decoration: BoxDecoration(
                color: theme.colorScheme.surface,
                borderRadius: .circular(10),
              ),
              child: Row(
                children: [
                  Expanded(
                    child: TextField(
                      controller: annotation.$3,
                      decoration: InputDecoration(
                        border: .none,
                        hintText: 'Note',
                        isDense: true,
                        contentPadding: .zero,
                      ),
                    ),
                  ),
                  const SizedBox(width: 8),
                  Text(
                    annotation.$2.toHumanString(),
                    style: theme.textTheme.labelSmall?.copyWith(
                      color: theme.colorScheme.onSurfaceVariant,
                    ),
                  ),
                ],
              ),
            ),
          ),
          const SizedBox(width: 8),
          IconButton(
            onPressed: () => setState(() {
              annotations.remove(annotation);
            }),
            icon: const Icon(Icons.remove_circle_outline_rounded),
            tooltip: 'Remove annotation',
            visualDensity: .compact,
          ),
        ],
      ),
    );
  }
}
