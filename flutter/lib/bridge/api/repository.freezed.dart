// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'repository.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$ConnectionError {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) network,
    required TResult Function() noSshKeysProvided,
    required TResult Function(String message) authentication,
    required TResult Function(
            String hostname, HostKeyType keyType, String hostKey)
        unknownHost,
    required TResult Function(String hostname) missingHostKey,
    required TResult Function() unknownKeyType,
    required TResult Function(String expected, String actual)
        missmatchRemoteKey,
    required TResult Function(String message) other,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? network,
    TResult? Function()? noSshKeysProvided,
    TResult? Function(String message)? authentication,
    TResult? Function(String hostname, HostKeyType keyType, String hostKey)?
        unknownHost,
    TResult? Function(String hostname)? missingHostKey,
    TResult? Function()? unknownKeyType,
    TResult? Function(String expected, String actual)? missmatchRemoteKey,
    TResult? Function(String message)? other,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? network,
    TResult Function()? noSshKeysProvided,
    TResult Function(String message)? authentication,
    TResult Function(String hostname, HostKeyType keyType, String hostKey)?
        unknownHost,
    TResult Function(String hostname)? missingHostKey,
    TResult Function()? unknownKeyType,
    TResult Function(String expected, String actual)? missmatchRemoteKey,
    TResult Function(String message)? other,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ConnectionError_Network value) network,
    required TResult Function(ConnectionError_NoSshKeysProvided value)
        noSshKeysProvided,
    required TResult Function(ConnectionError_Authentication value)
        authentication,
    required TResult Function(ConnectionError_UnknownHost value) unknownHost,
    required TResult Function(ConnectionError_MissingHostKey value)
        missingHostKey,
    required TResult Function(ConnectionError_UnknownKeyType value)
        unknownKeyType,
    required TResult Function(ConnectionError_MissmatchRemoteKey value)
        missmatchRemoteKey,
    required TResult Function(ConnectionError_Other value) other,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ConnectionError_Network value)? network,
    TResult? Function(ConnectionError_NoSshKeysProvided value)?
        noSshKeysProvided,
    TResult? Function(ConnectionError_Authentication value)? authentication,
    TResult? Function(ConnectionError_UnknownHost value)? unknownHost,
    TResult? Function(ConnectionError_MissingHostKey value)? missingHostKey,
    TResult? Function(ConnectionError_UnknownKeyType value)? unknownKeyType,
    TResult? Function(ConnectionError_MissmatchRemoteKey value)?
        missmatchRemoteKey,
    TResult? Function(ConnectionError_Other value)? other,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ConnectionError_Network value)? network,
    TResult Function(ConnectionError_NoSshKeysProvided value)?
        noSshKeysProvided,
    TResult Function(ConnectionError_Authentication value)? authentication,
    TResult Function(ConnectionError_UnknownHost value)? unknownHost,
    TResult Function(ConnectionError_MissingHostKey value)? missingHostKey,
    TResult Function(ConnectionError_UnknownKeyType value)? unknownKeyType,
    TResult Function(ConnectionError_MissmatchRemoteKey value)?
        missmatchRemoteKey,
    TResult Function(ConnectionError_Other value)? other,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ConnectionErrorCopyWith<$Res> {
  factory $ConnectionErrorCopyWith(
          ConnectionError value, $Res Function(ConnectionError) then) =
      _$ConnectionErrorCopyWithImpl<$Res, ConnectionError>;
}

/// @nodoc
class _$ConnectionErrorCopyWithImpl<$Res, $Val extends ConnectionError>
    implements $ConnectionErrorCopyWith<$Res> {
  _$ConnectionErrorCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$ConnectionError_NetworkImplCopyWith<$Res> {
  factory _$$ConnectionError_NetworkImplCopyWith(
          _$ConnectionError_NetworkImpl value,
          $Res Function(_$ConnectionError_NetworkImpl) then) =
      __$$ConnectionError_NetworkImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String message});
}

