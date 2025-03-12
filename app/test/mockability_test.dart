import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';
import 'package:stride/bridge/frb_generated.dart';

// Surely, you can use Mockito or whatever other mocking packages
// ignore: unreachable_from_main
class MockRustLibApi extends Mock implements RustLibApi {}

Future<void> main() async {
  // TODO: This doesn't seem to work!

  // final mockApi = MockRustLibApi();
  // // when(() => mockApi.initApp()).thenAnswer((_) async {});
  // //
  // await RustLib.init(api: mockApi);

  // test('Can mock Rust calls', () async {
  //   when(() => mockApi.greet(name: "Haled")).thenAnswer((_) => "Hello, Haled");

  //   // final actualResult = greet(name: "Haled");
  //   // expect(actualResult, isNot(3));
  //   // expect(actualResult, equals(123456789));
  //   verifyNever(() => mockApi.greet(name: "Haled"));
  // });

  test('dummy test', () {});
}
