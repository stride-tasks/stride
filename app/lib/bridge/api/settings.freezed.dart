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
  UuidValue get uuid => throw _privateConstructorUsedError;
  String get origin => throw _privateConstructorUsedError;
  String get author => throw _privateConstructorUsedError;
  String get email => throw _privateConstructorUsedError;
  String get branch => throw _privateConstructorUsedError;
  UuidValue? get sshKeyUuid => throw _privateConstructorUsedError;
  EncryptionKey? get encryption => throw _privateConstructorUsedError;

  /// Create a copy of Repository
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
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
      {UuidValue uuid,
      String origin,
      String author,
      String email,
      String branch,
      UuidValue? sshKeyUuid,
      EncryptionKey? encryption});
}

/// @nodoc
class _$RepositoryCopyWithImpl<$Res, $Val extends Repository>
    implements $RepositoryCopyWith<$Res> {
  _$RepositoryCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of Repository
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? uuid = null,
    Object? origin = null,
    Object? author = null,
    Object? email = null,
    Object? branch = null,
    Object? sshKeyUuid = freezed,
    Object? encryption = freezed,
  }) {
    return _then(_value.copyWith(
      uuid: null == uuid
          ? _value.uuid
          : uuid // ignore: cast_nullable_to_non_nullable
              as UuidValue,
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
      encryption: freezed == encryption
          ? _value.encryption
          : encryption // ignore: cast_nullable_to_non_nullable
              as EncryptionKey?,
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
      {UuidValue uuid,
      String origin,
      String author,
      String email,
      String branch,
      UuidValue? sshKeyUuid,
      EncryptionKey? encryption});
}

/// @nodoc
class __$$RepositoryImplCopyWithImpl<$Res>
    extends _$RepositoryCopyWithImpl<$Res, _$RepositoryImpl>
    implements _$$RepositoryImplCopyWith<$Res> {
  __$$RepositoryImplCopyWithImpl(
      _$RepositoryImpl _value, $Res Function(_$RepositoryImpl) _then)
      : super(_value, _then);

  /// Create a copy of Repository
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? uuid = null,
    Object? origin = null,
    Object? author = null,
    Object? email = null,
    Object? branch = null,
    Object? sshKeyUuid = freezed,
    Object? encryption = freezed,
  }) {
    return _then(_$RepositoryImpl(
      uuid: null == uuid
          ? _value.uuid
          : uuid // ignore: cast_nullable_to_non_nullable
              as UuidValue,
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
      encryption: freezed == encryption
          ? _value.encryption
          : encryption // ignore: cast_nullable_to_non_nullable
              as EncryptionKey?,
    ));
  }
}

/// @nodoc

class _$RepositoryImpl extends _Repository {
  const _$RepositoryImpl(
      {required this.uuid,
      required this.origin,
      required this.author,
      required this.email,
      required this.branch,
      this.sshKeyUuid,
      this.encryption})
      : super._();

  @override
  final UuidValue uuid;
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
  final EncryptionKey? encryption;

  @override
  String toString() {
    return 'Repository(uuid: $uuid, origin: $origin, author: $author, email: $email, branch: $branch, sshKeyUuid: $sshKeyUuid, encryption: $encryption)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$RepositoryImpl &&
            (identical(other.uuid, uuid) || other.uuid == uuid) &&
            (identical(other.origin, origin) || other.origin == origin) &&
            (identical(other.author, author) || other.author == author) &&
            (identical(other.email, email) || other.email == email) &&
            (identical(other.branch, branch) || other.branch == branch) &&
            (identical(other.sshKeyUuid, sshKeyUuid) ||
                other.sshKeyUuid == sshKeyUuid) &&
            (identical(other.encryption, encryption) ||
                other.encryption == encryption));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, uuid, origin, author, email, branch, sshKeyUuid, encryption);

  /// Create a copy of Repository
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$RepositoryImplCopyWith<_$RepositoryImpl> get copyWith =>
      __$$RepositoryImplCopyWithImpl<_$RepositoryImpl>(this, _$identity);
}

