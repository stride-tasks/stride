import 'package:flutter/material.dart';

ThemeData generateTheme({required bool darkMode}) {
  return ThemeData(
    brightness: darkMode ? Brightness.dark : Brightness.light,
    useMaterial3: true,
  );
}
