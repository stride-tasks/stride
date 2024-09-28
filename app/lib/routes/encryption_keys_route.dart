import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/bridge/api/logging.dart';
import 'package:stride/bridge/api/settings.dart';
import 'package:stride/routes/encryption_key_add_route.dart';
import 'package:stride/utils/functions.dart';
import 'package:uuid/uuid.dart';

class EncryptionKeysRoute extends StatelessWidget {
  final void Function(EncryptionKey key)? onTap;
  final bool hasDelete;
  final UuidValue? selected;

  const EncryptionKeysRoute({
    super.key,
    this.onTap,
    this.hasDelete = true,
    this.selected,
  });

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Encryption Keys')),
      body: BlocBuilder<SettingsBloc, SettingsState>(
        builder: (context, state) {
          final keys = state.settings.encryptionKeys;
          return SingleChildScrollView(
            child: Column(
              children: [
                ElevatedButton.icon(
                  icon: const Icon(Icons.generating_tokens),
                  label: const Text('Generate Key'),
                  onPressed: () async {
                    final sshKey = await EncryptionKey.generate();
                    if (context.mounted) {
                      context
                          .read<SettingsBloc>()
                          .add(SettingsAddEncryptionKeyEvent(key: sshKey));
                    }

                    Logger.trace(message: 'Encryption Key generated');
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
          Navigator.of(context).push<void>(
            MaterialPageRoute(
              builder: (context) => const EncryptionKeyAddRoute(),
            ),
          );
        },
      ),
    );
  }

  ListTile _listItem(BuildContext context, EncryptionKey key) {
    return ListTile(
      title: Text(key.key),
      selected: key.uuid == selected,
      leading: selected == null
          ? null
          : key.uuid != selected
              ? const Icon(Icons.circle_outlined)
              : const Icon(Icons.check_circle_outline_rounded),
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
                        'Are you sure you want to delete the ssh key? (action is irreversible)',
                        style: TextStyle(fontWeight: FontWeight.bold),
                        textAlign: TextAlign.center,
                      ),
                      const SizedBox(height: 5),
                      Text(key.key),
                    ],
                  ),
                  onConfirm: (context) async {
                    final result = await context.read<LogBloc>().catch_(
                          message: 'encryption key removal',
                          () async => EncryptionKey.removeKey(uuid: key.uuid),
                        );
                    if (result.isOk) {
                      if (context.mounted) Navigator.of(context).pop();
                      return true;
                    }

                    if (context.mounted) Navigator.of(context).pop();

                    await Logger.trace(
                      message: 'Encryption Key deleted with UUID: ${key.uuid}',
                    );
                    return true;
                  },
                );
              },
            ),
      onTap: onTap == null ? null : () => onTap!(key),
    );
  }
}
