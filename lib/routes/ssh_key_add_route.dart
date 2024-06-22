import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/src/rust/api/settings.dart';
import 'package:stride/widgets/settings_widget.dart';
import 'package:uuid/uuid.dart';

class SshKeyAddRoute extends StatefulWidget {
  const SshKeyAddRoute({super.key});

  @override
  State<SshKeyAddRoute> createState() => _SshKeyAddRouteState();
}

class _SshKeyAddRouteState extends State<SshKeyAddRoute> {
  String note = "";
  String publicKey = "";
  String privateKey = "";

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<SettingsBloc, SettingsState>(
      builder: (context, state) {
        final keys = state.settings.keys;
        return Scaffold(
          appBar: AppBar(title: const Text("SSH Key")),
          body: Container(
            padding: const EdgeInsets.symmetric(
              vertical: 10,
              horizontal: 20,
            ),
            child: ListView(
              children: [
                SettingsTileText(
                  leading: const Icon(Icons.public_sharp),
                  title: const Text("Public Key"),
                  text: publicKey,
                  hidden: true,
                  onChanged: (text) {
                    setState(() {
                      publicKey = text;
                    });
                  },
                ),
                const Divider(),
                SettingsTileText(
                  leading: const Icon(Icons.key),
                  title: const Text("Private Key"),
                  text: privateKey,
                  hidden: true,
                  multiline: true,
                  onChanged: (text) {
                    setState(() {
                      privateKey = text;
                    });
                  },
                ),
              ],
            ),
          ),
          floatingActionButton: FloatingActionButton(
            shape: const CircleBorder(),
            onPressed: () {
              if (publicKey.isEmpty) {
                ScaffoldMessenger.of(context).showSnackBar(
                  const SnackBar(
                    content: Text('Cannot add SSH key without public key'),
                    behavior: SnackBarBehavior.floating,
                  ),
                );
                return;
              }

              if (privateKey.isEmpty) {
                ScaffoldMessenger.of(context).showSnackBar(
                  const SnackBar(
                    content: Text('Cannot add SSH key without private key'),
                    behavior: SnackBarBehavior.floating,
                  ),
                );
                return;
              }

              context.read<SettingsBloc>().add(
                    SettingsUpdateEvent(
                      settings: state.settings.copyWith(
                        keys: keys.toList()
                          ..add(
                            SshKey(
                              uuid: UuidValue.fromString(const Uuid().v4()),
                              public: publicKey,
                              private: privateKey,
                            ),
                          ),
                      ),
                    ),
                  );
              Navigator.pop(context);
            },
            child: const Icon(Icons.add_task_sharp, size: 50),
          ),
        );
      },
    );
  }
}