/// @nodoc
class __$$ConnectionError_NetworkImplCopyWithImpl<$Res>
    extends _$ConnectionErrorCopyWithImpl<$Res, _$ConnectionError_NetworkImpl>
    implements _$$ConnectionError_NetworkImplCopyWith<$Res> {
  __$$ConnectionError_NetworkImplCopyWithImpl(
      _$ConnectionError_NetworkImpl _value,
      $Res Function(_$ConnectionError_NetworkImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? message = null,
  }) {
    return _then(_$ConnectionError_NetworkImpl(
      message: null == message
          ? _value.message
          : message // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$ConnectionError_NetworkImpl extends ConnectionError_Network {
  const _$ConnectionError_NetworkImpl({required this.message}) : super._();

  @override
  final String message;

  @override
  String toString() {
    return 'ConnectionError.network(message: $message)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ConnectionError_NetworkImpl &&
            (identical(other.message, message) || other.message == message));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ConnectionError_NetworkImplCopyWith<_$ConnectionError_NetworkImpl>
      get copyWith => __$$ConnectionError_NetworkImplCopyWithImpl<
          _$ConnectionError_NetworkImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) network,
    required TResult Function() noSshKeysProvided,
    required TResult Function(String message) authentication,
    required TResult Function(
            String hostname, HostKeyType keyType, String hostKey)
        unknownHost,
    required TResult Function(String hostname) missingHostKey,
    required TResult Function() unknownKeyType,
    required TResult Function(String expected, String actual)
        missmatchRemoteKey,
    required TResult Function(String message) other,
  }) {
    return network(message);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? network,
    TResult? Function()? noSshKeysProvided,
    TResult? Function(String message)? authentication,
    TResult? Function(String hostname, HostKeyType keyType, String hostKey)?
        unknownHost,
    TResult? Function(String hostname)? missingHostKey,
    TResult? Function()? unknownKeyType,
    TResult? Function(String expected, String actual)? missmatchRemoteKey,
    TResult? Function(String message)? other,
  }) {
    return network?.call(message);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? network,
    TResult Function()? noSshKeysProvided,
    TResult Function(String message)? authentication,
    TResult Function(String hostname, HostKeyType keyType, String hostKey)?
        unknownHost,
    TResult Function(String hostname)? missingHostKey,
    TResult Function()? unknownKeyType,
    TResult Function(String expected, String actual)? missmatchRemoteKey,
    TResult Function(String message)? other,
    required TResult orElse(),
  }) {
    if (network != null) {
      return network(message);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ConnectionError_Network value) network,
    required TResult Function(ConnectionError_NoSshKeysProvided value)
        noSshKeysProvided,
    required TResult Function(ConnectionError_Authentication value)
        authentication,
    required TResult Function(ConnectionError_UnknownHost value) unknownHost,
    required TResult Function(ConnectionError_MissingHostKey value)
        missingHostKey,
    required TResult Function(ConnectionError_UnknownKeyType value)
        unknownKeyType,
    required TResult Function(ConnectionError_MissmatchRemoteKey value)
        missmatchRemoteKey,
    required TResult Function(ConnectionError_Other value) other,
  }) {
    return network(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ConnectionError_Network value)? network,
    TResult? Function(ConnectionError_NoSshKeysProvided value)?
        noSshKeysProvided,
    TResult? Function(ConnectionError_Authentication value)? authentication,
    TResult? Function(ConnectionError_UnknownHost value)? unknownHost,
    TResult? Function(ConnectionError_MissingHostKey value)? missingHostKey,
    TResult? Function(ConnectionError_UnknownKeyType value)? unknownKeyType,
    TResult? Function(ConnectionError_MissmatchRemoteKey value)?
        missmatchRemoteKey,
    TResult? Function(ConnectionError_Other value)? other,
  }) {
    return network?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ConnectionError_Network value)? network,
    TResult Function(ConnectionError_NoSshKeysProvided value)?
        noSshKeysProvided,
    TResult Function(ConnectionError_Authentication value)? authentication,
    TResult Function(ConnectionError_UnknownHost value)? unknownHost,
    TResult Function(ConnectionError_MissingHostKey value)? missingHostKey,
    TResult Function(ConnectionError_UnknownKeyType value)? unknownKeyType,
    TResult Function(ConnectionError_MissmatchRemoteKey value)?
        missmatchRemoteKey,
    TResult Function(ConnectionError_Other value)? other,
    required TResult orElse(),
  }) {
    if (network != null) {
      return network(this);
    }
    return orElse();
  }
}

abstract class ConnectionError_Network extends ConnectionError {
  const factory ConnectionError_Network({required final String message}) =
      _$ConnectionError_NetworkImpl;
  const ConnectionError_Network._() : super._();

  String get message;
  @JsonKey(ignore: true)
  _$$ConnectionError_NetworkImplCopyWith<_$ConnectionError_NetworkImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ConnectionError_NoSshKeysProvidedImplCopyWith<$Res> {
  factory _$$ConnectionError_NoSshKeysProvidedImplCopyWith(
          _$ConnectionError_NoSshKeysProvidedImpl value,
          $Res Function(_$ConnectionError_NoSshKeysProvidedImpl) then) =
      __$$ConnectionError_NoSshKeysProvidedImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$ConnectionError_NoSshKeysProvidedImplCopyWithImpl<$Res>
    extends _$ConnectionErrorCopyWithImpl<$Res,
        _$ConnectionError_NoSshKeysProvidedImpl>
    implements _$$ConnectionError_NoSshKeysProvidedImplCopyWith<$Res> {
  __$$ConnectionError_NoSshKeysProvidedImplCopyWithImpl(
      _$ConnectionError_NoSshKeysProvidedImpl _value,
      $Res Function(_$ConnectionError_NoSshKeysProvidedImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$ConnectionError_NoSshKeysProvidedImpl
    extends ConnectionError_NoSshKeysProvided {
  const _$ConnectionError_NoSshKeysProvidedImpl() : super._();

  @override
  String toString() {
    return 'ConnectionError.noSshKeysProvided()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ConnectionError_NoSshKeysProvidedImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) network,
    required TResult Function() noSshKeysProvided,
    required TResult Function(String message) authentication,
    required TResult Function(
            String hostname, HostKeyType keyType, String hostKey)
        unknownHost,
    required TResult Function(String hostname) missingHostKey,
    required TResult Function() unknownKeyType,
    required TResult Function(String expected, String actual)
        missmatchRemoteKey,
    required TResult Function(String message) other,
  }) {
    return noSshKeysProvided();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? network,
    TResult? Function()? noSshKeysProvided,
    TResult? Function(String message)? authentication,
    TResult? Function(String hostname, HostKeyType keyType, String hostKey)?
        unknownHost,
    TResult? Function(String hostname)? missingHostKey,
    TResult? Function()? unknownKeyType,
    TResult? Function(String expected, String actual)? missmatchRemoteKey,
    TResult? Function(String message)? other,
  }) {
    return noSshKeysProvided?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? network,
    TResult Function()? noSshKeysProvided,
    TResult Function(String message)? authentication,
    TResult Function(String hostname, HostKeyType keyType, String hostKey)?
        unknownHost,
    TResult Function(String hostname)? missingHostKey,
    TResult Function()? unknownKeyType,
    TResult Function(String expected, String actual)? missmatchRemoteKey,
    TResult Function(String message)? other,
    required TResult orElse(),
  }) {
    if (noSshKeysProvided != null) {
      return noSshKeysProvided();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ConnectionError_Network value) network,
    required TResult Function(ConnectionError_NoSshKeysProvided value)
        noSshKeysProvided,
    required TResult Function(ConnectionError_Authentication value)
        authentication,
    required TResult Function(ConnectionError_UnknownHost value) unknownHost,
    required TResult Function(ConnectionError_MissingHostKey value)
        missingHostKey,
    required TResult Function(ConnectionError_UnknownKeyType value)
        unknownKeyType,
    required TResult Function(ConnectionError_MissmatchRemoteKey value)
        missmatchRemoteKey,
    required TResult Function(ConnectionError_Other value) other,
  }) {
    return noSshKeysProvided(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ConnectionError_Network value)? network,
    TResult? Function(ConnectionError_NoSshKeysProvided value)?
        noSshKeysProvided,
    TResult? Function(ConnectionError_Authentication value)? authentication,
    TResult? Function(ConnectionError_UnknownHost value)? unknownHost,
    TResult? Function(ConnectionError_MissingHostKey value)? missingHostKey,
    TResult? Function(ConnectionError_UnknownKeyType value)? unknownKeyType,
    TResult? Function(ConnectionError_MissmatchRemoteKey value)?
        missmatchRemoteKey,
    TResult? Function(ConnectionError_Other value)? other,
  }) {
    return noSshKeysProvided?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ConnectionError_Network value)? network,
    TResult Function(ConnectionError_NoSshKeysProvided value)?
        noSshKeysProvided,
    TResult Function(ConnectionError_Authentication value)? authentication,
    TResult Function(ConnectionError_UnknownHost value)? unknownHost,
    TResult Function(ConnectionError_MissingHostKey value)? missingHostKey,
    TResult Function(ConnectionError_UnknownKeyType value)? unknownKeyType,
    TResult Function(ConnectionError_MissmatchRemoteKey value)?
        missmatchRemoteKey,
    TResult Function(ConnectionError_Other value)? other,
    required TResult orElse(),
  }) {
    if (noSshKeysProvided != null) {
      return noSshKeysProvided(this);
    }
    return orElse();
  }
}

