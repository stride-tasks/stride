import 'package:stride/bridge/api/context.dart' as context;

class RustContext {
  static late Stream<String> _stream;

  static Stream<String> stream() => _stream;

  static Future<void> init() async {
    _stream = context.createContext().asBroadcastStream();
  }

  static Future<void> execute(String method, String args) async {
    return context.execute(method: method, args: args);
  }
}
