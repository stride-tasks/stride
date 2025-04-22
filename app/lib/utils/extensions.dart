extension ExtDateTime on DateTime {
  String toHumanString() {
    // NOTE: Stride stores dates in UTC, so we must convert to local before showing to the user.
    return toLocal().toString().substring(0, 16);
  }
}