abstract class _Repository extends Repository {
  const factory _Repository(
      {required final UuidValue uuid,
      required final String origin,
      required final String author,
      required final String email,
      required final String branch,
      final UuidValue? sshKeyUuid,
      final EncryptionKey? encryption}) = _$RepositoryImpl;
  const _Repository._() : super._();

  @override
  UuidValue get uuid;
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
  EncryptionKey? get encryption;

  /// Create a copy of Repository
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$RepositoryImplCopyWith<_$RepositoryImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$Settings {
  bool get darkMode => throw _privateConstructorUsedError;
  Repository get repository => throw _privateConstructorUsedError;
  bool get periodicSync => throw _privateConstructorUsedError;
  List<Filter> get filters => throw _privateConstructorUsedError;
  FilterSelection? get selectedFilter => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(
            bool darkMode,
            Repository repository,
            bool periodicSync,
            List<Filter> filters,
            FilterSelection? selectedFilter)
        raw,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(bool darkMode, Repository repository, bool periodicSync,
            List<Filter> filters, FilterSelection? selectedFilter)?
        raw,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(bool darkMode, Repository repository, bool periodicSync,
            List<Filter> filters, FilterSelection? selectedFilter)?
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

  /// Create a copy of Settings
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
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
      Repository repository,
      bool periodicSync,
      List<Filter> filters,
      FilterSelection? selectedFilter});

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

  /// Create a copy of Settings
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? darkMode = null,
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

  /// Create a copy of Settings
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $RepositoryCopyWith<$Res> get repository {
    return $RepositoryCopyWith<$Res>(_value.repository, (value) {
      return _then(_value.copyWith(repository: value) as $Val);
    });
  }

  /// Create a copy of Settings
  /// with the given fields replaced by the non-null parameter values.
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
      Repository repository,
      bool periodicSync,
      List<Filter> filters,
      FilterSelection? selectedFilter});

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

  /// Create a copy of Settings
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? darkMode = null,
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
      required this.repository,
      required this.periodicSync,
      required final List<Filter> filters,
      this.selectedFilter})
      : _filters = filters,
        super._();

  @override
  final bool darkMode;
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
    return 'Settings.raw(darkMode: $darkMode, repository: $repository, periodicSync: $periodicSync, filters: $filters, selectedFilter: $selectedFilter)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SettingsImpl &&
            (identical(other.darkMode, darkMode) ||
                other.darkMode == darkMode) &&
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
      repository,
      periodicSync,
      const DeepCollectionEquality().hash(_filters),
      selectedFilter);

  /// Create a copy of Settings
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$SettingsImplCopyWith<_$SettingsImpl> get copyWith =>
      __$$SettingsImplCopyWithImpl<_$SettingsImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(
            bool darkMode,
            Repository repository,
            bool periodicSync,
            List<Filter> filters,
            FilterSelection? selectedFilter)
        raw,
  }) {
    return raw(darkMode, repository, periodicSync, filters, selectedFilter);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(bool darkMode, Repository repository, bool periodicSync,
            List<Filter> filters, FilterSelection? selectedFilter)?
        raw,
  }) {
    return raw?.call(
        darkMode, repository, periodicSync, filters, selectedFilter);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(bool darkMode, Repository repository, bool periodicSync,
            List<Filter> filters, FilterSelection? selectedFilter)?
        raw,
    required TResult orElse(),
  }) {
    if (raw != null) {
      return raw(darkMode, repository, periodicSync, filters, selectedFilter);
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
      required final Repository repository,
      required final bool periodicSync,
      required final List<Filter> filters,
      final FilterSelection? selectedFilter}) = _$SettingsImpl;
  const _Settings._() : super._();

  @override
  bool get darkMode;
  @override
  Repository get repository;
  @override
  bool get periodicSync;
  @override
  List<Filter> get filters;
  @override
  FilterSelection? get selectedFilter;

  /// Create a copy of Settings
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$SettingsImplCopyWith<_$SettingsImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
