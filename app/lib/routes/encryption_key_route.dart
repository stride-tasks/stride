import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:qr_flutter/qr_flutter.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/bridge/api/settings.dart';
import 'package:stride/widgets/settings_widget.dart';

class EncryptionKeyRoute extends StatefulWidget {
  final RepositorySpecification repository;
  const EncryptionKeyRoute({super.key, required this.repository});

  @override
  State<EncryptionKeyRoute> createState() => EncryptionKeyRouteState();
}

class EncryptionKeyRouteState extends State<EncryptionKeyRoute> {
  String _key = '';
  bool showQrCode = false;

  @override
  void initState() {
    super.initState();

    _key = widget.repository.encryption?.key ?? '';
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Encryption')),
      body: Container(
        padding: const EdgeInsets.symmetric(
          vertical: 10,
          horizontal: 20,
        ),
        child: ListView(
          children: [
            SettingsTileText(
              leading: const Icon(Icons.key),
              title: const Text('Key'),
              text: _key,
              hidden: true,
              onChanged: (text) async {
                _key = text;
                showQrCode = false;

                if (_key.isEmpty) {
                  ScaffoldMessenger.of(context).showSnackBar(
                    const SnackBar(
                      content: Text('cannot add empty encryption key'),
                      behavior: SnackBarBehavior.floating,
                    ),
                  );
                  return;
                }

                await context.read<LogBloc>().catch_(
                      message: 'encrypiton key',
                      () async => EncryptionKey.save(
                        repositoryUuid: widget.repository.uuid,
                        key: _key,
                      ),
                    );
                setState(() {});
              },
            ),
            const Divider(),
            ElevatedButton.icon(
              onPressed: widget.repository.encryption == null
                  ? null
                  : () => setState(() => showQrCode = !showQrCode),
              label: showQrCode
                  ? const Text('Hide QR Code')
                  : const Text('Show QR Code'),
              icon: const Icon(Icons.qr_code),
            ),
            if (showQrCode)
              Column(
                children: [
                  const Divider(),
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
          ],
        ),
      ),
    );
  }
}
