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
mixin _$RepositorySpecification {
  UuidValue get uuid => throw _privateConstructorUsedError;
  String get name => throw _privateConstructorUsedError;
  String get origin => throw _privateConstructorUsedError;
  String get author => throw _privateConstructorUsedError;
  String get email => throw _privateConstructorUsedError;
  String get branch => throw _privateConstructorUsedError;
  UuidValue? get sshKeyUuid => throw _privateConstructorUsedError;
  EncryptionKey? get encryption => throw _privateConstructorUsedError;

  /// Create a copy of RepositorySpecification
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $RepositorySpecificationCopyWith<RepositorySpecification> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $RepositorySpecificationCopyWith<$Res> {
  factory $RepositorySpecificationCopyWith(RepositorySpecification value,
          $Res Function(RepositorySpecification) then) =
      _$RepositorySpecificationCopyWithImpl<$Res, RepositorySpecification>;
  @useResult
  $Res call(
      {UuidValue uuid,
      String name,
      String origin,
      String author,
      String email,
      String branch,
      UuidValue? sshKeyUuid,
      EncryptionKey? encryption});
}

/// @nodoc
class _$RepositorySpecificationCopyWithImpl<$Res,
        $Val extends RepositorySpecification>
    implements $RepositorySpecificationCopyWith<$Res> {
  _$RepositorySpecificationCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of RepositorySpecification
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? uuid = null,
    Object? name = null,
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
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
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
abstract class _$$RepositorySpecificationImplCopyWith<$Res>
    implements $RepositorySpecificationCopyWith<$Res> {
  factory _$$RepositorySpecificationImplCopyWith(
          _$RepositorySpecificationImpl value,
          $Res Function(_$RepositorySpecificationImpl) then) =
      __$$RepositorySpecificationImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {UuidValue uuid,
      String name,
      String origin,
      String author,
      String email,
      String branch,
      UuidValue? sshKeyUuid,
      EncryptionKey? encryption});
}

/// @nodoc
class __$$RepositorySpecificationImplCopyWithImpl<$Res>
    extends _$RepositorySpecificationCopyWithImpl<$Res,
        _$RepositorySpecificationImpl>
    implements _$$RepositorySpecificationImplCopyWith<$Res> {
  __$$RepositorySpecificationImplCopyWithImpl(
      _$RepositorySpecificationImpl _value,
      $Res Function(_$RepositorySpecificationImpl) _then)
      : super(_value, _then);

  /// Create a copy of RepositorySpecification
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? uuid = null,
    Object? name = null,
    Object? origin = null,
    Object? author = null,
    Object? email = null,
    Object? branch = null,
    Object? sshKeyUuid = freezed,
    Object? encryption = freezed,
  }) {
    return _then(_$RepositorySpecificationImpl(
      uuid: null == uuid
          ? _value.uuid
          : uuid // ignore: cast_nullable_to_non_nullable
              as UuidValue,
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
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

class _$RepositorySpecificationImpl extends _RepositorySpecification {
  const _$RepositorySpecificationImpl(
      {required this.uuid,
      required this.name,
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
  final String name;
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
    return 'RepositorySpecification(uuid: $uuid, name: $name, origin: $origin, author: $author, email: $email, branch: $branch, sshKeyUuid: $sshKeyUuid, encryption: $encryption)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$RepositorySpecificationImpl &&
            (identical(other.uuid, uuid) || other.uuid == uuid) &&
            (identical(other.name, name) || other.name == name) &&
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
  int get hashCode => Object.hash(runtimeType, uuid, name, origin, author,
      email, branch, sshKeyUuid, encryption);

  /// Create a copy of RepositorySpecification
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$RepositorySpecificationImplCopyWith<_$RepositorySpecificationImpl>
      get copyWith => __$$RepositorySpecificationImplCopyWithImpl<
          _$RepositorySpecificationImpl>(this, _$identity);
}

abstract class _RepositorySpecification extends RepositorySpecification {
  const factory _RepositorySpecification(
      {required final UuidValue uuid,
      required final String name,
      required final String origin,
      required final String author,
      required final String email,
      required final String branch,
      final UuidValue? sshKeyUuid,
      final EncryptionKey? encryption}) = _$RepositorySpecificationImpl;
  const _RepositorySpecification._() : super._();

  @override
  UuidValue get uuid;
  @override
  String get name;
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

  /// Create a copy of RepositorySpecification
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$RepositorySpecificationImplCopyWith<_$RepositorySpecificationImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$Settings {
  bool get darkMode => throw _privateConstructorUsedError;
  bool get periodicSync => throw _privateConstructorUsedError;
  List<Filter> get filters => throw _privateConstructorUsedError;
  FilterSelection? get selectedFilter => throw _privateConstructorUsedError;
  UuidValue? get currentRepository => throw _privateConstructorUsedError;
  List<RepositorySpecification> get repositories =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(
            bool darkMode,
            bool periodicSync,
            List<Filter> filters,
            FilterSelection? selectedFilter,
            UuidValue? currentRepository,
            List<RepositorySpecification> repositories)
        raw,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(
            bool darkMode,
            bool periodicSync,
            List<Filter> filters,
            FilterSelection? selectedFilter,
            UuidValue? currentRepository,
            List<RepositorySpecification> repositories)?
        raw,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(
            bool darkMode,
            bool periodicSync,
            List<Filter> filters,
            FilterSelection? selectedFilter,
            UuidValue? currentRepository,
            List<RepositorySpecification> repositories)?
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
      bool periodicSync,
      List<Filter> filters,
      FilterSelection? selectedFilter,
      UuidValue? currentRepository,
      List<RepositorySpecification> repositories});

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
    Object? periodicSync = null,
    Object? filters = null,
    Object? selectedFilter = freezed,
    Object? currentRepository = freezed,
    Object? repositories = null,
  }) {
    return _then(_value.copyWith(
      darkMode: null == darkMode
          ? _value.darkMode
          : darkMode // ignore: cast_nullable_to_non_nullable
              as bool,
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
      currentRepository: freezed == currentRepository
          ? _value.currentRepository
          : currentRepository // ignore: cast_nullable_to_non_nullable
              as UuidValue?,
      repositories: null == repositories
          ? _value.repositories
          : repositories // ignore: cast_nullable_to_non_nullable
              as List<RepositorySpecification>,
    ) as $Val);
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
      bool periodicSync,
      List<Filter> filters,
      FilterSelection? selectedFilter,
      UuidValue? currentRepository,
      List<RepositorySpecification> repositories});

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
    Object? periodicSync = null,
    Object? filters = null,
    Object? selectedFilter = freezed,
    Object? currentRepository = freezed,
    Object? repositories = null,
  }) {
    return _then(_$SettingsImpl(
      darkMode: null == darkMode
          ? _value.darkMode
          : darkMode // ignore: cast_nullable_to_non_nullable
              as bool,
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
      currentRepository: freezed == currentRepository
          ? _value.currentRepository
          : currentRepository // ignore: cast_nullable_to_non_nullable
              as UuidValue?,
      repositories: null == repositories
          ? _value._repositories
          : repositories // ignore: cast_nullable_to_non_nullable
              as List<RepositorySpecification>,
    ));
  }
}

