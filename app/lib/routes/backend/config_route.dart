import 'dart:convert';
import 'dart:math';
import 'dart:typed_data';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:qr_flutter/qr_flutter.dart';
import 'package:stride/blocs/log_bloc.dart';
import 'package:stride/bridge/api/repository.dart';
import 'package:stride/bridge/api/settings.dart';
import 'package:stride/utils/functions.dart';
import 'package:stride/widgets/settings_widget.dart';
import 'package:uuid/uuid.dart';

class Schema {
  final String name;
  final Map<String, SchemaValue> fields;

  Schema({required this.name, required this.fields});

  factory Schema.fromJson(Map<String, dynamic> json) {
    return Schema(
      name: json['name'] as String,
      fields: (json['fields'] as Map<String, dynamic>).map(
        (key, value) =>
            MapEntry(key, SchemaValue.fromJson(value as Map<String, dynamic>)),
      ),
    );
  }
}

sealed class SchemaValue {
  final bool showQrCode;
  const SchemaValue({required this.showQrCode});

  factory SchemaValue.fromJson(Map<String, dynamic> map) {
    final type = map['type'] as String;
    return switch (type) {
      'string' => StringSchemaValue.fromJson(map),
      'url' => UrlSchemaValue.fromJson(map),
      'uuid' => UuidSchemaValue.fromJson(map),
      'bytes' => BytesSchemaValue.fromJson(map),
      'ssh-key' => SshKeySchemaValue.fromJson(map),
      _ => throw UnsupportedError('Unsupported schema value type: $type'),
    };
  }
}

final class StringSchemaValue extends SchemaValue {
  final String? value;
  const StringSchemaValue({required super.showQrCode, this.value});

  factory StringSchemaValue.fromJson(Map<String, dynamic> map) {
    return StringSchemaValue(
      value: map['default'] as String?,
      showQrCode: (map['show-qr-code'] as bool?) ?? false,
    );
  }
}

final class UrlSchemaValue extends SchemaValue {
  final String? value;
  const UrlSchemaValue({required super.showQrCode, this.value});

  factory UrlSchemaValue.fromJson(Map<String, dynamic> map) {
    return UrlSchemaValue(
      value: map['default'] as String?,
      showQrCode: (map['show-qr-code'] as bool?) ?? false,
    );
  }
}

final class UuidSchemaValue extends SchemaValue {
  final String? value;
  const UuidSchemaValue({required super.showQrCode, this.value});

  factory UuidSchemaValue.fromJson(Map<String, dynamic> map) {
    return UuidSchemaValue(
      value: map['default'] as String?,
      showQrCode: (map['show-qr-code'] as bool?) ?? false,
    );
  }
}

enum BytesCategory {
  password;

  factory BytesCategory.fromString(String value) {
    switch (value) {
      case 'password':
        return BytesCategory.password;
      default:
        throw ArgumentError('Unknown bytes category: $value');
    }
  }
}

enum BytesGenerator {
  crytoRandom;

  factory BytesGenerator.fromString(String value) {
    switch (value) {
      case 'crypto-random':
        return BytesGenerator.crytoRandom;
      default:
        throw ArgumentError('Unknown bytes generator: $value');
    }
  }
}

final class BytesSchemaValue extends SchemaValue {
  final Uint8List? value;
  final int? min;
  final int? max;
  final BytesCategory? category;
  final BytesGenerator? generator;

  const BytesSchemaValue({
    required super.showQrCode,
    this.value,
    this.min,
    this.max,
    this.category,
    this.generator,
  });

  factory BytesSchemaValue.fromJson(Map<String, dynamic> map) {
    final bytes = map['default'] as String?;
    final min = map['min'] as int?;
    final max = map['max'] as int?;
    final category = BytesCategory.fromString(map['category'] as String);
    final generator = BytesGenerator.fromString(map['generator'] as String);

    Uint8List? value;
    if (bytes != null) {
      value = base64Url.decode(bytes);
    }

    return BytesSchemaValue(
      value: value,
      min: min,
      max: max,
      category: category,
      generator: generator,
      showQrCode: (map['show-qr-code'] as bool?) ?? false,
    );
  }
}

final class SshKeySchemaValue extends SchemaValue {
  final String? value;
  const SshKeySchemaValue({required super.showQrCode, this.value});

  factory SshKeySchemaValue.fromJson(Map<String, dynamic> map) {
    return SshKeySchemaValue(
      value: map['default'] as String?,
      showQrCode: (map['show-qr-code'] as bool?) ?? false,
    );
  }
}

