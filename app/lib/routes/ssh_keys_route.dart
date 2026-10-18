import 'package:flutter/material.dart';
import 'package:stride/api.dart';
import 'package:stride/context.dart';
import 'package:stride/logging.dart';
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
    _keys = SshKeyListMethod().execute().then((result) => result.keys);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('SSH Keys')),
      body: SingleChildScrollView(
        child: FutureBuilder(
          future: _keys,
          builder: (context, snapshot) {
            if (snapshot.hasError) {
              Log.error('could not load the SSH keys: ${snapshot.error!}');
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
                    final result = await SshKeyGenerateMethod(
                      format: SshKeyFormat.sshEd25519,
                    ).execute();
                    Log.trace('SSH Key generated with UUID: ${result.key.id}');
                    setState(_reset);
                  },
                ),
                const SizedBox(height: 5),
                ListView.builder(
                  shrinkWrap: true,
                  itemCount: keys.length,
                  itemBuilder: (context, index) {
                    final key = keys[index];
                    return Card(child: _listItem(context, key));
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

  ListTile _listItem(BuildContext context, SshKey key) {
    final publicKey = key.publicKey;
    final uuid = key.id;

    final subtitle = Text(uuid.toString());
    return ListTile(
      title: Text(publicKey),
      subtitle: subtitle,
      selected: key.id == widget.selected,
      leading: widget.selected == null
          ? null
          : key.id != widget.selected
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
                    await Log.catch_(
                      message: 'ssh key',
                      () async => SshKeyRemoveMethod(id: key.id).execute(),
                    );
                    Log.trace('SSH Key deleted with UUID: ${key.id}');
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
