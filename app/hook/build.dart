import 'dart:io';

import 'package:hooks/hooks.dart';
import 'package:logging/logging.dart';
import 'package:native_toolchain_rust/native_toolchain_rust.dart';

void main(List<String> args) async {
  await build(args, (input, output) async {
    final logger = Logger.detached('RustBuildHook')
      ..level = Level.ALL
      ..onRecord.listen((record) {
        void writeLog(Object? message) {
          if (record.level >= Level.WARNING) {
            stderr.writeln(message);
          } else {
            stdout.writeln(message);
          }
        }

        final message = record.level == Level.CONFIG
            ? '[RustBuildHook] Build configuration captured (environment omitted).'
            : '[RustBuildHook] ${record.message}';
        writeLog(message);
        if (record.error != null) {
          writeLog(record.error);
        }
        if (record.stackTrace != null) {
          writeLog(record.stackTrace);
        }
      });

    try {
      await RustBuilder(
        assetName: 'src/stride_flutter_bridge.g.dart',
        cratePath: '../crates/flutter_bridge',
        buildMode: BuildMode.release,
        // @note ensures that Cargo uses the locked versions of dependencies.
        extraCargoBuildArgs: ['--locked'],
      ).run(input: input, output: output, logger: logger);
    } catch (error, stackTrace) {
      throw BuildError(
        message: 'Rust native asset build failed.',
        wrappedException: error,
        wrappedTrace: stackTrace,
      );
    }
  });
}
