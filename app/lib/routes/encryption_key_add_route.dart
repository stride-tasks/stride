import 'package:flutter/material.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:stride/bridge/api/error.dart';
import 'package:stride/bridge/api/logging.dart';
import 'package:stride/bridge/api/settings.dart';
import 'package:stride/widgets/settings_widget.dart';

class EncryptionKeyAddRoute extends StatefulWidget {
  const EncryptionKeyAddRoute({super.key});

  @override
  State<EncryptionKeyAddRoute> createState() => EncryptionKeyAddRouteState();
}

class EncryptionKeyAddRouteState extends State<EncryptionKeyAddRoute> {
  String _key = '';

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

          try {
            await EncryptionKey.save(key: _key);
          } on RustError catch (error) {
            Logger.error(
              message: 'encryption key error: ${error.toErrorString()}',
            );
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(
                content: Text(error.toErrorString()),
                behavior: SnackBarBehavior.floating,
              ),
            );
          } on AnyhowException catch (error) {
            Logger.error(message: 'encryption key error: ${error.message}');
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(
                content: Text(error.message),
                behavior: SnackBarBehavior.floating,
              ),
            );
            // ignore: avoid_catches_without_on_clauses
          } catch (error) {
            Logger.error(message: 'encryption key error: ${error}');
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(
                content: Text(error.toString()),
                behavior: SnackBarBehavior.floating,
              ),
            );
          }
          Navigator.pop(context);
        },
        child: const Icon(Icons.add_task_sharp, size: 50),
      ),
    );
  }
}
