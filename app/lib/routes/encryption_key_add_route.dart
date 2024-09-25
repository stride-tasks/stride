import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:qr_flutter/qr_flutter.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/bridge/api/settings.dart';
import 'package:stride/widgets/settings_widget.dart';

class EncryptionKeyAddRoute extends StatefulWidget {
  final EncryptionKey? encryptionKey;
  const EncryptionKeyAddRoute({
    super.key,
    this.encryptionKey,
  });

  @override
  State<EncryptionKeyAddRoute> createState() => EncryptionKeyAddRouteState();
}

class EncryptionKeyAddRouteState extends State<EncryptionKeyAddRoute> {
  String _key = '';

  @override
  void initState() {
    super.initState();

    _key = widget.encryptionKey?.key ?? '';
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Encryption Key')),
      body: Container(
        padding: const EdgeInsets.symmetric(
          vertical: 10,
          horizontal: 20,
        ),
        child: ListView(
          children: [
            SettingsTileText(
              leading: const Icon(Icons.public_sharp),
              title: const Text('Key'),
              text: _key,
              hidden: true,
              onChanged: (text) {
                setState(() {
                  _key = text;
                });
              },
            ),
            if (widget.encryptionKey != null)
              Center(
                child: Padding(
                  padding: const EdgeInsets.all(8.0),
                  child: LimitedBox(
                    maxWidth: 512,
                    maxHeight: 512,
                    child: QrImageView(
                      data: _key,
                      backgroundColor: Colors.white,
                      padding: const EdgeInsets.all(16.0),
                    ),
                  ),
                ),
              ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        shape: const CircleBorder(),
        onPressed: () async {
          if (_key.isEmpty) {
            ScaffoldMessenger.of(context).showSnackBar(
              const SnackBar(
                content: Text('cannot add empty encryption key'),
                behavior: SnackBarBehavior.floating,
              ),
            );
            return;
          }

          await context.read<LogBloc>().catch_(() async {
            if (widget.encryptionKey == null) {
              await EncryptionKey.save(key: _key);
            } else {
              await EncryptionKey.update(
                uuid: widget.encryptionKey!.uuid,
                key: _key,
              );
            }
          });
          Navigator.pop(context);
        },
        child: const Icon(Icons.add_task_sharp, size: 50),
      ),
    );
  }
}