abstract class ConnectionError_NoSshKeysProvided extends ConnectionError {
  const factory ConnectionError_NoSshKeysProvided() =
      _$ConnectionError_NoSshKeysProvidedImpl;
  const ConnectionError_NoSshKeysProvided._() : super._();
}

/// @nodoc
abstract class _$$ConnectionError_AuthenticationImplCopyWith<$Res> {
  factory _$$ConnectionError_AuthenticationImplCopyWith(
          _$ConnectionError_AuthenticationImpl value,
          $Res Function(_$ConnectionError_AuthenticationImpl) then) =
      __$$ConnectionError_AuthenticationImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String message});
}

/// @nodoc
class __$$ConnectionError_AuthenticationImplCopyWithImpl<$Res>
    extends _$ConnectionErrorCopyWithImpl<$Res,
        _$ConnectionError_AuthenticationImpl>
    implements _$$ConnectionError_AuthenticationImplCopyWith<$Res> {
  __$$ConnectionError_AuthenticationImplCopyWithImpl(
      _$ConnectionError_AuthenticationImpl _value,
      $Res Function(_$ConnectionError_AuthenticationImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? message = null,
  }) {
    return _then(_$ConnectionError_AuthenticationImpl(
      message: null == message
          ? _value.message
          : message // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$ConnectionError_AuthenticationImpl
    extends ConnectionError_Authentication {
  const _$ConnectionError_AuthenticationImpl({required this.message})
      : super._();

  @override
  final String message;

  @override
  String toString() {
    return 'ConnectionError.authentication(message: $message)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ConnectionError_AuthenticationImpl &&
            (identical(other.message, message) || other.message == message));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ConnectionError_AuthenticationImplCopyWith<
          _$ConnectionError_AuthenticationImpl>
      get copyWith => __$$ConnectionError_AuthenticationImplCopyWithImpl<
          _$ConnectionError_AuthenticationImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) network,
    required TResult Function() noSshKeysProvided,
    required TResult Function(String message) authentication,
    required TResult Function(
            String hostname, HostKeyType keyType, String hostKey)
        unknownHost,
    required TResult Function(String hostname) missingHostKey,
    required TResult Function() unknownKeyType,
    required TResult Function(String expected, String actual)
        missmatchRemoteKey,
    required TResult Function(String message) other,
  }) {
    return authentication(message);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? network,
    TResult? Function()? noSshKeysProvided,
    TResult? Function(String message)? authentication,
    TResult? Function(String hostname, HostKeyType keyType, String hostKey)?
        unknownHost,
    TResult? Function(String hostname)? missingHostKey,
    TResult? Function()? unknownKeyType,
    TResult? Function(String expected, String actual)? missmatchRemoteKey,
    TResult? Function(String message)? other,
  }) {
    return authentication?.call(message);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? network,
    TResult Function()? noSshKeysProvided,
    TResult Function(String message)? authentication,
    TResult Function(String hostname, HostKeyType keyType, String hostKey)?
        unknownHost,
    TResult Function(String hostname)? missingHostKey,
    TResult Function()? unknownKeyType,
    TResult Function(String expected, String actual)? missmatchRemoteKey,
    TResult Function(String message)? other,
    required TResult orElse(),
  }) {
    if (authentication != null) {
      return authentication(message);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ConnectionError_Network value) network,
    required TResult Function(ConnectionError_NoSshKeysProvided value)
        noSshKeysProvided,
    required TResult Function(ConnectionError_Authentication value)
        authentication,
    required TResult Function(ConnectionError_UnknownHost value) unknownHost,
    required TResult Function(ConnectionError_MissingHostKey value)
        missingHostKey,
    required TResult Function(ConnectionError_UnknownKeyType value)
        unknownKeyType,
    required TResult Function(ConnectionError_MissmatchRemoteKey value)
        missmatchRemoteKey,
    required TResult Function(ConnectionError_Other value) other,
  }) {
    return authentication(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ConnectionError_Network value)? network,
    TResult? Function(ConnectionError_NoSshKeysProvided value)?
        noSshKeysProvided,
    TResult? Function(ConnectionError_Authentication value)? authentication,
    TResult? Function(ConnectionError_UnknownHost value)? unknownHost,
    TResult? Function(ConnectionError_MissingHostKey value)? missingHostKey,
    TResult? Function(ConnectionError_UnknownKeyType value)? unknownKeyType,
    TResult? Function(ConnectionError_MissmatchRemoteKey value)?
        missmatchRemoteKey,
    TResult? Function(ConnectionError_Other value)? other,
  }) {
    return authentication?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ConnectionError_Network value)? network,
    TResult Function(ConnectionError_NoSshKeysProvided value)?
        noSshKeysProvided,
    TResult Function(ConnectionError_Authentication value)? authentication,
    TResult Function(ConnectionError_UnknownHost value)? unknownHost,
    TResult Function(ConnectionError_MissingHostKey value)? missingHostKey,
    TResult Function(ConnectionError_UnknownKeyType value)? unknownKeyType,
    TResult Function(ConnectionError_MissmatchRemoteKey value)?
        missmatchRemoteKey,
    TResult Function(ConnectionError_Other value)? other,
    required TResult orElse(),
  }) {
    if (authentication != null) {
      return authentication(this);
    }
    return orElse();
  }
}

