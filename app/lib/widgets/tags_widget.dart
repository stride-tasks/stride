import 'package:flutter/material.dart';

class TagsWidget extends StatefulWidget {
  final Set<String> tags;
  final Set<String> availableTags;
  final void Function(Set<String>) onSubmit;

  const TagsWidget({
    super.key,
    required this.tags,
    required this.availableTags,
    required this.onSubmit,
  });

  @override
  State<TagsWidget> createState() => TagsWidgetState();
}

class TagsWidgetState extends State<TagsWidget> {
  late Set<String> items;
  late TextEditingController _tagController;

  @override
  void initState() {
    super.initState();
    items = widget.tags.toSet();
    _tagController = TextEditingController();
  }

  @override
  void dispose() {
    _tagController.dispose();
    super.dispose();
  }

  String _normalizeTag(String value) =>
      value.trim().replaceFirst(RegExp('^#'), '').toLowerCase();

  List<String> _suggestions(String raw) {
    final normalized = _normalizeTag(raw);
    final available =
        widget.availableTags
            .where((tag) => !items.contains(tag))
            .where((tag) => normalized.isEmpty || tag.contains(normalized))
            .toList()
          ..sort();
    return available;
  }

  void _selectTag(String value) {
    final normalized = _normalizeTag(value);
    if (normalized.isEmpty) {
      return;
    }

    setState(() {
      items.add(normalized);
      widget.onSubmit(items);
    });
    _tagController.clear();
  }

  void _addTags(String raw) {
    final normalized = raw
        .split(',')
        .map(_normalizeTag)
        .where((value) => value.isNotEmpty)
        .toSet();

    if (normalized.isEmpty) {
      return;
    }

    setState(() {
      items.addAll(normalized);
      widget.onSubmit(items);
    });
    _tagController.clear();
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Autocomplete<String>(
          optionsBuilder: (value) => _suggestions(value.text),
          onSelected: _selectTag,
          fieldViewBuilder: (context, controller, focusNode, onFieldSubmitted) {
            _tagController = controller;
            return TextField(
              controller: controller,
              focusNode: focusNode,
              textInputAction: .done,
              decoration: InputDecoration(
                hintText: 'Add tag, e.g. work, home',
                border: OutlineInputBorder(borderRadius: .circular(12)),
                contentPadding: const .symmetric(horizontal: 12, vertical: 12),
                suffixIcon: IconButton(
                  onPressed: () {
                    _addTags(controller.text);
                    controller.clear();
                  },
                  icon: const Icon(Icons.add_circle_outline_rounded),
                  tooltip: 'Add tag',
                ),
                filled: true,
                fillColor: theme.colorScheme.surface,
              ),
              onSubmitted: (_) {
                _addTags(controller.text);
                controller.clear();
              },
            );
          },
        ),
        const SizedBox(height: 12),
        if (items.isNotEmpty) _chips(),
      ],
    );
  }

  Widget _chips() {
    return Wrap(
      spacing: 8,
      runSpacing: 8,
      children: items
          .map(
            (tag) => InputChip(
              label: Text(tag),
              labelStyle: const TextStyle(fontSize: 12, fontWeight: .w600),
              onDeleted: () => setState(() {
                items.remove(tag);
                widget.onSubmit(items);
              }),
              backgroundColor: Theme.of(context).colorScheme.primaryContainer,
              deleteIconColor: Theme.of(context).colorScheme.onPrimaryContainer,
              side: BorderSide.none,
              avatar: const Icon(Icons.sell_rounded, size: 14),
            ),
          )
          .toList(),
    );
  }
}
