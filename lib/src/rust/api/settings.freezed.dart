// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'settings.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$Repository {
  String get origin => throw _privateConstructorUsedError;
  String get author => throw _privateConstructorUsedError;
  String get email => throw _privateConstructorUsedError;
  String get branch => throw _privateConstructorUsedError;
  UuidValue? get sshKeyUuid => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $RepositoryCopyWith<Repository> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $RepositoryCopyWith<$Res> {
  factory $RepositoryCopyWith(
          Repository value, $Res Function(Repository) then) =
      _$RepositoryCopyWithImpl<$Res, Repository>;
  @useResult
  $Res call(
      {String origin,
      String author,
      String email,
      String branch,
      UuidValue? sshKeyUuid});
}

/// @nodoc
class _$RepositoryCopyWithImpl<$Res, $Val extends Repository>
    implements $RepositoryCopyWith<$Res> {
  _$RepositoryCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? origin = null,
    Object? author = null,
    Object? email = null,
    Object? branch = null,
    Object? sshKeyUuid = freezed,
  }) {
    return _then(_value.copyWith(
      origin: null == origin
          ? _value.origin
          : origin // ignore: cast_nullable_to_non_nullable
              as String,
      author: null == author
          ? _value.author
          : author // ignore: cast_nullable_to_non_nullable
              as String,
      email: null == email
          ? _value.email
          : email // ignore: cast_nullable_to_non_nullable
              as String,
      branch: null == branch
          ? _value.branch
          : branch // ignore: cast_nullable_to_non_nullable
              as String,
      sshKeyUuid: freezed == sshKeyUuid
          ? _value.sshKeyUuid
          : sshKeyUuid // ignore: cast_nullable_to_non_nullable
              as UuidValue?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$RepositoryImplCopyWith<$Res>
    implements $RepositoryCopyWith<$Res> {
  factory _$$RepositoryImplCopyWith(
          _$RepositoryImpl value, $Res Function(_$RepositoryImpl) then) =
      __$$RepositoryImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String origin,
      String author,
      String email,
      String branch,
      UuidValue? sshKeyUuid});
}

/// @nodoc
class __$$RepositoryImplCopyWithImpl<$Res>
    extends _$RepositoryCopyWithImpl<$Res, _$RepositoryImpl>
    implements _$$RepositoryImplCopyWith<$Res> {
  __$$RepositoryImplCopyWithImpl(
      _$RepositoryImpl _value, $Res Function(_$RepositoryImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? origin = null,
    Object? author = null,
    Object? email = null,
    Object? branch = null,
    Object? sshKeyUuid = freezed,
  }) {
    return _then(_$RepositoryImpl(
      origin: null == origin
          ? _value.origin
          : origin // ignore: cast_nullable_to_non_nullable
              as String,
      author: null == author
          ? _value.author
          : author // ignore: cast_nullable_to_non_nullable
              as String,
      email: null == email
          ? _value.email
          : email // ignore: cast_nullable_to_non_nullable
              as String,
      branch: null == branch
          ? _value.branch
          : branch // ignore: cast_nullable_to_non_nullable
              as String,
      sshKeyUuid: freezed == sshKeyUuid
          ? _value.sshKeyUuid
          : sshKeyUuid // ignore: cast_nullable_to_non_nullable
              as UuidValue?,
    ));
  }
}

/// @nodoc

class _$RepositoryImpl implements _Repository {
  const _$RepositoryImpl(
      {required this.origin,
      required this.author,
      required this.email,
      required this.branch,
      this.sshKeyUuid});

  @override
  final String origin;
  @override
  final String author;
  @override
  final String email;
  @override
  final String branch;
  @override
  final UuidValue? sshKeyUuid;

  @override
  String toString() {
    return 'Repository(origin: $origin, author: $author, email: $email, branch: $branch, sshKeyUuid: $sshKeyUuid)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$RepositoryImpl &&
            (identical(other.origin, origin) || other.origin == origin) &&
            (identical(other.author, author) || other.author == author) &&
            (identical(other.email, email) || other.email == email) &&
            (identical(other.branch, branch) || other.branch == branch) &&
            (identical(other.sshKeyUuid, sshKeyUuid) ||
                other.sshKeyUuid == sshKeyUuid));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, origin, author, email, branch, sshKeyUuid);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$RepositoryImplCopyWith<_$RepositoryImpl> get copyWith =>
      __$$RepositoryImplCopyWithImpl<_$RepositoryImpl>(this, _$identity);
}

abstract class _Repository implements Repository {
  const factory _Repository(
      {required final String origin,
      required final String author,
      required final String email,
      required final String branch,
      final UuidValue? sshKeyUuid}) = _$RepositoryImpl;