enum ValueType {
  url,
  string,
  bytes,
  uuid;

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
      case ValueType.uuid:
        return 'uuid';
    }
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
        final value = json['content'] as String?;
        content = value == null ? null : UuidValue.withValidation(value);
      case ValueType.bytes:
        final value = json['content'] as String?;
        content = value == null ? null : Base64Codec.urlSafe().decode(value);
    }

    return Value(type: type, content: content);
  }

  Map<String, dynamic> toJson() {
    dynamic result;
    switch (type) {
      case ValueType.url:
      case ValueType.string:
        result = content;
      case ValueType.bytes:
        final bytes = content as Uint8List?;
        result = bytes == null ? null : Base64Codec.urlSafe().encode(bytes);
      case ValueType.uuid:
        result = (content as UuidValue?)?.toString();
    }
    return {'type': type.toJsonString(), 'content': result};
  }

  @override
  String toString() {
    final map = toJson();
    return '${map['type']}:${map['content']}';
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
  final UuidValue backendId;
  const BackendConfigRoute({
    super.key,
    required this.repository,
    required this.backendId,
  });

  @override
  State<BackendConfigRoute> createState() => _BackendConfigRouteState();
}

class _BackendConfigRouteState extends State<BackendConfigRoute> {
  Future<BackendRecord?>? _backend;

  @override
  void initState() {
    super.initState();
    _backend = widget.repository.backend(id: widget.backendId);
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder(
      future: _backend,
      builder: (context, snapshot) {
        if (snapshot.hasError) {
          context.read<LogBloc>().add(LogErrorEvent(error: snapshot.error!));
        }
        if (snapshot.connectionState != ConnectionState.done) {
          return Center(child: CircularProgressIndicator.adaptive());
        }

        final record = snapshot.data!;
        return Scaffold(
          appBar: AppBar(title: Text(record.name)),
          body: _ConfigSection(repository: widget.repository, record: record),
        );
      },
    );
  }
}

class _ConfigSection extends StatefulWidget {
  final Repository repository;
  final BackendRecord record;
  const _ConfigSection({required this.record, required this.repository});

  @override
  State<_ConfigSection> createState() => _ConfigSectionState();
}

class _ConfigSectionState extends State<_ConfigSection> {
  Future<List<SshKey>>? _sshKeys;

  late Schema _schema;
  late Config _config;

  @override
  void initState() {
    super.initState();

    _sshKeys = sshKeys();
    final configJson = jsonDecode(widget.record.config) as Map<String, dynamic>;
    final schemaJson = jsonDecode(widget.record.schema) as Map<String, dynamic>;

    _schema = Schema.fromJson(schemaJson);
    _config = Config.fromJson(configJson);
  }

  @override
  Widget build(BuildContext context) {
    final tiles = <Widget>[];
    for (final entry in _schema.fields.entries) {
      final name = entry.key;
      final field = entry.value;
      final value = _config.fields[name];

      switch (field) {
        case UrlSchemaValue(value: final defaultValue):
        case StringSchemaValue(value: final defaultValue):
          tiles.add(
            SettingsTileText(
              title: Text(name),
              text: (value?.content as String?) ?? defaultValue ?? '',
              onChanged: (text) async {
                setState(() {
                  _config.fields[name] = Value(
                    type: ValueType.string,
                    content: text,
                  );
                });
                await _save();
              },
            ),
          );

        case BytesSchemaValue():
          tiles.add(_bytesProperty(field, name, value, context));

        case SshKeySchemaValue():
          final uuid = value?.content as UuidValue?;
          tiles.add(_sshKeyProperty(name, uuid));

        case UuidSchemaValue():
          tiles.add(_uuidProperty(name, value));
      }
    }
    return SettingsList(sections: [SettingsSection(tiles: tiles)]);
  }

  SettingsTileUuid _uuidProperty(String name, Value? value) {
    return SettingsTileUuid(
      title: Text(name),
      value: value?.content as UuidValue?,
      onChanged: (uuid) async {
        setState(() {
          _config.fields[name] = Value(type: ValueType.uuid, content: uuid);
        });
        await _save();
      },
    );
  }

  FutureBuilder<List<SshKey>> _sshKeyProperty(String name, UuidValue? uuid) {
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
              _config.fields[name] = Value(type: ValueType.uuid, content: uuid);
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
                    _config.fields[name] = Value(
                      type: ValueType.uuid,
                      content: key,
                    );
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
    BytesSchemaValue field,
    String name,
    Value? value,
    BuildContext context,
  ) {
    final bytes = value?.content as Uint8List?;

    final BytesSchemaValue(:min, :max, :category, :generator) = field;
    return SettingsTileText(
      obscureText: category == BytesCategory.password,
      title: Text(name),
      text: bytes == null ? '' : base64Url.encode(bytes),
      onChanged: (text) async {
        setState(() {
          _config.fields[name] = Value(
            type: ValueType.bytes,
            content: base64Url.decode(text),
          );
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
            return 'input too small must be $max length, got: ${bytes.length}';
          }
        } on FormatException catch (error) {
          return error.message;
        }
        return null;
      },
      trailing: Wrap(
        children: [
          if (field.showQrCode && value != null)
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
                                data: value.toString(),
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
          if (generator == BytesGenerator.crytoRandom)
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
                      _config.fields[name] = Value(
                        type: ValueType.bytes,
                        content: key,
                      );
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
    final json = _config.toJson();
    final backend = BackendRecord(
      id: widget.record.id,
      name: widget.record.name,
      enabled: widget.record.enabled,
      schema: '',
      config: jsonEncode(json),
    );
    return widget.repository.updateBackend(backend: backend);
  }
}
