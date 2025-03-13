extension ExtDateTime on DateTime {
  String toHumanString() {
    return toString().substring(0, 16);
  }
}
