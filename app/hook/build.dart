import 'package:code_assets/code_assets.dart';
import 'package:hooks/hooks.dart';
import 'package:native_toolchain_rust/native_toolchain_rust.dart';

void main(List<String> args) async {
  await build(args, (input, output) async {
    // input.buildMode;
    if (input.config.buildCodeAssets) {
      final packageName = input.packageName;
      final rustBuilder = RustBuilder(
        assetName: '$packageName.dart',
        cratePath: '${input.packageRoot.path}/../crates/flutter_bridge',
        buildMode: BuildMode.debug,
      );

      await rustBuilder.run(input: input, output: output);
    }
  });
}
