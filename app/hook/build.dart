import 'package:hooks/hooks.dart';
import 'package:native_toolchain_rust/native_toolchain_rust.dart';

void main(List<String> args) async {
  await build(args, (input, output) async {
    await RustBuilder(
      assetName: 'src/stride_flutter_bridge.g.dart',
      cratePath: '../crates/flutter_bridge',
      extraCargoBuildArgs: ['--locked'],
    ).run(input: input, output: output);
  });
}
