import 'package:flutter/material.dart';
import 'package:stride/bridge/api/settings.dart';
import 'package:stride/routes/ssh_keys_route.dart';
import 'package:stride/utils/functions.dart';
import 'package:uuid/uuid_value.dart';
import 'package:uuid/validation.dart';

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
  final Widget? title;
  final List<Widget> tiles;

  const SettingsSection({super.key, this.title, required this.tiles});

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        if (title != null) Row(children: [title!]),
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
      onTap: () => onTap?.call(context),
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
    super.trailing,
    required this.builder,
  }) : super(
         onTap: (context) {
           // ignore: inference_failure_on_instance_creation
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
         onTap: (context) {
           onChanged(value);
         },
       );
}

class SettingsTileText extends SettingsTile {
  SettingsTileText({
    super.key,
    required super.title,
    Widget? description,
    super.leading,
    Widget? trailing,
    required String text,
    required void Function(String text) onChanged,
    String? Function(String? text)? validator,
    bool hidden = false,
    bool multiline = false,
    bool obscureText = false,
  }) : super(
         trailing: trailing ?? const Icon(Icons.arrow_forward),
         description: hidden
             ? null
             : description != null || text.isEmpty
             ? description
             : Text(
                 obscureText ? text.replaceAll(RegExp('.'), '*') : text,
                 overflow: TextOverflow.ellipsis,
                 style: const TextStyle(fontSize: 12.0, color: Colors.grey),
               ),
         onTap: (context) async {
           final controller = TextEditingController(text: text);
           await showAlertDialog(
             context: context,
             popRoute: false,
             content: StatefulBuilder(
               builder: (context, setState) => TextFormField(
                 controller: controller,
                 autocorrect: false,
                 autofocus: true,
                 maxLines: multiline ? null : 1,
                 keyboardType: multiline ? TextInputType.multiline : null,
                 decoration: InputDecoration(
                   errorText: validator?.call(controller.text),
                 ),
                 validator: validator,
                 onChanged: (_) => setState(() {}),
               ),
             ),
             onConfirm: (context) async {
               if (validator?.call(controller.text) != null) {
                 return false;
               }
               onChanged(controller.text);
               Navigator.of(context).pop();
               return true;
             },
           );
         },
       );
}

String? uuidValidator(String? text) {
  if (text == null) {
    return 'empty UUID';
  }
  return UuidValidation.isValidUUID(fromString: text) ? null : 'Invalid UUID';
}

class SettingsTileUuid extends SettingsTile {
  final UuidValue? value;
  final void Function(UuidValue value) onChanged;
  SettingsTileUuid({
    super.key,
    required super.title,
    super.leading,
    required this.value,
    required this.onChanged,
  }) : super(
         trailing: const Icon(Icons.arrow_forward),
         description: value == null
             ? null
             : Text(
                 value.toString(),
                 overflow: TextOverflow.ellipsis,
                 style: const TextStyle(fontSize: 12.0, color: Colors.grey),
               ),
         onTap: (context) async {
           final controller = TextEditingController(text: value?.toString());
           await showAlertDialog(
             context: context,
             content: StatefulBuilder(
               builder: (context, setState) => TextFormField(
                 controller: controller,
                 autocorrect: false,
                 autofocus: true,
                 decoration: InputDecoration(
                   errorText: uuidValidator(controller.text),
                 ),
                 validator: uuidValidator,
                 onChanged: (_) => setState(() {}),
               ),
             ),
             onConfirm: (context) {
               try {
                 final uuid = UuidValue.withValidation(controller.text);
                 onChanged(uuid);
               } on FormatException {
                 return false;
               }
               return true;
             },
           );
         },
       );
}

class SettingsTileSsh extends SettingsTile {
  SettingsTileSsh({
    super.key,
    required super.title,
    super.leading,
    required UuidValue? uuid,
    required void Function(UuidValue value) onChanged,
    List<SshKey>? keys,
    Widget? trailing,
  }) : super(
         trailing: trailing ?? const Icon(Icons.arrow_forward),
         description: uuid == null
             ? null
             : Text(
                 keys
                         ?.cast<SshKey?>()
                         .firstWhere(
                           (element) => element!.uuid == uuid,
                           orElse: () => null,
                         )
                         ?.publicKey ??
                     'key not found!',
                 overflow: TextOverflow.ellipsis,
                 style: const TextStyle(fontSize: 12.0, color: Colors.grey),
               ),
         onTap: (context) async {
           await Navigator.of(context).push(
             MaterialPageRoute<SshKey>(
               builder: (context) => SshKeysRoute(
                 selected: uuid,
                 onTap: (key) {
                   onChanged(key.uuid);
                   Navigator.of(context).pop();
                 },
                 hasDelete: false,
               ),
             ),
           );
         },
       );
}
