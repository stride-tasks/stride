import 'package:flutter/material.dart';

class TagsWidget extends StatefulWidget {
  final Set<String> tags;
  final void Function(Set<String>) onSubmit;
  const TagsWidget({super.key, required this.tags, required this.onSubmit});

  @override
  State<TagsWidget> createState() => TagsWidgetState();
}

class TagsWidgetState extends State<TagsWidget> {
  late Set<String> items;
  late final TextEditingController _controller;

  @override
  void initState() {
    super.initState();
    items = widget.tags.toSet();
    _controller = TextEditingController();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  void _addTags(String raw) {
    final normalized = raw
        .split(',')
        .map(
          (value) => value.trim().replaceFirst(RegExp(r'^#'), '').toLowerCase(),
        )
        .where((value) => value.isNotEmpty)
        .toSet();

    if (normalized.isEmpty) {
      return;
    }

    setState(() {
      items.addAll(normalized);
      _controller.clear();
      widget.onSubmit(items);
    });
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        TextField(
          controller: _controller,
          textInputAction: TextInputAction.done,
          decoration: InputDecoration(
            hintText: 'Add tag, e.g. work, home',
            border: OutlineInputBorder(borderRadius: BorderRadius.circular(12)),
            contentPadding: const EdgeInsets.symmetric(
              horizontal: 12,
              vertical: 12,
            ),
            suffixIcon: IconButton(
              onPressed: () => _addTags(_controller.text),
              icon: const Icon(Icons.add_circle_outline_rounded),
              tooltip: 'Add tag',
            ),
            filled: true,
            fillColor: theme.colorScheme.surface,
          ),
          onSubmitted: _addTags,
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
              labelStyle: const TextStyle(
                fontSize: 12,
                fontWeight: FontWeight.w600,
              ),
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
