import 'package:flutter/material.dart';
import 'package:stride/bridge/api/logging.dart';

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
      appBar: AppBar(title: const Text('Logging')),
      body: FutureBuilder(
        future: future,
        builder: (context, state) {
          if (state.connectionState != ConnectionState.done) {
            return const CircularProgressIndicator();
          }
          final data = state.data!.trim();

          if (data.isEmpty) {
            return const Column();
          }

          final lines = data.split('\n');
          final length = lines.length;

          return Column(
            children: [
              Expanded(
                child: ListView.builder(
                  itemCount: length,
                  itemBuilder: (context, index) {
                    final parts = lines[length - 1 - index].split(': ');
                    final time = parts[0].split(' ')[0].replaceFirst('T', ' ');
                    final level = parts[0].split(' ')[1];
                    final message = parts.sublist(1).join(': ');

                    // Log Levels: "OFF", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"
                    var levelIcon = const Icon(Icons.question_mark);
                    switch (level) {
                      case 'DEBUG':
                        levelIcon = const Icon(Icons.bug_report);
                      case 'TRACE':
                        levelIcon = const Icon(Icons.track_changes);
                      case 'INFO':
                        levelIcon = const Icon(Icons.info);
                      case 'WARN':
                        levelIcon = const Icon(Icons.warning);
                      case 'ERROR':
                        levelIcon = const Icon(
                          Icons.error,
                          color: Colors.redAccent,
                        );
                    }

                    final bodyMessageIndex = message.indexOf(r'\n');
                    var title = message;
                    var body = '';
                    if (bodyMessageIndex != -1) {
                      title = message.substring(0, bodyMessageIndex);
                      // NOTE: +2 skip over '\\n'.
                      body = message
                          .substring(bodyMessageIndex + 2)
                          .replaceAll(r'\n', '\n')
                          .trim();
                    }

                    final subtitle = Text(
                      time,
                      style: const TextStyle(fontSize: kDefaultFontSize / 1.05),
                    );

                    final isFirstTile = index == 0;

                    return Card(
                      child: body.isEmpty
                          ? ListTile(
                              leading: levelIcon,
                              title: Text(title),
                              subtitle: subtitle,
                            )
                          : ExpansionTile(
                              leading: levelIcon,
                              title: Text(title),
                              subtitle: subtitle,
                              initiallyExpanded: isFirstTile,
                              expandedAlignment: Alignment.topLeft,
                              childrenPadding: const EdgeInsets.all(8.0),
                              children: [Text(body)],
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
