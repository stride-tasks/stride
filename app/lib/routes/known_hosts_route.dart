import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/api.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/context.dart';
import 'package:stride/logging.dart';
import 'package:stride/utils/functions.dart';

class KnownHostsRoute extends StatefulWidget {
  final void Function(SshHost key)? onTap;
  final bool hasDelete;

  const KnownHostsRoute({super.key, this.onTap, this.hasDelete = true});

  @override
  State<KnownHostsRoute> createState() => _KnownHostsRouteState();
}

class _KnownHostsRouteState extends State<KnownHostsRoute> {
  Future<SshHostListMethodResult>? _knownHosts;

  @override
  void initState() {
    super.initState();

    _knownHosts = SshHostListMethod().execute();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('SSH Known Hosts')),
      body: FutureBuilder(
        future: _knownHosts,
        builder: (context, state) {
          if (state.connectionState != ConnectionState.done) {
            return const Center(child: CircularProgressIndicator());
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
              return Card(child: _listItem(hosts, index, context));
            },
          );
        },
      ),
    );
  }

  ListTile _listItem(List<SshHost> hosts, int index, BuildContext context) {
    final host = hosts[index];
    return ListTile(
      title: Text('${host.hostname} - ${host.key.type.name}'),
      subtitle: Text(host.key.public),
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
                        '${host.hostname} - ${host.key.type.name} - ${host.key.public}',
                      ),
                    ],
                  ),
                  onConfirm: (context) async {
                    context.read<LogBloc>().catch_(
                      () async => SshHostRemoveMethod(
                        hostname: host.hostname,
                        format: host.key.type,
                      ).execute(),
                    );
                    Log.trace('Known host deleted');
                    return true;
                  },
                );
              },
            ),
      onTap: widget.onTap == null ? null : () => widget.onTap!(host),
    );
  }
}
