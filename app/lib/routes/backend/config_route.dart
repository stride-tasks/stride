import 'dart:convert';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:qr_flutter/qr_flutter.dart';
import 'package:stride/bridge/api/repository.dart';
import 'package:stride/bridge/api/settings.dart';
import 'package:stride/bridge/third_party/stride_backend_git/encryption_key.dart';
import 'package:stride/utils/functions.dart';
import 'package:stride/widgets/settings_widget.dart';
import 'package:uuid/uuid.dart';

enum ValueType {
  url,
  string,
  bytes,
  sshkey,
  uuid,
  encryption;

  static ValueType fromString(String value) {
    switch (value) {
      case 'url':
        return ValueType.url;
      case 'string':
        return ValueType.string;
      case 'bytes':
        return ValueType.bytes;
      case 'uuid':
        return ValueType.uuid;
      case 'ssh-key':
        return ValueType.sshkey;
      case 'encryption':
        return ValueType.encryption;
      default:
        throw ArgumentError('Unknown ValueType: $value');
    }
  }

  String toJsonString() {
    switch (this) {
      case ValueType.url:
        return 'url';
      case ValueType.string:
        return 'string';
      case ValueType.bytes:
        return 'bytes';
      case ValueType.sshkey:
        return 'ssh-key';
      case ValueType.uuid:
        return 'uuid';
      case ValueType.encryption:
        return 'encryption';
    }
  }
}

enum EncryptionMode {
  aesOcb256;

  static EncryptionMode fromString(String value) {
    switch (value) {
      case 'AesOcb256':
        return EncryptionMode.aesOcb256;
      default:
        throw ArgumentError('Unknown ValueType: $value');
    }
  }

  String toJsonString() {
    switch (this) {
      case EncryptionMode.aesOcb256:
        return 'AesOcb256';
    }
  }
}

class EncryptionValue {
  EncryptionMode mode;
  Uint8List? bytes;

  EncryptionValue({required this.mode, required this.bytes});

  factory EncryptionValue.fromJson(Map<String, dynamic> json) {
    final mode = EncryptionMode.fromString(json['mode'] as String);

    final bytesJson = json['bytes'];

    Uint8List? bytes;
    switch (mode) {
      case EncryptionMode.aesOcb256:
        bytes = bytesJson == null
            ? null
            : Uint8List.fromList(List<int>.from(bytesJson as List));
    }

    return EncryptionValue(mode: mode, bytes: bytes);
  }

  Map<String, dynamic> toJson() {
    return {'mode': mode.toJsonString(), 'bytes': bytes?.toList()};
  }
}

class Value {
  ValueType type;
  dynamic content;

  Value({required this.type, required this.content});

  factory Value.fromJson(Map<String, dynamic> json) {
    final type = ValueType.fromString(json['type'] as String);

    dynamic content;
    switch (type) {
      case ValueType.url:
      case ValueType.string:
        content = json['content'] as String?;
      case ValueType.uuid:
      case ValueType.sshkey:
        final value = json['content'] as String?;
        content = value == null ? null : UuidValue.withValidation(value);
      case ValueType.bytes:
        final value = json['content'] as List?;
        content = value == null ? null : List<int>.from(value);
      case ValueType.encryption:
        content = EncryptionValue.fromJson(
          json['content'] as Map<String, dynamic>,
        );
    }

    return Value(type: type, content: content);
  }

  Map<String, dynamic> toJson() {
    dynamic result;
    switch (type) {
      case ValueType.url:
      case ValueType.string:
      case ValueType.bytes:
        result = content;
      case ValueType.uuid:
      case ValueType.sshkey:
        result = (content as UuidValue?)?.toString();
      case ValueType.encryption:
        result = (content as EncryptionValue).toJson();
    }
    return {'type': type.toJsonString(), 'content': result};
  }
}

class Config {
  Map<String, Value> fields;

  Config({required this.fields});

  factory Config.fromJson(Map<String, dynamic> json) {
    final fieldsJson = json['fields'] as Map<String, dynamic>;

    final fields = fieldsJson.map(
      (key, value) =>
          MapEntry(key, Value.fromJson(value as Map<String, dynamic>)),
    );

    return Config(fields: fields);
  }

  Map<String, dynamic> toJson() {
    return {
      'fields': fields.map((key, value) => MapEntry(key, value.toJson())),
    };
  }
}

class BackendConfigRoute extends StatefulWidget {
  final Repository repository;
  final BackendRecord backend;
  const BackendConfigRoute({
    super.key,
    required this.repository,
    required this.backend,
  });

  @override
  State<BackendConfigRoute> createState() => _BackendConfigRouteState();
}

class _BackendConfigRouteState extends State<BackendConfigRoute> {
  late Map<String, dynamic> _json;
  Future<List<SshKey>>? _sshKeys;

  @override
  void initState() {
    super.initState();
    _json = jsonDecode(widget.backend.config) as Map<String, dynamic>;
    _sshKeys = sshKeys();
  }

