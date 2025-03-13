import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/bridge/api/logging.dart';
import 'package:stride/bridge/git/known_hosts.dart';
import 'package:stride/utils/functions.dart';

class KnownHostsRoute extends StatefulWidget {
  final void Function(Host key)? onTap;
  final bool hasDelete;

  const KnownHostsRoute({
    super.key,
    this.onTap,
    this.hasDelete = true,
  });

  @override
  State<KnownHostsRoute> createState() => _KnownHostsRouteState();
}

class _KnownHostsRouteState extends State<KnownHostsRoute> {
  Future<KnownHosts>? _knownHosts;

  @override
  void initState() {
    super.initState();

    _knownHosts = KnownHosts.load();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('SSH Known Hosts')),
      body: FutureBuilder<KnownHosts>(
        future: _knownHosts,
        builder: (context, state) {
          if (state.connectionState != ConnectionState.done) {
            return const Center(
              child: CircularProgressIndicator(),
            );
          }

          if (state.hasError) {
            context.read<LogBloc>().add(LogErrorEvent(error: state.error!));
            Navigator.of(context).pop();
            return const Placeholder();
          }

          final knownHosts = state.data!;
          final hosts = knownHosts.hosts;
          return ListView.builder(
            itemCount: hosts.length,
            itemBuilder: (context, index) {
              return Card(child: _listItem(knownHosts, index, context));
            },
          );
        },
      ),
    );
  }

  ListTile _listItem(KnownHosts knownHosts, int index, BuildContext context) {
    final host = knownHosts.hosts[index];
    return ListTile(
      title: Text('${host.hostname} - ${host.keyType.name}'),
      subtitle: Text(host.key),
      trailing: !widget.hasDelete
          ? null
          : IconButton(
              icon: const Icon(Icons.delete),
              onPressed: () async {
                await showAlertDialog(
                  context: context,
                  content: Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Text(
                        'Are you sure you want to delete the known host ${host.hostname}? (action is irreversible)',
                        style: const TextStyle(fontWeight: FontWeight.bold),
                        textAlign: TextAlign.center,
                      ),
                      const SizedBox(height: 5),
                      Text(
                        '${host.hostname} - ${host.keyType.name} - ${host.key}',
                      ),
                    ],
                  ),
                  onConfirm: (context) async {
                    context.read<LogBloc>().catch_(
                          () async => KnownHosts.save(
                            this_: knownHosts.copyWith(
                              hosts: knownHosts.hosts.toList()
                                ..removeWhere((element) => element == host),
                            ),
                          ),
                        );
                    Navigator.of(context).pop();
                    Logger.trace(message: 'Known host deleted');
                    return true;
                  },
                );
              },
            ),
      onTap: widget.onTap == null ? null : () => widget.onTap!(host),
    );
  }
}