abstract class ConnectionError_Authentication extends ConnectionError {
  const factory ConnectionError_Authentication(
      {required final String message}) = _$ConnectionError_AuthenticationImpl;
  const ConnectionError_Authentication._() : super._();

  String get message;
  @JsonKey(ignore: true)
  _$$ConnectionError_AuthenticationImplCopyWith<
          _$ConnectionError_AuthenticationImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ConnectionError_UnknownHostImplCopyWith<$Res> {
  factory _$$ConnectionError_UnknownHostImplCopyWith(
          _$ConnectionError_UnknownHostImpl value,
          $Res Function(_$ConnectionError_UnknownHostImpl) then) =
      __$$ConnectionError_UnknownHostImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String hostname, HostKeyType keyType, String hostKey});
}

/// @nodoc
class __$$ConnectionError_UnknownHostImplCopyWithImpl<$Res>
    extends _$ConnectionErrorCopyWithImpl<$Res,
        _$ConnectionError_UnknownHostImpl>
    implements _$$ConnectionError_UnknownHostImplCopyWith<$Res> {
  __$$ConnectionError_UnknownHostImplCopyWithImpl(
      _$ConnectionError_UnknownHostImpl _value,
      $Res Function(_$ConnectionError_UnknownHostImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? hostname = null,
    Object? keyType = null,
    Object? hostKey = null,
  }) {
    return _then(_$ConnectionError_UnknownHostImpl(
      hostname: null == hostname
          ? _value.hostname
          : hostname // ignore: cast_nullable_to_non_nullable
              as String,
      keyType: null == keyType
          ? _value.keyType
          : keyType // ignore: cast_nullable_to_non_nullable
              as HostKeyType,
      hostKey: null == hostKey
          ? _value.hostKey
          : hostKey // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$ConnectionError_UnknownHostImpl extends ConnectionError_UnknownHost {
  const _$ConnectionError_UnknownHostImpl(
      {required this.hostname, required this.keyType, required this.hostKey})
      : super._();

  @override
  final String hostname;
  @override
  final HostKeyType keyType;
  @override
  final String hostKey;

  @override
  String toString() {
    return 'ConnectionError.unknownHost(hostname: $hostname, keyType: $keyType, hostKey: $hostKey)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ConnectionError_UnknownHostImpl &&
            (identical(other.hostname, hostname) ||
                other.hostname == hostname) &&
            (identical(other.keyType, keyType) || other.keyType == keyType) &&
            (identical(other.hostKey, hostKey) || other.hostKey == hostKey));
  }

  @override
  int get hashCode => Object.hash(runtimeType, hostname, keyType, hostKey);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ConnectionError_UnknownHostImplCopyWith<_$ConnectionError_UnknownHostImpl>
      get copyWith => __$$ConnectionError_UnknownHostImplCopyWithImpl<
          _$ConnectionError_UnknownHostImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) network,
    required TResult Function() noSshKeysProvided,
    required TResult Function(String message) authentication,
    required TResult Function(
            String hostname, HostKeyType keyType, String hostKey)
        unknownHost,
    required TResult Function(String hostname) missingHostKey,
    required TResult Function() unknownKeyType,
    required TResult Function(String expected, String actual)
        missmatchRemoteKey,
    required TResult Function(String message) other,
  }) {
    return unknownHost(hostname, keyType, hostKey);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? network,
    TResult? Function()? noSshKeysProvided,
    TResult? Function(String message)? authentication,
    TResult? Function(String hostname, HostKeyType keyType, String hostKey)?
        unknownHost,
    TResult? Function(String hostname)? missingHostKey,
    TResult? Function()? unknownKeyType,
    TResult? Function(String expected, String actual)? missmatchRemoteKey,
    TResult? Function(String message)? other,
  }) {
    return unknownHost?.call(hostname, keyType, hostKey);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? network,
    TResult Function()? noSshKeysProvided,
    TResult Function(String message)? authentication,
    TResult Function(String hostname, HostKeyType keyType, String hostKey)?
        unknownHost,
    TResult Function(String hostname)? missingHostKey,
    TResult Function()? unknownKeyType,
    TResult Function(String expected, String actual)? missmatchRemoteKey,
    TResult Function(String message)? other,
    required TResult orElse(),
  }) {
    if (unknownHost != null) {
      return unknownHost(hostname, keyType, hostKey);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ConnectionError_Network value) network,
    required TResult Function(ConnectionError_NoSshKeysProvided value)
        noSshKeysProvided,
    required TResult Function(ConnectionError_Authentication value)
        authentication,
    required TResult Function(ConnectionError_UnknownHost value) unknownHost,
    required TResult Function(ConnectionError_MissingHostKey value)
        missingHostKey,
    required TResult Function(ConnectionError_UnknownKeyType value)
        unknownKeyType,
    required TResult Function(ConnectionError_MissmatchRemoteKey value)
        missmatchRemoteKey,
    required TResult Function(ConnectionError_Other value) other,
  }) {
    return unknownHost(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ConnectionError_Network value)? network,
    TResult? Function(ConnectionError_NoSshKeysProvided value)?
        noSshKeysProvided,
    TResult? Function(ConnectionError_Authentication value)? authentication,
    TResult? Function(ConnectionError_UnknownHost value)? unknownHost,
    TResult? Function(ConnectionError_MissingHostKey value)? missingHostKey,
    TResult? Function(ConnectionError_UnknownKeyType value)? unknownKeyType,
    TResult? Function(ConnectionError_MissmatchRemoteKey value)?
        missmatchRemoteKey,
    TResult? Function(ConnectionError_Other value)? other,
  }) {
    return unknownHost?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ConnectionError_Network value)? network,
    TResult Function(ConnectionError_NoSshKeysProvided value)?
        noSshKeysProvided,
    TResult Function(ConnectionError_Authentication value)? authentication,
    TResult Function(ConnectionError_UnknownHost value)? unknownHost,
    TResult Function(ConnectionError_MissingHostKey value)? missingHostKey,
    TResult Function(ConnectionError_UnknownKeyType value)? unknownKeyType,
    TResult Function(ConnectionError_MissmatchRemoteKey value)?
        missmatchRemoteKey,
    TResult Function(ConnectionError_Other value)? other,
    required TResult orElse(),
  }) {
    if (unknownHost != null) {
      return unknownHost(this);
    }
    return orElse();
  }
}

