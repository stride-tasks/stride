import 'package:flutter/material.dart';
import 'package:stride/bridge/api/repository.dart';

class CommitsRoute extends StatefulWidget {
  final TaskStorage repository;
  final int chunkSize;
  const CommitsRoute({
    super.key,
    required this.repository,
    this.chunkSize = 50,
  });

  @override
  State<CommitsRoute> createState() => _CommitsRouteState();
}

class _CommitsRouteState extends State<CommitsRoute> {
  final _scrollController = ScrollController();

  Future<bool>? _future;
  List<CommitItem> _commits = [];
  Oid? _nextCommit;
  bool _endOfCommits = false;

  @override
  void initState() {
    super.initState();

    _future = _getCommits();
    _scrollController.addListener(_loadMoreCommits);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Commits')),
      body: FutureBuilder(
        future: _future,
        builder: (context, state) {
          if (state.connectionState != ConnectionState.done) {
            return const CircularProgressIndicator();
          }
          final data = state.data;
          if (data == null) {
            return const Center(
              child: Text('Nothing to show... repository not initialized.'),
            );
          }
          if (!data) {
            return const Column();
          }

          return Column(
            children: [
              Expanded(
                child: ListView.builder(
                  controller: _scrollController,
                  itemCount:
                      _endOfCommits ? _commits.length : _commits.length + 1,
                  itemBuilder: (context, index) {
                    if (index >= _commits.length) {
                      return const Padding(
                        padding: EdgeInsets.all(16.0),
                        child: Center(
                          child: CircularProgressIndicator(),
                        ),
                      );
                    }

                    final item = _commits[index];

                    final subtitle = Text(
                      '${item.author} ${item.email}',
                      style: const TextStyle(fontSize: kDefaultFontSize / 1.05),
                    );

                    return Card(
                      child: ListTile(
                        leading: CircleAvatar(
                          child: Text('${index + 1}'),
                        ),
                        title: Text(item.message),
                        subtitle: subtitle,
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

  Future<bool> _getCommits() async {
    if (_endOfCommits) {
      return true;
    }

    final data = await widget.repository.log(
      oid: _nextCommit,
      n: widget.chunkSize,
    );

    if (data == null) {
      return false;
    }

    _nextCommit = data.lastOrNull?.parent;

    if (_nextCommit == null) {
      _endOfCommits = true;
    }

    _commits.addAll(data);
    setState(() {});
    return true;
  }

  Future<void> _loadMoreCommits() async {
    if (_scrollController.position.pixels !=
        _scrollController.position.maxScrollExtent) {
      return;
    }

    await _getCommits();
  }
}
