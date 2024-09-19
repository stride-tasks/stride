import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
// import 'package:stride/main.dart';
import 'package:stride/bridge/frb_generated.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  setUpAll(() async => RustLib.init());
  testWidgets('Can call rust function', (tester) async {
    // await tester.pumpWidget(const MyApp());
    // expect(find.textContaining('Result: `Hello, Tom!`'), findsOneWidget);
  });
}
