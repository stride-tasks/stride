import 'package:flutter/material.dart';

class TagsWidget extends StatefulWidget {
  final List<String> tags;
  final void Function(List<String>) onSubmit;
  const TagsWidget({
    super.key,
    required this.tags,
    required this.onSubmit,
  });

  @override
  State<TagsWidget> createState() => TagsWidgetState();
}

class TagsWidgetState extends State<TagsWidget> {
  late List<String> items;

  @override
  void initState() {
    super.initState();
    items = widget.tags.toList();
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        TextField(
          decoration: const InputDecoration(
            labelText: 'Tags',
          ),
          onSubmitted: (text) {
            setState(() {
              items.add(text.toLowerCase());
              widget.onSubmit(items);
            });
          },
        ),
        const SizedBox(height: 10),
        _chips(),
      ],
    );
  }

  Widget _chips() {
    return Wrap(
      children: items
          .map(
            (tag) => Padding(
              padding: const EdgeInsets.symmetric(horizontal: 2),
              child: _chip(tag),
            ),
          )
          .toList(),
    );
  }

  Widget _chip(String text) {
    return InputChip(
      label: Text(text),
      labelStyle: const TextStyle(
        fontSize: 12,
        fontWeight: FontWeight.bold,
      ),
      onDeleted: () => setState(() {
        items.remove(text);
        widget.onSubmit(items);
      }),
    );
  }
}
