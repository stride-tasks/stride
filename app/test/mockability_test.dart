import 'package:flutter_test/flutter_test.dart';

import 'package:stride/api.dart';
import 'package:stride/background.dart';

Future<void> main() async {
  test('Output can round-trip through isolate-safe map data', () {
    final original = Output(
      name: const Name(method: 'task.sync', unique: 'repo-123'),
      inputData: {'repository': 'repo-123'},
      timestamp: DateTime.utc(2026, 8, 13, 12, 0, 0),
      done: true,
    );

    final roundTrip = Output.fromMap(original.toMap());

    expect(roundTrip, equals(original));
    expect(roundTrip.name, equals(original.name));
    expect(roundTrip.inputData, equals(original.inputData));
    expect(roundTrip.done, isTrue);
  });

  test('Task deserializes dependency UUIDs from JSON lists', () {
    final task = Task.fromJson({
      'id': '019b9b4a-0000-7000-8000-000000000001',
      'depends': const <String>[],
    });

    expect(task.depends, isEmpty);
  });
}
