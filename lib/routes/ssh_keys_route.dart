import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/routes/routes.dart';
import 'package:stride/src/rust/api/settings.dart';
import 'package:stride/widgets/custom_app_bar.dart';

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
      appBar: const CustomAppBar(title: "SSH Keys"),
      body: BlocBuilder<SettingsBloc, SettingsState>(
        builder: (context, state) {
          final keys = state.settings.keys;
          return SingleChildScrollView(
            child: Column(
              children: [
                ElevatedButton(
                  child: const Icon(Icons.generating_tokens),
                  onPressed: () async {
                    final sshKey = await SshKey.generate();
                    if (context.mounted) {
                      context
                          .read<SettingsBloc>()
                          .add(SettingsAddSshKeyEvent(key: sshKey));
                    }
                  },
                ),
                ListView.builder(
                  shrinkWrap: true,
                  itemCount: keys.length,
                  itemBuilder: (context, index) {
                    final key = keys[index];
                    return Card(
                      child: ListTile(
                        title: Text(key.public),
                        trailing: !hasDelete
                            ? null
                            : IconButton(
                                icon: const Icon(Icons.delete),
                                onPressed: () {
                                  context.read<SettingsBloc>().add(
                                        SettingsRemoveSshKeyEvent(
                                          uuid: key.uuid,
                                        ),
                                      );
                                },
                              ),
                        onTap: onTap == null
                            ? null
                            : () {
                                onTap!(key);
                              },
                      ),
                    );
                  },
                ),
              ],
            ),
          );
        },
      ),
      floatingActionButton: IconButton(
        icon: const Icon(Icons.add_circle_outline),
        onPressed: () {
          Navigator.of(context).pushNamed(Routes.sshKeysAdd);
        },
      ),
    );
  }
}
