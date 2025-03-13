import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/bridge/api/settings.dart';
import 'package:stride/widgets/settings_widget.dart';

class SshKeyAddRoute extends StatefulWidget {
  const SshKeyAddRoute({super.key});

  @override
  State<SshKeyAddRoute> createState() => _SshKeyAddRouteState();
}

class _SshKeyAddRouteState extends State<SshKeyAddRoute> {
  String publicKey = '';
  String privateKey = '';

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('SSH Key')),
      body: Container(
        padding: const EdgeInsets.symmetric(
          vertical: 10,
          horizontal: 20,
        ),
        child: ListView(
          children: [
            SettingsTileText(
              leading: const Icon(Icons.public_sharp),
              title: const Text('Public Key'),
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
              title: const Text('Private Key'),
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
        onPressed: () async {
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

          await context.read<LogBloc>().catch_(message: 'ssh key', () async {
            return SshKey.save(publicKey: publicKey, privateKey: privateKey);
          });
          if (context.mounted) Navigator.pop(context);
        },
        child: const Icon(Icons.add_task_sharp, size: 50),
      ),
    );
  }
}
