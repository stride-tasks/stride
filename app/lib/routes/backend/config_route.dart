import 'dart:convert';
import 'dart:math';
import 'dart:typed_data';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:qr_flutter/qr_flutter.dart';
import 'package:stride/api.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/bridge/api/repository.dart';
import 'package:stride/bridge/api/settings.dart';
import 'package:stride/context.dart';
import 'package:stride/utils/functions.dart';
import 'package:stride/widgets/settings_widget.dart';
import 'package:uuid/uuid.dart';

Uint8List generateCryptoRandomBytes(int length) {
  final secureRandom = Random.secure();
  final bytes = Uint8List(length);
  for (var i = 0; i < length; i++) {
    bytes[i] = secureRandom.nextInt(256);
  }
  return bytes;
}

class BackendConfigRoute extends StatefulWidget {
  final Repository repository;
  final UuidValue repositoryId;
  final UuidValue backendId;
  const BackendConfigRoute({
    super.key,
    required this.repository,
    required this.backendId,
    required this.repositoryId,
  });

  @override
  State<BackendConfigRoute> createState() => _BackendConfigRouteState();
}

class _BackendConfigRouteState extends State<BackendConfigRoute> {
  Future<BackendRecord?>? _backendRecord;

  @override
  void initState() {
    super.initState();
    _backendRecord = RepositoryBackendGetMethod(
      repositoryId: widget.repositoryId,
      backendId: widget.backendId,
    ).execute().then((value) => value.backendRecord);
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder(
      future: _backendRecord,
      builder: (context, snapshot) {
        if (snapshot.hasError) {
          context.read<LogBloc>().add(LogErrorEvent(error: snapshot.error!));
        }
        if (snapshot.connectionState != ConnectionState.done) {
          return Center(child: CircularProgressIndicator.adaptive());
        }

        final record = snapshot.data!;
        return Scaffold(
          appBar: AppBar(title: Text(record.instance.name)),
          body: _ConfigSection(
            repository: widget.repository,
            repositoryId: widget.repositoryId,
            record: record,
          ),
        );
      },
    );
  }
}

class _ConfigSection extends StatefulWidget {
  final Repository repository;
  final UuidValue repositoryId;
  final BackendRecord record;
  const _ConfigSection({
    required this.record,
    required this.repository,
    required this.repositoryId,
  });

  @override
  State<_ConfigSection> createState() => _ConfigSectionState();
}

class _ConfigSectionState extends State<_ConfigSection> {
  Future<List<SshKey>>? _sshKeys;

  late ConfigSchema _schema;
  late Config _config;

  @override
  void initState() {
    super.initState();

    _sshKeys = sshKeys();

    // TODO: Verify that descriptor is not null.
    _schema = widget.record.descriptor!.schema;
    _config = widget.record.instance.config;
  }

  @override
  Widget build(BuildContext context) {
    final tiles = <Widget>[];
    for (final field in _schema.fields) {
      final value = _valueForField(field.id);

      switch (field.value) {
        case ConfigSchemaValue_Url(default_: final defaultValue):
          tiles.add(
            SettingsTileText(
              title: Text(field.name),
              text: switch (value) {
                ConfigValue_Url(value: final uri) => uri.toString(),
                _ => defaultValue?.toString() ?? '',
              },
              onChanged: (text) async {
                setState(() {
                  _setValue(field.id, ConfigValue.url(value: Uri.parse(text)));
                });
                await _save();
              },
              validator: (text) {
                if (text == null || Uri.tryParse(text) != null) {
                  return null;
                }
                return 'Enter a valid URL';
              },
            ),
          );
        case ConfigSchemaValue_String(default_: final defaultValue):
          tiles.add(
            SettingsTileText(
              title: Text(field.name),
              text: switch (value) {
                ConfigValue_String(value: final content) => content,
                _ => defaultValue ?? '',
              },
              onChanged: (text) async {
                setState(() {
                  _setValue(field.id, ConfigValue.string(value: text));
                });
                await _save();
              },
            ),
          );

        case final ConfigSchemaValue_Bytes bytesSchema:
          tiles.add(_bytesProperty(field, bytesSchema, value, context));

        case ConfigSchemaValue_SshKey(default_: final defaultValue):
          final uuid = switch (value) {
            ConfigValue_Uuid(value: final uuid) => uuid,
            _ => defaultValue,
          };
          tiles.add(_sshKeyProperty(field.id, field.name, uuid));

        case ConfigSchemaValue_Uuid(default_: final defaultValue):
          tiles.add(_uuidProperty(field.id, field.name, value, defaultValue));
      }
    }
    return SettingsList(sections: [SettingsSection(tiles: tiles)]);
  }