/// @nodoc

class _$SettingsImpl extends _Settings {
  const _$SettingsImpl(
      {required this.darkMode,
      required this.periodicSync,
      required final List<Filter> filters,
      this.selectedFilter,
      this.currentRepository,
      required final List<RepositorySpecification> repositories})
      : _filters = filters,
        _repositories = repositories,
        super._();

  @override
  final bool darkMode;
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
  final UuidValue? currentRepository;
  final List<RepositorySpecification> _repositories;
  @override
  List<RepositorySpecification> get repositories {
    if (_repositories is EqualUnmodifiableListView) return _repositories;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_repositories);
  }

  @override
  String toString() {
    return 'Settings.raw(darkMode: $darkMode, periodicSync: $periodicSync, filters: $filters, selectedFilter: $selectedFilter, currentRepository: $currentRepository, repositories: $repositories)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SettingsImpl &&
            (identical(other.darkMode, darkMode) ||
                other.darkMode == darkMode) &&
            (identical(other.periodicSync, periodicSync) ||
                other.periodicSync == periodicSync) &&
            const DeepCollectionEquality().equals(other._filters, _filters) &&
            (identical(other.selectedFilter, selectedFilter) ||
                other.selectedFilter == selectedFilter) &&
            (identical(other.currentRepository, currentRepository) ||
                other.currentRepository == currentRepository) &&
            const DeepCollectionEquality()
                .equals(other._repositories, _repositories));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType,
      darkMode,
      periodicSync,
      const DeepCollectionEquality().hash(_filters),
      selectedFilter,
      currentRepository,
      const DeepCollectionEquality().hash(_repositories));

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
            bool periodicSync,
            List<Filter> filters,
            FilterSelection? selectedFilter,
            UuidValue? currentRepository,
            List<RepositorySpecification> repositories)
        raw,
  }) {
    return raw(darkMode, periodicSync, filters, selectedFilter,
        currentRepository, repositories);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(
            bool darkMode,
            bool periodicSync,
            List<Filter> filters,
            FilterSelection? selectedFilter,
            UuidValue? currentRepository,
            List<RepositorySpecification> repositories)?
        raw,
  }) {
    return raw?.call(darkMode, periodicSync, filters, selectedFilter,
        currentRepository, repositories);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(
            bool darkMode,
            bool periodicSync,
            List<Filter> filters,
            FilterSelection? selectedFilter,
            UuidValue? currentRepository,
            List<RepositorySpecification> repositories)?
        raw,
    required TResult orElse(),
  }) {
    if (raw != null) {
      return raw(darkMode, periodicSync, filters, selectedFilter,
          currentRepository, repositories);
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
          required final bool periodicSync,
          required final List<Filter> filters,
          final FilterSelection? selectedFilter,
          final UuidValue? currentRepository,
          required final List<RepositorySpecification> repositories}) =
      _$SettingsImpl;
  const _Settings._() : super._();

  @override
  bool get darkMode;
  @override
  bool get periodicSync;
  @override
  List<Filter> get filters;
  @override
  FilterSelection? get selectedFilter;
  @override
  UuidValue? get currentRepository;
  @override
  List<RepositorySpecification> get repositories;

  /// Create a copy of Settings
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$SettingsImplCopyWith<_$SettingsImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
