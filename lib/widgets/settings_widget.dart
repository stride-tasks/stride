import 'package:flutter/material.dart';

Iterable<T> insertBetween<T>(Iterable<T> iterable, T element) sync* {
  final iterator = iterable.iterator;
  if (iterator.moveNext()) {
    yield iterator.current;
    while (iterator.moveNext()) {
      yield element;
      yield iterator.current;
    }
  }
}

class SettingsList extends StatelessWidget {
  final List<SettingsSection> sections;

  const SettingsList({super.key, required this.sections});

  @override
  Widget build(BuildContext context) {
    return SingleChildScrollView(
      child: Container(
        padding: const EdgeInsets.all(16),
        alignment: Alignment.center,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: insertBetween(
            sections,
            const SizedBox(height: 10),
          ).toList(),
        ),
      ),
    );
  }
}

class SettingsSection extends StatelessWidget {
  final Widget title;
  final List<SettingsTile> tiles;

  const SettingsSection({
    super.key,
    required this.title,
    required this.tiles,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.start,
          children: [title],
        ),
        ...insertBetween(tiles, const Divider()),
      ],
    );
  }
}

class SettingsTile extends StatelessWidget {
  final Widget? leading;
  final Widget? trailing;
  final Widget? title;
  final Widget? description;
  final void Function(BuildContext context)? onTap;

  const SettingsTile({
    required this.title,
    this.leading,
    this.description,
    this.onTap,
    this.trailing = const Icon(Icons.arrow_forward),
    super.key,
  });

  @override
  Widget build(BuildContext context) {
    return ListTile(
      leading: leading,
      title: title,
      subtitle: description,
      trailing: trailing,
      onTap: () {
        if (onTap != null) onTap!(context);
      },
    );
  }
}

class SettingsTileNavigation extends SettingsTile {
  final Widget Function(BuildContext context) builder;
  SettingsTileNavigation({
    super.key,
    required super.title,
    super.description,
    super.leading,
    required this.builder,
  }) : super(
          onTap: (context) {
            Navigator.of(context).push(MaterialPageRoute(builder: builder));
          },
        );
}

class SettingsTileSwitch extends SettingsTile {
  final bool value;
  final void Function(bool value) onChanged;
  SettingsTileSwitch({
    super.key,
    required super.title,
    super.description,
    super.leading,
    required this.value,
    required this.onChanged,
  }) : super(
          trailing: Switch(
            value: value,
            activeColor: Colors.redAccent,
            onChanged: onChanged,
          ),
        );
}

class SettingsTileText extends SettingsTile {
  final String text;
  final bool hidden;
  final bool multiline;
  final void Function(String text) onChanged;

  SettingsTileText({
    super.key,
    required super.title,
    super.description,
    super.leading,
    required this.text,
    required this.onChanged,
    this.hidden = false,
    this.multiline = false,
  }) : super(
          trailing: hidden ? const Icon(Icons.arrow_forward) : Text(text),
          onTap: (context) async {
            var controller = TextEditingController(text: text);
            await showDialog(
              context: context,
              builder: (context) => AlertDialog(
                content: TextField(
                  controller: controller,
                  autocorrect: false,
                  autofocus: true,
                  maxLines: multiline ? null : 1,
                  keyboardType: multiline ? TextInputType.multiline : null,
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
                      onChanged(controller.text);
                      Navigator.pop(context);
                    },
                  ),
                ],
              ),
            );
          },
        );
}
