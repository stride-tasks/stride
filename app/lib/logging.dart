import 'package:stride/api.dart';
import 'package:stride/context.dart';

final class Log {
  static Future<String> getLogs() async {
    return LogGetMethod().execute().then((result) => result.content);
  }

  static Future<void> event({
    required LogLevel level,
    required String message,
  }) async {
    await LogEventMethod(level: level, message: message).execute();
  }

  static Future<void> debug(String message) async {
    await event(level: LogLevel.debug, message: message);
  }

  static Future<void> trace(String message) async {
    await event(level: LogLevel.trace, message: message);
  }

  static Future<void> info(String message) async {
    await event(level: LogLevel.info, message: message);
  }

  static Future<void> warn(String message) async {
    await event(level: LogLevel.warn, message: message);
  }

  static Future<void> error(String message) async {
    await event(level: LogLevel.error, message: message);
  }
}
