import 'package:flutter_test/flutter_test.dart';

import 'package:stride/api.dart';

Future<void> main() async {
  test('Task deserializes dependency UUIDs from JSON lists', () {
    final task = Task.fromJson({
      'id': '019b9b4a-0000-7000-8000-000000000001',
      'depends': const <String>[],
    });

    expect(task.depends, isEmpty);
  });
}
