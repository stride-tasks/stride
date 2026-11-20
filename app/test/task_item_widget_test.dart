import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:stride/api.dart';
import 'package:stride/widgets/task_item_widget.dart';
import 'package:uuid/uuid.dart';

void main() {
  testWidgets('annotations do not change collapsed item height', (
    tester,
  ) async {
    // ignore: use_named_constants
    const id = UuidValue.fromNamespace(Namespace.nil);
    final task = Task(id: id, title: 'test', depends: const []);

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: Column(
            children: [
              TaskItem(key: const Key('plain'), task: task),
              TaskItem(
                key: const Key('annotated'),
                task: task.copyWith(
                  annotations: [
                    TaskAnnotation(
                      id: id,
                      entry: DateTime.utc(2026),
                      text: 'annotation',
                    ),
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
    );

    expect(
      tester.getSize(find.byKey(const Key('annotated'))).height,
      tester.getSize(find.byKey(const Key('plain'))).height,
    );
  });
}
