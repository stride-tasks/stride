import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/bridge/api/backend/git.dart';
import 'package:stride/utils/functions.dart';

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

    _reloadCommits();
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

          final listWidget = ListView.builder(
            controller: _scrollController,
            itemCount: _endOfCommits ? _commits.length : _commits.length + 1,
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
                '${oidToString(oid: item.oid)} - ${item.author} ${item.email}',
                style: const TextStyle(fontSize: kDefaultFontSize / 1.05),
              );

              return Card(
                child: ListTile(
                  leading: CircleAvatar(
                    child: Text('${index + 1}'),
                  ),
                  title: Text(item.message),
                  subtitle: subtitle,
                  trailing: index == 0
                      ? null
                      : Tooltip(
                          message: 'Force Reset',
                          child: IconButton(
                            onPressed: () async => _forceHardReset(item.oid),
                            icon: const Icon(
                              Icons.code,
                              color: Colors.red,
                            ),
                          ),
                        ),
                ),
              );
            },
          );

          return Column(children: [Expanded(child: listWidget)]);
        },
      ),
    );
  }

  void _reloadCommits() {
    _commits = [];
    _nextCommit = null;
    _endOfCommits = false;
    _future = _getCommits();
  }

  Future<void> _forceHardReset(Oid commit) async {
    await showAlertDialog(
      context: context,
      content: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(
            'Are you sure you want to force hard reset to ${oidToString(oid: commit)} commit?',
            style: const TextStyle(fontWeight: FontWeight.bold),
            textAlign: TextAlign.center,
          ),
          const SizedBox(height: 5),
          const Text('Action is irreversible!'),
        ],
      ),
      onConfirm: (context) async {
        context.read<LogBloc>().catch_(
              message: 'force hard reset',
              () async => widget.repository.forceHardReset(commit: commit),
            );
        Navigator.pop(context);
        setState(_reloadCommits);
        return true;
      },
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
