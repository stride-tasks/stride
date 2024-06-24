import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/routes/routes.dart';
import 'package:stride/src/rust/api/logging.dart';
import 'package:stride/src/rust/api/settings.dart';
import 'package:stride/utils/functions.dart';

class SshKeysRoute extends StatelessWidget {
  final void Function(SshKey key)? onTap;
  final bool hasDelete;

  const SshKeysRoute({
    super.key,
    this.onTap,
    this.hasDelete = true,
  });

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text("SSH Keys")),
      body: BlocBuilder<SettingsBloc, SettingsState>(
        builder: (context, state) {
          final keys = state.settings.keys;
          return SingleChildScrollView(
            child: Column(
              children: [
                ElevatedButton.icon(
                  icon: const Icon(Icons.generating_tokens),
                  label: const Text("Generate Key"),
                  onPressed: () async {
                    final sshKey = await SshKey.generate();
                    if (context.mounted) {
                      context
                          .read<SettingsBloc>()
                          .add(SettingsAddSshKeyEvent(key: sshKey));
                    }

                    Logger.trace(message: "SSH Key generated");
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
            ),
          );
        },
      ),
      floatingActionButton: IconButton(
        icon: const Icon(Icons.add_circle_outline, size: 50),
        onPressed: () {
          Navigator.of(context).pushNamed(Routes.sshKeysAdd);
        },
      ),
    );
  }

  ListTile _listItem(BuildContext context, SshKey key) {
    return ListTile(
      title: Text(key.public),
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
                      const Text(
                        "Are you sure you want to delete the ssh key? (action is irreversible)",
                        style: TextStyle(fontWeight: FontWeight.bold),
                        textAlign: TextAlign.center,
                      ),
                      const SizedBox(height: 5),
                      Text(key.public),
                    ],
                  ),
                  onConfirm: (context) {
                    context
                        .read<SettingsBloc>()
                        .add(SettingsRemoveSshKeyEvent(uuid: key.uuid));
                    Navigator.of(context).pop();

                    Logger.trace(message: "SSH Key deleted");
                    return Future.value(true);
                  },
                );
              },
            ),
      onTap: onTap == null ? null : () => onTap!(key),
    );
  }
}
