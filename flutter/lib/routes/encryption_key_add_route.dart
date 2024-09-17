import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:qr_flutter/qr_flutter.dart';
import 'package:stride/blocs/settings_bloc.dart';
import 'package:stride/bridge/api/error.dart';
import 'package:stride/bridge/api/logging.dart';
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
                  padding: EdgeInsets.all(8.0),
                  child: LimitedBox(
                    maxWidth: 512,
                    maxHeight: 512,
                    child: QrImageView(
                      data: _key,
                      backgroundColor: Colors.white,
                      padding: EdgeInsets.all(16.0),
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

          try {
            if (widget.encryptionKey == null) {
              await EncryptionKey.save(key: _key);
            } else {
              await EncryptionKey.update(
                uuid: widget.encryptionKey!.uuid,
                key: _key,
              );
            }
            context.read<SettingsBloc>().add(
                  SettingsUpdateEvent(
                    settings: context.read<SettingsBloc>().settings,
                  ),
                );
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