  ConfigValue? _valueForField(String id) {
    for (final field in _config.fields) {
      if (field.id == id) {
        return field.value;
      }
    }
    return null;
  }

  void _setValue(String id, ConfigValue value) {
    final fields = [..._config.fields];
    final index = fields.indexWhere((field) => field.id == id);
    final field = ConfigField(id: id, value: value);
    if (index == -1) {
      fields.add(field);
    } else {
      fields[index] = field;
    }
    _config = _config.copyWith(fields: fields);
  }

  SettingsTileUuid _uuidProperty(
    String id,
    String name,
    ConfigValue? value,
    UuidValue? defaultValue,
  ) {
    return SettingsTileUuid(
      title: Text(name),
      value: switch (value) {
        ConfigValue_Uuid(value: final uuid) => uuid,
        _ => defaultValue,
      },
      onChanged: (uuid) async {
        setState(() {
          _setValue(id, ConfigValue.uuid(value: uuid));
        });
        await _save();
      },
    );
  }

  FutureBuilder<List<SshKey>> _sshKeyProperty(
    String id,
    String name,
    UuidValue? uuid,
  ) {
    return FutureBuilder(
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
              _setValue(id, ConfigValue.uuid(value: uuid));
            });
            await _save();
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
                    _setValue(id, ConfigValue.uuid(value: key));
                    _sshKeys = sshKeys();
                  });

                  await _save();
                }
              },
              icon: Icon(Icons.generating_tokens),
            ),
          ),
        );
      },
    );
  }

  SettingsTileText _bytesProperty(
    ConfigSchemaField field,
    ConfigSchemaValue_Bytes schema,
    ConfigValue? value,
    BuildContext context,
  ) {
    final bytes = switch (value) {
      ConfigValue_Bytes(value: final bytes) => bytes,
      _ => schema.default_,
    };
    final ConfigSchemaValue_Bytes(:min, :max, :category, :generator) = schema;
    return SettingsTileText(
      obscureText: category == ConfigSchemaByteCategory.password,
      title: Text(field.name),
      text: bytes == null ? '' : base64Url.encode(bytes),
      onChanged: (text) async {
        final bytes = base64Url.decode(text);
        setState(() {
          _setValue(field.id, ConfigValue.bytes(value: bytes));
        });
        await _save();
      },
      validator: (text) {
        if (text == null) {
          return null;
        }

        try {
          final bytes = base64Url.decode(text);
          if (min != null && bytes.length < min) {
            return 'input too small must be $min length, got: ${bytes.length}';
          }
          if (max != null && bytes.length > max) {
            return 'input too large must be $max length, got: ${bytes.length}';
          }
        } on FormatException catch (error) {
          return error.message;
        }
        return null;
      },
      trailing: Wrap(
        children: [
          if (field.showQrCode && bytes != null)
            Tooltip(
              message: 'Show QR Code',
              child: IconButton(
                onPressed: () async {
                  await showGeneralDialog(
                    context: context,
                    pageBuilder: (context, animation, secondaryAnimation) {
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
                                data: base64Url.encode(bytes),
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
            ),
          if (generator == ConfigSchemaByteGenerator.cryptoRandom)
            Tooltip(
              message: 'Generate',
              child: IconButton(
                onPressed: () async {
                  final key = await showAlertDialog<Uint8List>(
                    context: context,
                    content: Text(
                      'Are you sure you want to generate a new encryption key?',
                    ),
                    onConfirm: (context) async {
                      return generateCryptoRandomBytes(max ?? min ?? 0);
                    },
                  );

                  if (key != null) {
                    setState(() {
                      _setValue(field.id, ConfigValue.bytes(value: key));
                    });
                    await _save();
                  }
                },
                icon: Icon(Icons.generating_tokens),
              ),
            ),
        ],
      ),
    );
  }

  Future<void> _save() async {
    final instance = widget.record.instance.copyWith(config: _config);

    await RepositoryBackendSetMethod(
      repositoryId: widget.repositoryId,
      backendInstance: instance,
    ).execute();
  }
}
