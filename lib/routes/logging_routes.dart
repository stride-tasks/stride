import 'package:flutter/material.dart';
import 'package:stride/src/rust/api/logging.dart';

class LoggingRoute extends StatefulWidget {
  const LoggingRoute({super.key});

  @override
  State<LoggingRoute> createState() => _LoggingRouteState();
}

class _LoggingRouteState extends State<LoggingRoute> {
  Future<String>? future;

  @override
  void initState() {
    super.initState();

    future = getLogs();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text("Logging")),
      body: FutureBuilder(
        future: future,
        builder: (context, state) {
          if (state.connectionState != ConnectionState.done) {
            return const CircularProgressIndicator();
          }
          final data = state.data!;

          final lines = data.trim().split("\n");
          final length = lines.length;

          return Column(
            children: [
              Expanded(
                child: ListView.builder(
                  itemCount: length,
                  itemBuilder: (context, index) {
                    final parts = lines[length - 1 - index].split(": ");
                    final time = parts[0].split(" ")[0].replaceFirst("T", " ");
                    final level = parts[0].split(" ")[1];
                    final title = parts.sublist(1).join(": ");
                    var levelIcon = const Icon(Icons.question_mark);

                    // Log Levels: "OFF", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"
                    switch (level) {
                      case "DEBUG":
                        levelIcon = const Icon(Icons.bug_report);
                        break;
                      case "TRACE":
                        levelIcon = const Icon(Icons.track_changes);
                        break;
                      case "INFO":
                        levelIcon = const Icon(Icons.info);
                        break;
                      case "WARN":
                        levelIcon = const Icon(Icons.warning);
                        break;
                      case "ERROR":
                        levelIcon = const Icon(
                          Icons.error,
                          color: Colors.redAccent,
                        );
                        break;
                    }
                    return Card(
                      child: ListTile(
                        leading: levelIcon,
                        title: Text(title),
                        subtitle: Text(
                          time,
                          style: const TextStyle(
                            fontSize: kDefaultFontSize / 1.05,
                          ),
                        ),
                      ),
                    );
                  },
                ),
              ),
            ],
          );
        },
      ),
    );
  }
}