  @override
  Widget build(BuildContext context) {
    final config = Config.fromJson(_json);

    final tiles = <Widget>[];
    for (final entry in config.fields.entries) {
      final name = entry.key;
      final value = entry.value;

      switch (value.type) {
        case ValueType.url:
        case ValueType.string:
          tiles.add(
            SettingsTileText(
              title: Text(name),
              text: value.content as String? ?? '',
              onChanged: (text) async {
                setState(() {
                  value.content = text;
                });
                await _save(config);
              },
            ),
          );

        case ValueType.bytes:
          final bytes = value.content as List<int>?;
          tiles.add(
            SettingsTileText(
              title: Text(name),
              text: bytes == null ? '' : base64Url.encode(bytes),
              onChanged: (text) async {
                setState(() {
                  value.content = base64Url.decode(text);
                });
                await _save(config);
              },
            ),
          );

        case ValueType.sshkey:
          final uuid = value.content as UuidValue?;

          tiles.add(
            FutureBuilder(
              future: _sshKeys,
              builder: (context, snapshot) {
                if (snapshot.connectionState != ConnectionState.done) {
                  return CircularProgressIndicator.adaptive();
                }
                return SettingsTileSsh(
                  title: Text(name),
                  uuid: uuid,
                  keys: snapshot.data!,
                  onChanged: (uuid) async {
                    setState(() {
                      value.content = uuid;
                    });
                    await _save(config);
                  },
                  trailing: Tooltip(
                    message: 'Generate new SSH key',
                    child: IconButton(
                      onPressed: () async {
                        final key = await showAlertDialog<UuidValue>(
                          context: context,
                          content: Text(
                            'Are you sure you want to generate a new SSH Key?',
                          ),
                          onConfirm: (context) async {
                            final key = await SshKey.generate();
                            return key.uuid;
                          },
                        );

                        if (key != null) {
                          setState(() {
                            value.content = key;
                          });

                          await _save(config);
                        }
                      },
                      icon: Icon(Icons.generating_tokens),
                    ),
                  ),
                );
              },
            ),
          );
        case ValueType.uuid:
          tiles.add(
            SettingsTileUuid(
              title: Text(name),
              value: value.content as UuidValue?,
              onChanged: (uuid) async {
                setState(() {
                  value.content = uuid;
                });
                await _save(config);
              },
            ),
          );
        case ValueType.encryption:
          final encryption = value.content as EncryptionValue;
          tiles.add(
            SettingsTileText(
              obscureText: true,
              title: Text('$name ${encryption.mode.toJsonString()}'),
              text: encryption.bytes == null
                  ? ''
                  : base64Url.encode(encryption.bytes!),
              onChanged: (text) async {
                setState(() {
                  encryption.bytes = base64Url.decode(text);
                });
                await _save(config);
              },
              validator: (text) {
                if (text == null) {
                  return 'empty encryption key';
                }

                Uint8List bytes;
                try {
                  bytes = base64Url.decode(text);
                } on FormatException catch (e) {
                  return e.message;
                }

                if (bytes.length != 32) {
                  return 'Must be 32 bytes (actual ${bytes.length} bytes)';
                }
                return null;
              },
              trailing: Tooltip(
                message: 'Show QR Code',
                child: Wrap(
                  children: [
                    if (encryption.bytes != null)
                      IconButton(
                        onPressed: () async {
                          await showGeneralDialog(
                            context: context,
                            pageBuilder:
                                (context, animation, secondaryAnimation) {
                                  return GestureDetector(
                                    onTap: () {
                                      Navigator.of(context).pop();
                                    },
                                    child: Center(
                                      child: Padding(
                                        padding: const EdgeInsets.all(26.0),
                                        child: LimitedBox(
                                          maxWidth: 128,
                                          maxHeight: 128,
                                          child: QrImageView(
                                            data: base64Url.encode(
                                              encryption.bytes!,
                                            ),
                                            backgroundColor: Colors.white,
                                            padding: const EdgeInsets.all(16.0),
                                          ),
                                        ),
                                      ),
                                    ),
                                  );
                                },
                          );
                        },
                        icon: Icon(Icons.qr_code),
                      ),
                    IconButton(
                      onPressed: () async {
                        final key = await showAlertDialog<Uint8List>(
                          context: context,
                          content: Text(
                            'Are you sure you want to generate a new encryption key?',
                          ),
                          onConfirm: (context) async {
                            return (await EncryptionKey.generate()).key;
                          },
                        );

                        if (key != null) {
                          setState(() {
                            encryption.bytes = key;
                          });

                          await _save(config);
                        }
                      },
                      icon: Icon(Icons.generating_tokens),
                    ),
                  ],
                ),
              ),
            ),
          );
      }
    }

    return Scaffold(
      appBar: AppBar(title: Text(widget.backend.name)),
      body: SettingsList(sections: [SettingsSection(tiles: tiles)]),
    );
  }

  Future<void> _save(Config config) async {
    _json = config.toJson();

    final string = jsonEncode(_json);
    final backend = BackendRecord(
      id: widget.backend.id,
      name: widget.backend.name,
      enabled: widget.backend.enabled,
      config: string,
    );
    return widget.repository.updateBackend(backend: backend);
  }
}
