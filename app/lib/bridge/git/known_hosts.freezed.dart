// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'known_hosts.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$KnownHosts {
  List<Host> get hosts => throw _privateConstructorUsedError;

  /// Create a copy of KnownHosts
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $KnownHostsCopyWith<KnownHosts> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $KnownHostsCopyWith<$Res> {
  factory $KnownHostsCopyWith(
          KnownHosts value, $Res Function(KnownHosts) then) =
      _$KnownHostsCopyWithImpl<$Res, KnownHosts>;
  @useResult
  $Res call({List<Host> hosts});
}

/// @nodoc
class _$KnownHostsCopyWithImpl<$Res, $Val extends KnownHosts>
    implements $KnownHostsCopyWith<$Res> {
  _$KnownHostsCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of KnownHosts
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? hosts = null,
  }) {
    return _then(_value.copyWith(
      hosts: null == hosts
          ? _value.hosts
          : hosts // ignore: cast_nullable_to_non_nullable
              as List<Host>,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$KnownHostsImplCopyWith<$Res>
    implements $KnownHostsCopyWith<$Res> {
  factory _$$KnownHostsImplCopyWith(
          _$KnownHostsImpl value, $Res Function(_$KnownHostsImpl) then) =
      __$$KnownHostsImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({List<Host> hosts});
}

/// @nodoc
class __$$KnownHostsImplCopyWithImpl<$Res>
    extends _$KnownHostsCopyWithImpl<$Res, _$KnownHostsImpl>
    implements _$$KnownHostsImplCopyWith<$Res> {
  __$$KnownHostsImplCopyWithImpl(
      _$KnownHostsImpl _value, $Res Function(_$KnownHostsImpl) _then)
      : super(_value, _then);

  /// Create a copy of KnownHosts
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? hosts = null,
  }) {
    return _then(_$KnownHostsImpl(
      hosts: null == hosts
          ? _value._hosts
          : hosts // ignore: cast_nullable_to_non_nullable
              as List<Host>,
    ));
  }
}

/// @nodoc

class _$KnownHostsImpl implements _KnownHosts {
  const _$KnownHostsImpl({required final List<Host> hosts}) : _hosts = hosts;

  final List<Host> _hosts;
  @override
  List<Host> get hosts {
    if (_hosts is EqualUnmodifiableListView) return _hosts;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_hosts);
  }

  @override
  String toString() {
    return 'KnownHosts(hosts: $hosts)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KnownHostsImpl &&
            const DeepCollectionEquality().equals(other._hosts, _hosts));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(_hosts));

  /// Create a copy of KnownHosts
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$KnownHostsImplCopyWith<_$KnownHostsImpl> get copyWith =>
      __$$KnownHostsImplCopyWithImpl<_$KnownHostsImpl>(this, _$identity);
}

abstract class _KnownHosts implements KnownHosts {
  const factory _KnownHosts({required final List<Host> hosts}) =
      _$KnownHostsImpl;

  @override
  List<Host> get hosts;

  /// Create a copy of KnownHosts
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$KnownHostsImplCopyWith<_$KnownHostsImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