  @override
  String get origin;
  @override
  String get author;
  @override
  String get email;
  @override
  String get branch;
  @override
  UuidValue? get sshKeyUuid;
  @override
  @JsonKey(ignore: true)
  _$$RepositoryImplCopyWith<_$RepositoryImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$Settings {
  bool get darkMode => throw _privateConstructorUsedError;
  List<SshKey> get keys => throw _privateConstructorUsedError;
  KnownHosts get knownHosts => throw _privateConstructorUsedError;
  Repository get repository => throw _privateConstructorUsedError;
  bool get periodicSync => throw _privateConstructorUsedError;
  List<Filter> get filters => throw _privateConstructorUsedError;
  FilterSelection? get selectedFilter => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(
            bool darkMode,
            List<SshKey> keys,
            KnownHosts knownHosts,
            Repository repository,
            bool periodicSync,
            List<Filter> filters,
            FilterSelection? selectedFilter)
        raw,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(
            bool darkMode,
            List<SshKey> keys,
            KnownHosts knownHosts,
            Repository repository,
            bool periodicSync,
            List<Filter> filters,
            FilterSelection? selectedFilter)?
        raw,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(
            bool darkMode,
            List<SshKey> keys,
            KnownHosts knownHosts,
            Repository repository,
            bool periodicSync,
            List<Filter> filters,
            FilterSelection? selectedFilter)?
        raw,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(_Settings value) raw,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(_Settings value)? raw,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(_Settings value)? raw,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $SettingsCopyWith<Settings> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $SettingsCopyWith<$Res> {
  factory $SettingsCopyWith(Settings value, $Res Function(Settings) then) =
      _$SettingsCopyWithImpl<$Res, Settings>;
  @useResult
  $Res call(
      {bool darkMode,
      List<SshKey> keys,
      KnownHosts knownHosts,
      Repository repository,
      bool periodicSync,
      List<Filter> filters,
      FilterSelection? selectedFilter});

  $KnownHostsCopyWith<$Res> get knownHosts;
  $RepositoryCopyWith<$Res> get repository;
  $FilterSelectionCopyWith<$Res>? get selectedFilter;
}

/// @nodoc
class _$SettingsCopyWithImpl<$Res, $Val extends Settings>
    implements $SettingsCopyWith<$Res> {
  _$SettingsCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? darkMode = null,
    Object? keys = null,
    Object? knownHosts = null,
    Object? repository = null,
    Object? periodicSync = null,
    Object? filters = null,
    Object? selectedFilter = freezed,
  }) {
    return _then(_value.copyWith(
      darkMode: null == darkMode
          ? _value.darkMode
          : darkMode // ignore: cast_nullable_to_non_nullable
              as bool,
      keys: null == keys
          ? _value.keys
          : keys // ignore: cast_nullable_to_non_nullable
              as List<SshKey>,
      knownHosts: null == knownHosts
          ? _value.knownHosts
          : knownHosts // ignore: cast_nullable_to_non_nullable
              as KnownHosts,
      repository: null == repository
          ? _value.repository
          : repository // ignore: cast_nullable_to_non_nullable
              as Repository,
      periodicSync: null == periodicSync
          ? _value.periodicSync
          : periodicSync // ignore: cast_nullable_to_non_nullable
              as bool,
      filters: null == filters
          ? _value.filters
          : filters // ignore: cast_nullable_to_non_nullable
              as List<Filter>,
      selectedFilter: freezed == selectedFilter
          ? _value.selectedFilter
          : selectedFilter // ignore: cast_nullable_to_non_nullable
              as FilterSelection?,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $KnownHostsCopyWith<$Res> get knownHosts {
    return $KnownHostsCopyWith<$Res>(_value.knownHosts, (value) {
      return _then(_value.copyWith(knownHosts: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $RepositoryCopyWith<$Res> get repository {
    return $RepositoryCopyWith<$Res>(_value.repository, (value) {
      return _then(_value.copyWith(repository: value) as $Val);
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $FilterSelectionCopyWith<$Res>? get selectedFilter {
    if (_value.selectedFilter == null) {
      return null;
    }

    return $FilterSelectionCopyWith<$Res>(_value.selectedFilter!, (value) {
      return _then(_value.copyWith(selectedFilter: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$SettingsImplCopyWith<$Res>
    implements $SettingsCopyWith<$Res> {
  factory _$$SettingsImplCopyWith(
          _$SettingsImpl value, $Res Function(_$SettingsImpl) then) =
      __$$SettingsImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {bool darkMode,
      List<SshKey> keys,
      KnownHosts knownHosts,
      Repository repository,
      bool periodicSync,
      List<Filter> filters,
      FilterSelection? selectedFilter});

  @override
  $KnownHostsCopyWith<$Res> get knownHosts;
  @override
  $RepositoryCopyWith<$Res> get repository;
  @override
  $FilterSelectionCopyWith<$Res>? get selectedFilter;
}

/// @nodoc
class __$$SettingsImplCopyWithImpl<$Res>
    extends _$SettingsCopyWithImpl<$Res, _$SettingsImpl>
    implements _$$SettingsImplCopyWith<$Res> {
  __$$SettingsImplCopyWithImpl(
      _$SettingsImpl _value, $Res Function(_$SettingsImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? darkMode = null,
    Object? keys = null,
    Object? knownHosts = null,
    Object? repository = null,
    Object? periodicSync = null,
    Object? filters = null,
    Object? selectedFilter = freezed,
  }) {
    return _then(_$SettingsImpl(
      darkMode: null == darkMode
          ? _value.darkMode
          : darkMode // ignore: cast_nullable_to_non_nullable
              as bool,
      keys: null == keys
          ? _value._keys
          : keys // ignore: cast_nullable_to_non_nullable
              as List<SshKey>,
      knownHosts: null == knownHosts
          ? _value.knownHosts
          : knownHosts // ignore: cast_nullable_to_non_nullable
              as KnownHosts,
      repository: null == repository
          ? _value.repository
          : repository // ignore: cast_nullable_to_non_nullable
              as Repository,
      periodicSync: null == periodicSync
          ? _value.periodicSync
          : periodicSync // ignore: cast_nullable_to_non_nullable
              as bool,
      filters: null == filters
          ? _value._filters
          : filters // ignore: cast_nullable_to_non_nullable
              as List<Filter>,
      selectedFilter: freezed == selectedFilter
          ? _value.selectedFilter
          : selectedFilter // ignore: cast_nullable_to_non_nullable
              as FilterSelection?,
    ));
  }
}

/// @nodoc

class _$SettingsImpl extends _Settings {
  const _$SettingsImpl(
      {required this.darkMode,
      required final List<SshKey> keys,
      required this.knownHosts,
      required this.repository,
      required this.periodicSync,
      required final List<Filter> filters,
      this.selectedFilter})
      : _keys = keys,
        _filters = filters,
        super._();

  @override
  final bool darkMode;
  final List<SshKey> _keys;
  @override
  List<SshKey> get keys {
    if (_keys is EqualUnmodifiableListView) return _keys;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_keys);
  }

  @override
  final KnownHosts knownHosts;
  @override
  final Repository repository;
  @override
  final bool periodicSync;
  final List<Filter> _filters;
  @override
  List<Filter> get filters {
    if (_filters is EqualUnmodifiableListView) return _filters;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_filters);
  }

  @override
  final FilterSelection? selectedFilter;

  @override
  String toString() {
    return 'Settings.raw(darkMode: $darkMode, keys: $keys, knownHosts: $knownHosts, repository: $repository, periodicSync: $periodicSync, filters: $filters, selectedFilter: $selectedFilter)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SettingsImpl &&
            (identical(other.darkMode, darkMode) ||
                other.darkMode == darkMode) &&
            const DeepCollectionEquality().equals(other._keys, _keys) &&
            (identical(other.knownHosts, knownHosts) ||
                other.knownHosts == knownHosts) &&
            (identical(other.repository, repository) ||
                other.repository == repository) &&
            (identical(other.periodicSync, periodicSync) ||
                other.periodicSync == periodicSync) &&
            const DeepCollectionEquality().equals(other._filters, _filters) &&
            (identical(other.selectedFilter, selectedFilter) ||
                other.selectedFilter == selectedFilter));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType,
      darkMode,
      const DeepCollectionEquality().hash(_keys),
      knownHosts,
      repository,
      periodicSync,
      const DeepCollectionEquality().hash(_filters),
      selectedFilter);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SettingsImplCopyWith<_$SettingsImpl> get copyWith =>
      __$$SettingsImplCopyWithImpl<_$SettingsImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(
            bool darkMode,
            List<SshKey> keys,
            KnownHosts knownHosts,
            Repository repository,
            bool periodicSync,
            List<Filter> filters,
            FilterSelection? selectedFilter)
        raw,
  }) {
    return raw(darkMode, keys, knownHosts, repository, periodicSync, filters,
        selectedFilter);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(
            bool darkMode,
            List<SshKey> keys,
            KnownHosts knownHosts,
            Repository repository,
            bool periodicSync,
            List<Filter> filters,
            FilterSelection? selectedFilter)?
        raw,
  }) {
    return raw?.call(darkMode, keys, knownHosts, repository, periodicSync,
        filters, selectedFilter);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(
            bool darkMode,
            List<SshKey> keys,
            KnownHosts knownHosts,
            Repository repository,
            bool periodicSync,
            List<Filter> filters,
            FilterSelection? selectedFilter)?
        raw,
    required TResult orElse(),
  }) {
    if (raw != null) {
      return raw(darkMode, keys, knownHosts, repository, periodicSync, filters,
          selectedFilter);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(_Settings value) raw,
  }) {
    return raw(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(_Settings value)? raw,
  }) {
    return raw?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(_Settings value)? raw,
    required TResult orElse(),
  }) {
    if (raw != null) {
      return raw(this);
    }
    return orElse();
  }
}

abstract class _Settings extends Settings {
  const factory _Settings(
      {required final bool darkMode,
      required final List<SshKey> keys,
      required final KnownHosts knownHosts,
      required final Repository repository,
      required final bool periodicSync,
      required final List<Filter> filters,
      final FilterSelection? selectedFilter}) = _$SettingsImpl;
  const _Settings._() : super._();

  @override
  bool get darkMode;
  @override
  List<SshKey> get keys;
  @override
  KnownHosts get knownHosts;
  @override
  Repository get repository;
  @override
  bool get periodicSync;
  @override
  List<Filter> get filters;
  @override
  FilterSelection? get selectedFilter;
  @override
  @JsonKey(ignore: true)
  _$$SettingsImplCopyWith<_$SettingsImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$SshKey {
  UuidValue get uuid => throw _privateConstructorUsedError;
  String get public => throw _privateConstructorUsedError;
  String get private => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $SshKeyCopyWith<SshKey> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $SshKeyCopyWith<$Res> {
  factory $SshKeyCopyWith(SshKey value, $Res Function(SshKey) then) =
      _$SshKeyCopyWithImpl<$Res, SshKey>;
  @useResult
  $Res call({UuidValue uuid, String public, String private});
}

/// @nodoc
class _$SshKeyCopyWithImpl<$Res, $Val extends SshKey>
    implements $SshKeyCopyWith<$Res> {
  _$SshKeyCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? uuid = null,
    Object? public = null,
    Object? private = null,
  }) {
    return _then(_value.copyWith(
      uuid: null == uuid
          ? _value.uuid
          : uuid // ignore: cast_nullable_to_non_nullable
              as UuidValue,
      public: null == public
          ? _value.public
          : public // ignore: cast_nullable_to_non_nullable
              as String,
      private: null == private
          ? _value.private
          : private // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$SshKeyImplCopyWith<$Res> implements $SshKeyCopyWith<$Res> {
  factory _$$SshKeyImplCopyWith(
          _$SshKeyImpl value, $Res Function(_$SshKeyImpl) then) =
      __$$SshKeyImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({UuidValue uuid, String public, String private});
}

/// @nodoc
class __$$SshKeyImplCopyWithImpl<$Res>
    extends _$SshKeyCopyWithImpl<$Res, _$SshKeyImpl>
    implements _$$SshKeyImplCopyWith<$Res> {
  __$$SshKeyImplCopyWithImpl(
      _$SshKeyImpl _value, $Res Function(_$SshKeyImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? uuid = null,
    Object? public = null,
    Object? private = null,
  }) {
    return _then(_$SshKeyImpl(
      uuid: null == uuid
          ? _value.uuid
          : uuid // ignore: cast_nullable_to_non_nullable
              as UuidValue,
      public: null == public
          ? _value.public
          : public // ignore: cast_nullable_to_non_nullable
              as String,
      private: null == private
          ? _value.private
          : private // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$SshKeyImpl extends _SshKey {
  const _$SshKeyImpl(
      {required this.uuid, required this.public, required this.private})
      : super._();

  @override
  final UuidValue uuid;
  @override
  final String public;
  @override
  final String private;

  @override
  String toString() {
    return 'SshKey(uuid: $uuid, public: $public, private: $private)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SshKeyImpl &&
            (identical(other.uuid, uuid) || other.uuid == uuid) &&
            (identical(other.public, public) || other.public == public) &&
            (identical(other.private, private) || other.private == private));
  }

  @override
  int get hashCode => Object.hash(runtimeType, uuid, public, private);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SshKeyImplCopyWith<_$SshKeyImpl> get copyWith =>
      __$$SshKeyImplCopyWithImpl<_$SshKeyImpl>(this, _$identity);
}

abstract class _SshKey extends SshKey {
  const factory _SshKey(
      {required final UuidValue uuid,
      required final String public,
      required final String private}) = _$SshKeyImpl;
  const _SshKey._() : super._();

  @override
  UuidValue get uuid;
  @override
  String get public;
  @override
  String get private;
  @override
  @JsonKey(ignore: true)
  _$$SshKeyImplCopyWith<_$SshKeyImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
