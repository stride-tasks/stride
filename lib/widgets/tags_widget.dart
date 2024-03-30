import 'package:flutter/material.dart';

class TagsWidget extends StatefulWidget {
  final List<String> tags;
  final List<String> currentItems;
  final void Function(List<String>) onSubmit;
  const TagsWidget({
    super.key,
    required this.tags,
    required this.onSubmit,
    this.currentItems = const [],
  });

  @override
  State<TagsWidget> createState() => TagsWidgetState();
}

class TagsWidgetState extends State<TagsWidget> {
  late List<String> items;

  @override
  void initState() {
    super.initState();
    items = widget.currentItems.toList();
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        TextField(
          decoration: const InputDecoration(
            labelText: "Tags",
          ),
          onSubmitted: (text) {
            setState(() {
              items.add(text.toLowerCase());
              widget.onSubmit(items);
            });
          },
        ),
        const SizedBox(height: 10),
        _chips(items),
      ],
    );
  }

  Widget _chips(List<String> items) {
    return Wrap(
      children: items
          .map(
            (e) => _chip(e, () {
              setState(() {
                items.remove(e);
              });
            }),
          )
          .map((e) => Padding(
                padding: const EdgeInsets.all(2),
                child: e,
              ))
          .toList(),
    );
  }

  Widget _chip(String text, void Function() onDelete) {
    return InputChip(
      label: Text(text),
      labelStyle: const TextStyle(
        fontSize: 14,
        fontWeight: FontWeight.bold,
      ),
      onDeleted: onDelete,
    );
  }
}