abstract class ConnectionError_UnknownHost extends ConnectionError {
  const factory ConnectionError_UnknownHost(
      {required final String hostname,
      required final HostKeyType keyType,
      required final String hostKey}) = _$ConnectionError_UnknownHostImpl;
  const ConnectionError_UnknownHost._() : super._();

  String get hostname;
  HostKeyType get keyType;
  String get hostKey;
  @JsonKey(ignore: true)
  _$$ConnectionError_UnknownHostImplCopyWith<_$ConnectionError_UnknownHostImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ConnectionError_MissingHostKeyImplCopyWith<$Res> {
  factory _$$ConnectionError_MissingHostKeyImplCopyWith(
          _$ConnectionError_MissingHostKeyImpl value,
          $Res Function(_$ConnectionError_MissingHostKeyImpl) then) =
      __$$ConnectionError_MissingHostKeyImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String hostname});
}

/// @nodoc
class __$$ConnectionError_MissingHostKeyImplCopyWithImpl<$Res>
    extends _$ConnectionErrorCopyWithImpl<$Res,
        _$ConnectionError_MissingHostKeyImpl>
    implements _$$ConnectionError_MissingHostKeyImplCopyWith<$Res> {
  __$$ConnectionError_MissingHostKeyImplCopyWithImpl(
      _$ConnectionError_MissingHostKeyImpl _value,
      $Res Function(_$ConnectionError_MissingHostKeyImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? hostname = null,
  }) {
    return _then(_$ConnectionError_MissingHostKeyImpl(
      hostname: null == hostname
          ? _value.hostname
          : hostname // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$ConnectionError_MissingHostKeyImpl
    extends ConnectionError_MissingHostKey {
  const _$ConnectionError_MissingHostKeyImpl({required this.hostname})
      : super._();

  @override
  final String hostname;

  @override
  String toString() {
    return 'ConnectionError.missingHostKey(hostname: $hostname)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ConnectionError_MissingHostKeyImpl &&
            (identical(other.hostname, hostname) ||
                other.hostname == hostname));
  }

  @override
  int get hashCode => Object.hash(runtimeType, hostname);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ConnectionError_MissingHostKeyImplCopyWith<
          _$ConnectionError_MissingHostKeyImpl>
      get copyWith => __$$ConnectionError_MissingHostKeyImplCopyWithImpl<
          _$ConnectionError_MissingHostKeyImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) network,
    required TResult Function() noSshKeysProvided,
    required TResult Function(String message) authentication,
    required TResult Function(
            String hostname, HostKeyType keyType, String hostKey)
        unknownHost,
    required TResult Function(String hostname) missingHostKey,
    required TResult Function() unknownKeyType,
    required TResult Function(String expected, String actual)
        missmatchRemoteKey,
    required TResult Function(String message) other,
  }) {
    return missingHostKey(hostname);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? network,
    TResult? Function()? noSshKeysProvided,
    TResult? Function(String message)? authentication,
    TResult? Function(String hostname, HostKeyType keyType, String hostKey)?
        unknownHost,
    TResult? Function(String hostname)? missingHostKey,
    TResult? Function()? unknownKeyType,
    TResult? Function(String expected, String actual)? missmatchRemoteKey,
    TResult? Function(String message)? other,
  }) {
    return missingHostKey?.call(hostname);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? network,
    TResult Function()? noSshKeysProvided,
    TResult Function(String message)? authentication,
    TResult Function(String hostname, HostKeyType keyType, String hostKey)?
        unknownHost,
    TResult Function(String hostname)? missingHostKey,
    TResult Function()? unknownKeyType,
    TResult Function(String expected, String actual)? missmatchRemoteKey,
    TResult Function(String message)? other,
    required TResult orElse(),
  }) {
    if (missingHostKey != null) {
      return missingHostKey(hostname);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ConnectionError_Network value) network,
    required TResult Function(ConnectionError_NoSshKeysProvided value)
        noSshKeysProvided,
    required TResult Function(ConnectionError_Authentication value)
        authentication,
    required TResult Function(ConnectionError_UnknownHost value) unknownHost,
    required TResult Function(ConnectionError_MissingHostKey value)
        missingHostKey,
    required TResult Function(ConnectionError_UnknownKeyType value)
        unknownKeyType,
    required TResult Function(ConnectionError_MissmatchRemoteKey value)
        missmatchRemoteKey,
    required TResult Function(ConnectionError_Other value) other,
  }) {
    return missingHostKey(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ConnectionError_Network value)? network,
    TResult? Function(ConnectionError_NoSshKeysProvided value)?
        noSshKeysProvided,
    TResult? Function(ConnectionError_Authentication value)? authentication,
    TResult? Function(ConnectionError_UnknownHost value)? unknownHost,
    TResult? Function(ConnectionError_MissingHostKey value)? missingHostKey,
    TResult? Function(ConnectionError_UnknownKeyType value)? unknownKeyType,
    TResult? Function(ConnectionError_MissmatchRemoteKey value)?
        missmatchRemoteKey,
    TResult? Function(ConnectionError_Other value)? other,
  }) {
    return missingHostKey?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ConnectionError_Network value)? network,
    TResult Function(ConnectionError_NoSshKeysProvided value)?
        noSshKeysProvided,
    TResult Function(ConnectionError_Authentication value)? authentication,
    TResult Function(ConnectionError_UnknownHost value)? unknownHost,
    TResult Function(ConnectionError_MissingHostKey value)? missingHostKey,
    TResult Function(ConnectionError_UnknownKeyType value)? unknownKeyType,
    TResult Function(ConnectionError_MissmatchRemoteKey value)?
        missmatchRemoteKey,
    TResult Function(ConnectionError_Other value)? other,
    required TResult orElse(),
  }) {
    if (missingHostKey != null) {
      return missingHostKey(this);
    }
    return orElse();
  }
}

