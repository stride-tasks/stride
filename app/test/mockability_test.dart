import 'package:flutter_test/flutter_test.dart';

import 'package:stride/api.dart';
import 'package:stride/background.dart';

Future<void> main() async {
  test('Repository sync status round-trips through service data', () {
    const original = RepositorySyncStatus(
      repositoryId: 'repo-123',
      syncing: false,
      error: 'sync failed',
    );

    final roundTrip = RepositorySyncStatus.fromMap(original.toMap());

    expect(roundTrip.repositoryId, original.repositoryId);
    expect(roundTrip.syncing, original.syncing);
    expect(roundTrip.error, original.error);
  });

  test('Task deserializes dependency UUIDs from JSON lists', () {
    final task = Task.fromJson({
      'id': '019b9b4a-0000-7000-8000-000000000001',
      'depends': const <String>[],
    });

    expect(task.depends, isEmpty);
  });
}
