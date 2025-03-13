import 'package:flutter/material.dart';

class IconTextButton extends StatelessWidget {
  final Icon icon;
  final String text;
  final VoidCallback onPressed;
  final TextStyle? textStyle;
  final String? tooltip;

  const IconTextButton({
    super.key,
    required this.icon,
    required this.text,
    required this.onPressed,
    this.textStyle,
    this.tooltip,
  });

  @override
  Widget build(BuildContext context) {
    Widget widget = ElevatedButton(
      onPressed: onPressed,
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          icon,
          Text(
            text,
            style: textStyle,
          ),
        ],
      ),
    );

    if (tooltip != null) {
      widget = Tooltip(
        message: tooltip!,
        preferBelow: false,
        child: widget,
      );
    }

    return widget;
  }
}
