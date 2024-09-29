import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
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

                    final copyButton = IconButton(
                      onPressed: () async {
                        final message = '$level: $title\n\n$body';
                        await Clipboard.setData(ClipboardData(text: message));
                        if (context.mounted) {
                          ScaffoldMessenger.of(context).showSnackBar(
                            const SnackBar(
                              content: Text('Copied to clipbard!'),
                            ),
                          );
                        }
                      },
                      icon: const Icon(Icons.copy),
                    );

                    return Card(
                      // SelectionArea selection removes newlines.
                      // See: https://github.com/flutter/flutter/issues/104548
                      child: SelectionArea(
                        child: LogEntry(
                          body: body,
                          levelIcon: levelIcon,
                          copyButton: copyButton,
                          title: title,
                          subtitle: subtitle,
                          isInitialExpanded: isFirstTile,
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

class LogEntry extends StatefulWidget {
  const LogEntry({
    super.key,
    required this.body,
    required this.levelIcon,
    required this.copyButton,
    required this.title,
    required this.subtitle,
    required this.isInitialExpanded,
  });

  final String body;
  final Icon levelIcon;
  final IconButton copyButton;
  final String title;
  final Text subtitle;
  final bool isInitialExpanded;

  @override
  State<LogEntry> createState() => _LogEntryState();
}

class _LogEntryState extends State<LogEntry> {
  bool _isExpanded = false;

  @override
  void initState() {
    super.initState();

    _isExpanded = widget.isInitialExpanded;
  }

  @override
  Widget build(BuildContext context) {
    return ExpansionTile(
      leading: widget.levelIcon,
      title: Text(widget.title),
      trailing: _isExpanded ? widget.copyButton : null,
      onExpansionChanged: (value) => setState(() => _isExpanded = value),
      subtitle: SelectionContainer.disabled(child: widget.subtitle),
      initiallyExpanded: _isExpanded,
      expandedAlignment: Alignment.topLeft,
      childrenPadding: const EdgeInsets.all(8.0),
      children: [if (widget.body.isNotEmpty) Text(widget.body)],
    );
  }
}
