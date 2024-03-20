import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/src/rust/git/known_hosts.dart';
import 'package:stride/widgets/custom_app_bar.dart';

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
      appBar: const CustomAppBar(title: "SSH Known Hosts"),
      body: BlocBuilder<SettingsBloc, SettingsState>(
        builder: (context, state) {
          final hosts = state.settings.knownHosts.hosts;
          return ListView.builder(
            itemCount: hosts.length,
            itemBuilder: (context, index) {
              final host = hosts[index];
              return Card(
                child: ListTile(
                  title: Text("${host.hostname} - ${host.remoteKeyType.name}"),
                  subtitle: Text(host.remoteHostKey),
                  trailing: !hasDelete
                      ? null
                      : IconButton(
                          icon: const Icon(Icons.delete),
                          onPressed: () {
                            context
                                .read<SettingsBloc>()
                                .add(SettingsRemoveKnownHostEvent(host: host));
                          },
                        ),
                  onTap: onTap == null
                      ? null
                      : () {
                          onTap!(host);
                        },
                ),
              );
            },
          );
        },
      ),
    );
  }
}