abstract class ConnectionError_MissingHostKey extends ConnectionError {
  const factory ConnectionError_MissingHostKey(
      {required final String hostname}) = _$ConnectionError_MissingHostKeyImpl;
  const ConnectionError_MissingHostKey._() : super._();

  String get hostname;
  @JsonKey(ignore: true)
  _$$ConnectionError_MissingHostKeyImplCopyWith<
          _$ConnectionError_MissingHostKeyImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ConnectionError_UnknownKeyTypeImplCopyWith<$Res> {
  factory _$$ConnectionError_UnknownKeyTypeImplCopyWith(
          _$ConnectionError_UnknownKeyTypeImpl value,
          $Res Function(_$ConnectionError_UnknownKeyTypeImpl) then) =
      __$$ConnectionError_UnknownKeyTypeImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$ConnectionError_UnknownKeyTypeImplCopyWithImpl<$Res>
    extends _$ConnectionErrorCopyWithImpl<$Res,
        _$ConnectionError_UnknownKeyTypeImpl>
    implements _$$ConnectionError_UnknownKeyTypeImplCopyWith<$Res> {
  __$$ConnectionError_UnknownKeyTypeImplCopyWithImpl(
      _$ConnectionError_UnknownKeyTypeImpl _value,
      $Res Function(_$ConnectionError_UnknownKeyTypeImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$ConnectionError_UnknownKeyTypeImpl
    extends ConnectionError_UnknownKeyType {
  const _$ConnectionError_UnknownKeyTypeImpl() : super._();

  @override
  String toString() {
    return 'ConnectionError.unknownKeyType()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ConnectionError_UnknownKeyTypeImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) network,
    required TResult Function() noSshKeysProvided,
    required TResult Function(String message) authentication,
    required TResult Function(
            String hostname, HostKeyType keyType, String hostKey)
        unknownHost,
    required TResult Function(String hostname) missingHostKey,
    required TResult Function() unknownKeyType,
    required TResult Function(String expected, String actual)
        missmatchRemoteKey,
    required TResult Function(String message) other,
  }) {
    return unknownKeyType();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? network,
    TResult? Function()? noSshKeysProvided,
    TResult? Function(String message)? authentication,
    TResult? Function(String hostname, HostKeyType keyType, String hostKey)?
        unknownHost,
    TResult? Function(String hostname)? missingHostKey,
    TResult? Function()? unknownKeyType,
    TResult? Function(String expected, String actual)? missmatchRemoteKey,
    TResult? Function(String message)? other,
  }) {
    return unknownKeyType?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? network,
    TResult Function()? noSshKeysProvided,
    TResult Function(String message)? authentication,
    TResult Function(String hostname, HostKeyType keyType, String hostKey)?
        unknownHost,
    TResult Function(String hostname)? missingHostKey,
    TResult Function()? unknownKeyType,
    TResult Function(String expected, String actual)? missmatchRemoteKey,
    TResult Function(String message)? other,
    required TResult orElse(),
  }) {
    if (unknownKeyType != null) {
      return unknownKeyType();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ConnectionError_Network value) network,
    required TResult Function(ConnectionError_NoSshKeysProvided value)
        noSshKeysProvided,
    required TResult Function(ConnectionError_Authentication value)
        authentication,
    required TResult Function(ConnectionError_UnknownHost value) unknownHost,
    required TResult Function(ConnectionError_MissingHostKey value)
        missingHostKey,
    required TResult Function(ConnectionError_UnknownKeyType value)
        unknownKeyType,
    required TResult Function(ConnectionError_MissmatchRemoteKey value)
        missmatchRemoteKey,
    required TResult Function(ConnectionError_Other value) other,
  }) {
    return unknownKeyType(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ConnectionError_Network value)? network,
    TResult? Function(ConnectionError_NoSshKeysProvided value)?
        noSshKeysProvided,
    TResult? Function(ConnectionError_Authentication value)? authentication,
    TResult? Function(ConnectionError_UnknownHost value)? unknownHost,
    TResult? Function(ConnectionError_MissingHostKey value)? missingHostKey,
    TResult? Function(ConnectionError_UnknownKeyType value)? unknownKeyType,
    TResult? Function(ConnectionError_MissmatchRemoteKey value)?
        missmatchRemoteKey,
    TResult? Function(ConnectionError_Other value)? other,
  }) {
    return unknownKeyType?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ConnectionError_Network value)? network,
    TResult Function(ConnectionError_NoSshKeysProvided value)?
        noSshKeysProvided,
    TResult Function(ConnectionError_Authentication value)? authentication,
    TResult Function(ConnectionError_UnknownHost value)? unknownHost,
    TResult Function(ConnectionError_MissingHostKey value)? missingHostKey,
    TResult Function(ConnectionError_UnknownKeyType value)? unknownKeyType,
    TResult Function(ConnectionError_MissmatchRemoteKey value)?
        missmatchRemoteKey,
    TResult Function(ConnectionError_Other value)? other,
    required TResult orElse(),
  }) {
    if (unknownKeyType != null) {
      return unknownKeyType(this);
    }
    return orElse();
  }
}

