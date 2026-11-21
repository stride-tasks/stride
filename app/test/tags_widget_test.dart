import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:stride/widgets/tags_widget.dart';

void main() {
  testWidgets('can be removed without disposing an internal controller twice', (
    tester,
  ) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: TagsWidget(
            tags: const {},
            availableTags: const {'work'},
            onSubmit: (_) {},
          ),
        ),
      ),
    );

    await tester.pumpWidget(const MaterialApp(home: SizedBox()));

    expect(tester.takeException(), isNull);
  });
}
