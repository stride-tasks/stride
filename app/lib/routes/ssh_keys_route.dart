import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/bridge/api/error.dart';
import 'package:stride/bridge/api/logging.dart' as logging;
import 'package:stride/bridge/api/settings.dart';
import 'package:stride/routes/ssh_key_add_route.dart';
import 'package:stride/utils/functions.dart';
import 'package:uuid/uuid.dart';

class SshKeysRoute extends StatefulWidget {
  final void Function(SshKey key)? onTap;
  final bool hasDelete;
  final UuidValue? selected;

  const SshKeysRoute({
    super.key,
    this.onTap,
    this.hasDelete = true,
    this.selected,
  });

  @override
  State<SshKeysRoute> createState() => _SshKeysRouteState();
}

class _SshKeysRouteState extends State<SshKeysRoute> {
  Future<List<SshKey>>? _keys;

  @override
  void initState() {
    super.initState();

    _reset();
  }

  void _reset() {
    _keys = sshKeys();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('SSH Keys')),
      body: SingleChildScrollView(
        child: FutureBuilder(
          future: _keys,
          builder: (context, snapshot) {
            if (snapshot.error is RustError) {
              final error = snapshot.error! as RustError;
              logging.error(
                message:
                    'could not load the SSH keys: ${error.toErrorString()}',
              );
              Navigator.of(context).pop();
            } else if (snapshot.hasError) {
              logging.error(
                message: 'could not load the SSH keys: ${snapshot.error!}',
              );
              Navigator.of(context).pop();
            }

            if (snapshot.connectionState != ConnectionState.done) {
              return const Center(child: CircularProgressIndicator.adaptive());
            }

            final keys = snapshot.data!;
            return Column(
              children: [
                ElevatedButton.icon(
                  icon: const Icon(Icons.generating_tokens),
                  label: const Text('Generate Key'),
                  onPressed: () async {
                    final sshKey = await SshKey.generate();
                    logging.trace(
                      message: 'SSH Key generated with UUID: ${sshKey.uuid}',
                    );
                    setState(_reset);
                  },
                ),
                const SizedBox(height: 5),
                BlocBuilder<SettingsBloc, SettingsState>(
                  builder: (context, state) {
                    return ListView.builder(
                      shrinkWrap: true,
                      itemCount: keys.length,
                      itemBuilder: (context, index) {
                        final key = keys[index];
                        return Card(
                          child: _listItem(context, key, state.settings),
                        );
                      },
                    );
                  },
                ),
              ],
            );
          },
        ),
      ),
      floatingActionButton: IconButton(
        icon: const Icon(Icons.add_circle_outline, size: 50),
        onPressed: () {
          Navigator.of(context).push<void>(
            MaterialPageRoute(builder: (context) => const SshKeyAddRoute()),
          );
        },
      ),
    );
  }

  ListTile _listItem(BuildContext context, SshKey key, Settings settings) {
    final publicKey = key.publicKey;
    final uuid = key.uuid;

    final subtitle = Text(uuid.toString());
    // final subtitle = switch (settings.repositories.any(
    //   (element) => element.sshKeyUuid == uuid,
    // )) {
    //   false => Text(uuid.toString()),
    //   true => RichText(
    //     text: TextSpan(
    //       children: [
    //         TextSpan(text: '$uuid '),
    //         WidgetSpan(
    //           child: Icon(Icons.info, size: 16, color: Colors.blue[200]),
    //         ),
    //         const TextSpan(text: ' Key In Use'),
    //       ],
    //     ),
    //   ),
    // };

    return ListTile(
      title: Text(publicKey),
      subtitle: subtitle,
      selected: key.uuid == widget.selected,
      leading: widget.selected == null
          ? null
          : key.uuid != widget.selected
          ? const Icon(Icons.circle_outlined)
          : const Icon(Icons.check_circle_outline_rounded),
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
                      const Text(
                        'Are you sure you want to delete the ssh key? (action is irreversible)',
                        style: TextStyle(fontWeight: FontWeight.bold),
                        textAlign: TextAlign.center,
                      ),
                      const SizedBox(height: 5),
                      Text(publicKey),
                    ],
                  ),
                  onConfirm: (context) async {
                    await context.read<LogBloc>().catch_(
                      message: 'ssh key',
                      () async => SshKey.removeKey(uuid: key.uuid),
                    );
                    logging.trace(
                      message: 'SSH Key deleted with UUID: ${key.uuid}',
                    );
                    setState(_reset);
                    return true;
                  },
                );
              },
            ),
      onTap: widget.onTap == null ? null : () => widget.onTap!(key),
    );
  }
}