abstract class ConnectionError_UnknownKeyType extends ConnectionError {
  const factory ConnectionError_UnknownKeyType() =
      _$ConnectionError_UnknownKeyTypeImpl;
  const ConnectionError_UnknownKeyType._() : super._();
}

/// @nodoc
abstract class _$$ConnectionError_MissmatchRemoteKeyImplCopyWith<$Res> {
  factory _$$ConnectionError_MissmatchRemoteKeyImplCopyWith(
          _$ConnectionError_MissmatchRemoteKeyImpl value,
          $Res Function(_$ConnectionError_MissmatchRemoteKeyImpl) then) =
      __$$ConnectionError_MissmatchRemoteKeyImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String expected, String actual});
}

/// @nodoc
class __$$ConnectionError_MissmatchRemoteKeyImplCopyWithImpl<$Res>
    extends _$ConnectionErrorCopyWithImpl<$Res,
        _$ConnectionError_MissmatchRemoteKeyImpl>
    implements _$$ConnectionError_MissmatchRemoteKeyImplCopyWith<$Res> {
  __$$ConnectionError_MissmatchRemoteKeyImplCopyWithImpl(
      _$ConnectionError_MissmatchRemoteKeyImpl _value,
      $Res Function(_$ConnectionError_MissmatchRemoteKeyImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? expected = null,
    Object? actual = null,
  }) {
    return _then(_$ConnectionError_MissmatchRemoteKeyImpl(
      expected: null == expected
          ? _value.expected
          : expected // ignore: cast_nullable_to_non_nullable
              as String,
      actual: null == actual
          ? _value.actual
          : actual // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$ConnectionError_MissmatchRemoteKeyImpl
    extends ConnectionError_MissmatchRemoteKey {
  const _$ConnectionError_MissmatchRemoteKeyImpl(
      {required this.expected, required this.actual})
      : super._();

  @override
  final String expected;
  @override
  final String actual;

  @override
  String toString() {
    return 'ConnectionError.missmatchRemoteKey(expected: $expected, actual: $actual)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ConnectionError_MissmatchRemoteKeyImpl &&
            (identical(other.expected, expected) ||
                other.expected == expected) &&
            (identical(other.actual, actual) || other.actual == actual));
  }

  @override
  int get hashCode => Object.hash(runtimeType, expected, actual);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ConnectionError_MissmatchRemoteKeyImplCopyWith<
          _$ConnectionError_MissmatchRemoteKeyImpl>
      get copyWith => __$$ConnectionError_MissmatchRemoteKeyImplCopyWithImpl<
          _$ConnectionError_MissmatchRemoteKeyImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) network,
    required TResult Function() noSshKeysProvided,
    required TResult Function(String message) authentication,
    required TResult Function(
            String hostname, HostKeyType keyType, String hostKey)
        unknownHost,
    required TResult Function(String hostname) missingHostKey,
    required TResult Function() unknownKeyType,
    required TResult Function(String expected, String actual)
        missmatchRemoteKey,
    required TResult Function(String message) other,
  }) {
    return missmatchRemoteKey(expected, actual);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? network,
    TResult? Function()? noSshKeysProvided,
    TResult? Function(String message)? authentication,
    TResult? Function(String hostname, HostKeyType keyType, String hostKey)?
        unknownHost,
    TResult? Function(String hostname)? missingHostKey,
    TResult? Function()? unknownKeyType,
    TResult? Function(String expected, String actual)? missmatchRemoteKey,
    TResult? Function(String message)? other,
  }) {
    return missmatchRemoteKey?.call(expected, actual);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? network,
    TResult Function()? noSshKeysProvided,
    TResult Function(String message)? authentication,
    TResult Function(String hostname, HostKeyType keyType, String hostKey)?
        unknownHost,
    TResult Function(String hostname)? missingHostKey,
    TResult Function()? unknownKeyType,
    TResult Function(String expected, String actual)? missmatchRemoteKey,
    TResult Function(String message)? other,
    required TResult orElse(),
  }) {
    if (missmatchRemoteKey != null) {
      return missmatchRemoteKey(expected, actual);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ConnectionError_Network value) network,
    required TResult Function(ConnectionError_NoSshKeysProvided value)
        noSshKeysProvided,
    required TResult Function(ConnectionError_Authentication value)
        authentication,
    required TResult Function(ConnectionError_UnknownHost value) unknownHost,
    required TResult Function(ConnectionError_MissingHostKey value)
        missingHostKey,
    required TResult Function(ConnectionError_UnknownKeyType value)
        unknownKeyType,
    required TResult Function(ConnectionError_MissmatchRemoteKey value)
        missmatchRemoteKey,
    required TResult Function(ConnectionError_Other value) other,
  }) {
    return missmatchRemoteKey(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ConnectionError_Network value)? network,
    TResult? Function(ConnectionError_NoSshKeysProvided value)?
        noSshKeysProvided,
    TResult? Function(ConnectionError_Authentication value)? authentication,
    TResult? Function(ConnectionError_UnknownHost value)? unknownHost,
    TResult? Function(ConnectionError_MissingHostKey value)? missingHostKey,
    TResult? Function(ConnectionError_UnknownKeyType value)? unknownKeyType,
    TResult? Function(ConnectionError_MissmatchRemoteKey value)?
        missmatchRemoteKey,
    TResult? Function(ConnectionError_Other value)? other,
  }) {
    return missmatchRemoteKey?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ConnectionError_Network value)? network,
    TResult Function(ConnectionError_NoSshKeysProvided value)?
        noSshKeysProvided,
    TResult Function(ConnectionError_Authentication value)? authentication,
    TResult Function(ConnectionError_UnknownHost value)? unknownHost,
    TResult Function(ConnectionError_MissingHostKey value)? missingHostKey,
    TResult Function(ConnectionError_UnknownKeyType value)? unknownKeyType,
    TResult Function(ConnectionError_MissmatchRemoteKey value)?
        missmatchRemoteKey,
    TResult Function(ConnectionError_Other value)? other,
    required TResult orElse(),
  }) {
    if (missmatchRemoteKey != null) {
      return missmatchRemoteKey(this);
    }
    return orElse();
  }
}

