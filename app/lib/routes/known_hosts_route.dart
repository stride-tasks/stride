import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/bridge/api/logging.dart';
import 'package:stride/bridge/git/known_hosts.dart';
import 'package:stride/utils/functions.dart';

class KnownHostsRoute extends StatelessWidget {
  final void Function(Host key)? onTap;
  final bool hasDelete;

  const KnownHostsRoute({
    super.key,
    this.onTap,
    this.hasDelete = true,
  });

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('SSH Known Hosts')),
      body: BlocBuilder<SettingsBloc, SettingsState>(
        builder: (context, state) {
          final hosts = state.settings.knownHosts.hosts;
          return ListView.builder(
            itemCount: hosts.length,
            itemBuilder: (context, index) {
              final host = hosts[index];
              return Card(child: _listItem(host, context));
            },
          );
        },
      ),
    );
  }

  ListTile _listItem(Host host, BuildContext context) {
    return ListTile(
      title: Text('${host.hostname} - ${host.keyType.name}'),
      subtitle: Text(host.key),
      trailing: !hasDelete
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
                  onConfirm: (context) {
                    context
                        .read<SettingsBloc>()
                        .add(SettingsRemoveKnownHostEvent(host: host));
                    Navigator.of(context).pop();

                    Logger.trace(message: 'SSH Key deleted');
                    return Future.value(true);
                  },
                );
              },
            ),
      onTap: onTap == null ? null : () => onTap!(host),
    );
  }
}
