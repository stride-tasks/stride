import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:stride/routes/initial_route.dart';

void main() {
  testWidgets('shows helpful actions when no repository is found', (
    tester,
  ) async {
    await tester.pumpWidget(const MaterialApp(home: InitialRoute()));

    expect(find.text('No repository found'), findsOneWidget);
    expect(
      find.text(
        'Create a repository or clone an existing Git repository to start '
        'organizing your tasks.',
      ),
      findsOneWidget,
    );
    expect(find.widgetWithText(FilledButton, 'New Repository'), findsOneWidget);
    expect(
      find.widgetWithText(OutlinedButton, 'Clone Repository'),
      findsOneWidget,
    );
  });

  testWidgets('new repository opens the creation wizard', (tester) async {
    await tester.pumpWidget(const MaterialApp(home: InitialRoute()));

    await tester.tap(find.widgetWithText(FilledButton, 'New Repository'));
    await tester.pumpAndSettle();

    expect(find.widgetWithText(AppBar, 'New Repository'), findsOneWidget);
    expect(find.text('Git Integration (optional)'), findsOneWidget);
  });

  testWidgets('clone repository opens the clone wizard', (tester) async {
    await tester.pumpWidget(const MaterialApp(home: InitialRoute()));

    await tester.tap(find.widgetWithText(OutlinedButton, 'Clone Repository'));
    await tester.pumpAndSettle();

    expect(find.widgetWithText(AppBar, 'Clone Repository'), findsOneWidget);
    expect(find.text('Encryption'), findsOneWidget);
  });
}