abstract class ConnectionError_MissmatchRemoteKey extends ConnectionError {
  const factory ConnectionError_MissmatchRemoteKey(
      {required final String expected,
      required final String actual}) = _$ConnectionError_MissmatchRemoteKeyImpl;
  const ConnectionError_MissmatchRemoteKey._() : super._();

  String get expected;
  String get actual;
  @JsonKey(ignore: true)
  _$$ConnectionError_MissmatchRemoteKeyImplCopyWith<
          _$ConnectionError_MissmatchRemoteKeyImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ConnectionError_OtherImplCopyWith<$Res> {
  factory _$$ConnectionError_OtherImplCopyWith(
          _$ConnectionError_OtherImpl value,
          $Res Function(_$ConnectionError_OtherImpl) then) =
      __$$ConnectionError_OtherImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String message});
}

/// @nodoc
class __$$ConnectionError_OtherImplCopyWithImpl<$Res>
    extends _$ConnectionErrorCopyWithImpl<$Res, _$ConnectionError_OtherImpl>
    implements _$$ConnectionError_OtherImplCopyWith<$Res> {
  __$$ConnectionError_OtherImplCopyWithImpl(_$ConnectionError_OtherImpl _value,
      $Res Function(_$ConnectionError_OtherImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? message = null,
  }) {
    return _then(_$ConnectionError_OtherImpl(
      message: null == message
          ? _value.message
          : message // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$ConnectionError_OtherImpl extends ConnectionError_Other {
  const _$ConnectionError_OtherImpl({required this.message}) : super._();

  @override
  final String message;

  @override
  String toString() {
    return 'ConnectionError.other(message: $message)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ConnectionError_OtherImpl &&
            (identical(other.message, message) || other.message == message));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ConnectionError_OtherImplCopyWith<_$ConnectionError_OtherImpl>
      get copyWith => __$$ConnectionError_OtherImplCopyWithImpl<
          _$ConnectionError_OtherImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) network,
    required TResult Function() noSshKeysProvided,
    required TResult Function(String message) authentication,
    required TResult Function(
            String hostname, HostKeyType keyType, String hostKey)
        unknownHost,
    required TResult Function(String hostname) missingHostKey,
    required TResult Function() unknownKeyType,
    required TResult Function(String expected, String actual)
        missmatchRemoteKey,
    required TResult Function(String message) other,
  }) {
    return other(message);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? network,
    TResult? Function()? noSshKeysProvided,
    TResult? Function(String message)? authentication,
    TResult? Function(String hostname, HostKeyType keyType, String hostKey)?
        unknownHost,
    TResult? Function(String hostname)? missingHostKey,
    TResult? Function()? unknownKeyType,
    TResult? Function(String expected, String actual)? missmatchRemoteKey,
    TResult? Function(String message)? other,
  }) {
    return other?.call(message);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? network,
    TResult Function()? noSshKeysProvided,
    TResult Function(String message)? authentication,
    TResult Function(String hostname, HostKeyType keyType, String hostKey)?
        unknownHost,
    TResult Function(String hostname)? missingHostKey,
    TResult Function()? unknownKeyType,
    TResult Function(String expected, String actual)? missmatchRemoteKey,
    TResult Function(String message)? other,
    required TResult orElse(),
  }) {
    if (other != null) {
      return other(message);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ConnectionError_Network value) network,
    required TResult Function(ConnectionError_NoSshKeysProvided value)
        noSshKeysProvided,
    required TResult Function(ConnectionError_Authentication value)
        authentication,
    required TResult Function(ConnectionError_UnknownHost value) unknownHost,
    required TResult Function(ConnectionError_MissingHostKey value)
        missingHostKey,
    required TResult Function(ConnectionError_UnknownKeyType value)
        unknownKeyType,
    required TResult Function(ConnectionError_MissmatchRemoteKey value)
        missmatchRemoteKey,
    required TResult Function(ConnectionError_Other value) other,
  }) {
    return other(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ConnectionError_Network value)? network,
    TResult? Function(ConnectionError_NoSshKeysProvided value)?
        noSshKeysProvided,
    TResult? Function(ConnectionError_Authentication value)? authentication,
    TResult? Function(ConnectionError_UnknownHost value)? unknownHost,
    TResult? Function(ConnectionError_MissingHostKey value)? missingHostKey,
    TResult? Function(ConnectionError_UnknownKeyType value)? unknownKeyType,
    TResult? Function(ConnectionError_MissmatchRemoteKey value)?
        missmatchRemoteKey,
    TResult? Function(ConnectionError_Other value)? other,
  }) {
    return other?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ConnectionError_Network value)? network,
    TResult Function(ConnectionError_NoSshKeysProvided value)?
        noSshKeysProvided,
    TResult Function(ConnectionError_Authentication value)? authentication,
    TResult Function(ConnectionError_UnknownHost value)? unknownHost,
    TResult Function(ConnectionError_MissingHostKey value)? missingHostKey,
    TResult Function(ConnectionError_UnknownKeyType value)? unknownKeyType,
    TResult Function(ConnectionError_MissmatchRemoteKey value)?
        missmatchRemoteKey,
    TResult Function(ConnectionError_Other value)? other,
    required TResult orElse(),
  }) {
    if (other != null) {
      return other(this);
    }
    return orElse();
  }
}

abstract class ConnectionError_Other extends ConnectionError {
  const factory ConnectionError_Other({required final String message}) =
      _$ConnectionError_OtherImpl;
  const ConnectionError_Other._() : super._();

  String get message;
  @JsonKey(ignore: true)
  _$$ConnectionError_OtherImplCopyWith<_$ConnectionError_OtherImpl>
      get copyWith => throw _privateConstructorUsedError;
}
