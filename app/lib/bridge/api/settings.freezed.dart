// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'settings.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$RepositorySpecification {

 UuidValue get uuid; String get name; String get origin; String get author; String get email; String get branch; UuidValue? get sshKeyUuid; EncryptionKey? get encryption;
/// Create a copy of RepositorySpecification
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RepositorySpecificationCopyWith<RepositorySpecification> get copyWith => _$RepositorySpecificationCopyWithImpl<RepositorySpecification>(this as RepositorySpecification, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is RepositorySpecification&&(identical(other.uuid, uuid) || other.uuid == uuid)&&(identical(other.name, name) || other.name == name)&&(identical(other.origin, origin) || other.origin == origin)&&(identical(other.author, author) || other.author == author)&&(identical(other.email, email) || other.email == email)&&(identical(other.branch, branch) || other.branch == branch)&&(identical(other.sshKeyUuid, sshKeyUuid) || other.sshKeyUuid == sshKeyUuid)&&(identical(other.encryption, encryption) || other.encryption == encryption));
}


@override
int get hashCode => Object.hash(runtimeType,uuid,name,origin,author,email,branch,sshKeyUuid,encryption);

@override
String toString() {
  return 'RepositorySpecification(uuid: $uuid, name: $name, origin: $origin, author: $author, email: $email, branch: $branch, sshKeyUuid: $sshKeyUuid, encryption: $encryption)';
}


}

/// @nodoc
abstract mixin class $RepositorySpecificationCopyWith<$Res>  {
  factory $RepositorySpecificationCopyWith(RepositorySpecification value, $Res Function(RepositorySpecification) _then) = _$RepositorySpecificationCopyWithImpl;
@useResult
$Res call({
 UuidValue uuid, String name, String origin, String author, String email, String branch, UuidValue? sshKeyUuid, EncryptionKey? encryption
});




}
/// @nodoc
class _$RepositorySpecificationCopyWithImpl<$Res>
    implements $RepositorySpecificationCopyWith<$Res> {
  _$RepositorySpecificationCopyWithImpl(this._self, this._then);

  final RepositorySpecification _self;
  final $Res Function(RepositorySpecification) _then;

/// Create a copy of RepositorySpecification
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? uuid = null,Object? name = null,Object? origin = null,Object? author = null,Object? email = null,Object? branch = null,Object? sshKeyUuid = freezed,Object? encryption = freezed,}) {
  return _then(_self.copyWith(
uuid: null == uuid ? _self.uuid : uuid // ignore: cast_nullable_to_non_nullable
as UuidValue,name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,origin: null == origin ? _self.origin : origin // ignore: cast_nullable_to_non_nullable
as String,author: null == author ? _self.author : author // ignore: cast_nullable_to_non_nullable
as String,email: null == email ? _self.email : email // ignore: cast_nullable_to_non_nullable
as String,branch: null == branch ? _self.branch : branch // ignore: cast_nullable_to_non_nullable
as String,sshKeyUuid: freezed == sshKeyUuid ? _self.sshKeyUuid : sshKeyUuid // ignore: cast_nullable_to_non_nullable
as UuidValue?,encryption: freezed == encryption ? _self.encryption : encryption // ignore: cast_nullable_to_non_nullable
as EncryptionKey?,
  ));
}

}


/// Adds pattern-matching-related methods to [RepositorySpecification].
extension RepositorySpecificationPatterns on RepositorySpecification {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _RepositorySpecification value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _RepositorySpecification() when $default != null:
return $default(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _RepositorySpecification value)  $default,){
final _that = this;
switch (_that) {
case _RepositorySpecification():
return $default(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _RepositorySpecification value)?  $default,){
final _that = this;
switch (_that) {
case _RepositorySpecification() when $default != null:
return $default(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( UuidValue uuid,  String name,  String origin,  String author,  String email,  String branch,  UuidValue? sshKeyUuid,  EncryptionKey? encryption)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _RepositorySpecification() when $default != null:
return $default(_that.uuid,_that.name,_that.origin,_that.author,_that.email,_that.branch,_that.sshKeyUuid,_that.encryption);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( UuidValue uuid,  String name,  String origin,  String author,  String email,  String branch,  UuidValue? sshKeyUuid,  EncryptionKey? encryption)  $default,) {final _that = this;
switch (_that) {
case _RepositorySpecification():
return $default(_that.uuid,_that.name,_that.origin,_that.author,_that.email,_that.branch,_that.sshKeyUuid,_that.encryption);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( UuidValue uuid,  String name,  String origin,  String author,  String email,  String branch,  UuidValue? sshKeyUuid,  EncryptionKey? encryption)?  $default,) {final _that = this;
switch (_that) {
case _RepositorySpecification() when $default != null:
return $default(_that.uuid,_that.name,_that.origin,_that.author,_that.email,_that.branch,_that.sshKeyUuid,_that.encryption);case _:
  return null;

}
}

}

/// @nodoc


class _RepositorySpecification extends RepositorySpecification {
  const _RepositorySpecification({required this.uuid, required this.name, required this.origin, required this.author, required this.email, required this.branch, this.sshKeyUuid, this.encryption}): super._();
  

@override final  UuidValue uuid;
@override final  String name;
@override final  String origin;
@override final  String author;
@override final  String email;
@override final  String branch;
@override final  UuidValue? sshKeyUuid;
@override final  EncryptionKey? encryption;

/// Create a copy of RepositorySpecification
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$RepositorySpecificationCopyWith<_RepositorySpecification> get copyWith => __$RepositorySpecificationCopyWithImpl<_RepositorySpecification>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _RepositorySpecification&&(identical(other.uuid, uuid) || other.uuid == uuid)&&(identical(other.name, name) || other.name == name)&&(identical(other.origin, origin) || other.origin == origin)&&(identical(other.author, author) || other.author == author)&&(identical(other.email, email) || other.email == email)&&(identical(other.branch, branch) || other.branch == branch)&&(identical(other.sshKeyUuid, sshKeyUuid) || other.sshKeyUuid == sshKeyUuid)&&(identical(other.encryption, encryption) || other.encryption == encryption));
}


@override
int get hashCode => Object.hash(runtimeType,uuid,name,origin,author,email,branch,sshKeyUuid,encryption);

@override
String toString() {
  return 'RepositorySpecification(uuid: $uuid, name: $name, origin: $origin, author: $author, email: $email, branch: $branch, sshKeyUuid: $sshKeyUuid, encryption: $encryption)';
}


}

/// @nodoc
abstract mixin class _$RepositorySpecificationCopyWith<$Res> implements $RepositorySpecificationCopyWith<$Res> {
  factory _$RepositorySpecificationCopyWith(_RepositorySpecification value, $Res Function(_RepositorySpecification) _then) = __$RepositorySpecificationCopyWithImpl;
@override @useResult
$Res call({
 UuidValue uuid, String name, String origin, String author, String email, String branch, UuidValue? sshKeyUuid, EncryptionKey? encryption
});




}
/// @nodoc
class __$RepositorySpecificationCopyWithImpl<$Res>
    implements _$RepositorySpecificationCopyWith<$Res> {
  __$RepositorySpecificationCopyWithImpl(this._self, this._then);

  final _RepositorySpecification _self;
  final $Res Function(_RepositorySpecification) _then;

/// Create a copy of RepositorySpecification
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? uuid = null,Object? name = null,Object? origin = null,Object? author = null,Object? email = null,Object? branch = null,Object? sshKeyUuid = freezed,Object? encryption = freezed,}) {
  return _then(_RepositorySpecification(
uuid: null == uuid ? _self.uuid : uuid // ignore: cast_nullable_to_non_nullable
as UuidValue,name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,origin: null == origin ? _self.origin : origin // ignore: cast_nullable_to_non_nullable
as String,author: null == author ? _self.author : author // ignore: cast_nullable_to_non_nullable
as String,email: null == email ? _self.email : email // ignore: cast_nullable_to_non_nullable
as String,branch: null == branch ? _self.branch : branch // ignore: cast_nullable_to_non_nullable
as String,sshKeyUuid: freezed == sshKeyUuid ? _self.sshKeyUuid : sshKeyUuid // ignore: cast_nullable_to_non_nullable
as UuidValue?,encryption: freezed == encryption ? _self.encryption : encryption // ignore: cast_nullable_to_non_nullable
as EncryptionKey?,
  ));
}


}

/// @nodoc
mixin _$Settings {

 bool get darkMode; bool get periodicSync; List<Filter> get filters; FilterSelection? get selectedFilter; UuidValue? get currentRepository; List<RepositorySpecification> get repositories;
/// Create a copy of Settings
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$SettingsCopyWith<Settings> get copyWith => _$SettingsCopyWithImpl<Settings>(this as Settings, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Settings&&(identical(other.darkMode, darkMode) || other.darkMode == darkMode)&&(identical(other.periodicSync, periodicSync) || other.periodicSync == periodicSync)&&const DeepCollectionEquality().equals(other.filters, filters)&&(identical(other.selectedFilter, selectedFilter) || other.selectedFilter == selectedFilter)&&(identical(other.currentRepository, currentRepository) || other.currentRepository == currentRepository)&&const DeepCollectionEquality().equals(other.repositories, repositories));
}


@override
int get hashCode => Object.hash(runtimeType,darkMode,periodicSync,const DeepCollectionEquality().hash(filters),selectedFilter,currentRepository,const DeepCollectionEquality().hash(repositories));

@override
String toString() {
  return 'Settings(darkMode: $darkMode, periodicSync: $periodicSync, filters: $filters, selectedFilter: $selectedFilter, currentRepository: $currentRepository, repositories: $repositories)';
}


}

/// @nodoc
abstract mixin class $SettingsCopyWith<$Res>  {
  factory $SettingsCopyWith(Settings value, $Res Function(Settings) _then) = _$SettingsCopyWithImpl;
@useResult
$Res call({
 bool darkMode, bool periodicSync, List<Filter> filters, FilterSelection? selectedFilter, UuidValue? currentRepository, List<RepositorySpecification> repositories
});


$FilterSelectionCopyWith<$Res>? get selectedFilter;

}
/// @nodoc
class _$SettingsCopyWithImpl<$Res>
    implements $SettingsCopyWith<$Res> {
  _$SettingsCopyWithImpl(this._self, this._then);

  final Settings _self;
  final $Res Function(Settings) _then;

/// Create a copy of Settings
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? darkMode = null,Object? periodicSync = null,Object? filters = null,Object? selectedFilter = freezed,Object? currentRepository = freezed,Object? repositories = null,}) {
  return _then(_self.copyWith(
darkMode: null == darkMode ? _self.darkMode : darkMode // ignore: cast_nullable_to_non_nullable
as bool,periodicSync: null == periodicSync ? _self.periodicSync : periodicSync // ignore: cast_nullable_to_non_nullable
as bool,filters: null == filters ? _self.filters : filters // ignore: cast_nullable_to_non_nullable
as List<Filter>,selectedFilter: freezed == selectedFilter ? _self.selectedFilter : selectedFilter // ignore: cast_nullable_to_non_nullable
as FilterSelection?,currentRepository: freezed == currentRepository ? _self.currentRepository : currentRepository // ignore: cast_nullable_to_non_nullable
as UuidValue?,repositories: null == repositories ? _self.repositories : repositories // ignore: cast_nullable_to_non_nullable
as List<RepositorySpecification>,
  ));
}
/// Create a copy of Settings
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$FilterSelectionCopyWith<$Res>? get selectedFilter {
    if (_self.selectedFilter == null) {
    return null;
  }

  return $FilterSelectionCopyWith<$Res>(_self.selectedFilter!, (value) {
    return _then(_self.copyWith(selectedFilter: value));
  });
}
}


/// Adds pattern-matching-related methods to [Settings].
extension SettingsPatterns on Settings {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( _Settings value)?  raw,required TResult orElse(),}){
final _that = this;
switch (_that) {
case _Settings() when raw != null:
return raw(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( _Settings value)  raw,}){
final _that = this;
switch (_that) {
case _Settings():
return raw(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( _Settings value)?  raw,}){
final _that = this;
switch (_that) {
case _Settings() when raw != null:
return raw(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( bool darkMode,  bool periodicSync,  List<Filter> filters,  FilterSelection? selectedFilter,  UuidValue? currentRepository,  List<RepositorySpecification> repositories)?  raw,required TResult orElse(),}) {final _that = this;
switch (_that) {
case _Settings() when raw != null:
return raw(_that.darkMode,_that.periodicSync,_that.filters,_that.selectedFilter,_that.currentRepository,_that.repositories);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( bool darkMode,  bool periodicSync,  List<Filter> filters,  FilterSelection? selectedFilter,  UuidValue? currentRepository,  List<RepositorySpecification> repositories)  raw,}) {final _that = this;
switch (_that) {
case _Settings():
return raw(_that.darkMode,_that.periodicSync,_that.filters,_that.selectedFilter,_that.currentRepository,_that.repositories);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( bool darkMode,  bool periodicSync,  List<Filter> filters,  FilterSelection? selectedFilter,  UuidValue? currentRepository,  List<RepositorySpecification> repositories)?  raw,}) {final _that = this;
switch (_that) {
case _Settings() when raw != null:
return raw(_that.darkMode,_that.periodicSync,_that.filters,_that.selectedFilter,_that.currentRepository,_that.repositories);case _:
  return null;

}
}

}

/// @nodoc


class _Settings extends Settings {
  const _Settings({required this.darkMode, required this.periodicSync, required final  List<Filter> filters, this.selectedFilter, this.currentRepository, required final  List<RepositorySpecification> repositories}): _filters = filters,_repositories = repositories,super._();
  

@override final  bool darkMode;
@override final  bool periodicSync;
 final  List<Filter> _filters;
@override List<Filter> get filters {
  if (_filters is EqualUnmodifiableListView) return _filters;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_filters);
}

@override final  FilterSelection? selectedFilter;
@override final  UuidValue? currentRepository;
 final  List<RepositorySpecification> _repositories;
@override List<RepositorySpecification> get repositories {
  if (_repositories is EqualUnmodifiableListView) return _repositories;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_repositories);
}


/// Create a copy of Settings
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$SettingsCopyWith<_Settings> get copyWith => __$SettingsCopyWithImpl<_Settings>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _Settings&&(identical(other.darkMode, darkMode) || other.darkMode == darkMode)&&(identical(other.periodicSync, periodicSync) || other.periodicSync == periodicSync)&&const DeepCollectionEquality().equals(other._filters, _filters)&&(identical(other.selectedFilter, selectedFilter) || other.selectedFilter == selectedFilter)&&(identical(other.currentRepository, currentRepository) || other.currentRepository == currentRepository)&&const DeepCollectionEquality().equals(other._repositories, _repositories));
}


@override
int get hashCode => Object.hash(runtimeType,darkMode,periodicSync,const DeepCollectionEquality().hash(_filters),selectedFilter,currentRepository,const DeepCollectionEquality().hash(_repositories));

@override
String toString() {
  return 'Settings.raw(darkMode: $darkMode, periodicSync: $periodicSync, filters: $filters, selectedFilter: $selectedFilter, currentRepository: $currentRepository, repositories: $repositories)';
}


}

/// @nodoc
abstract mixin class _$SettingsCopyWith<$Res> implements $SettingsCopyWith<$Res> {
  factory _$SettingsCopyWith(_Settings value, $Res Function(_Settings) _then) = __$SettingsCopyWithImpl;
@override @useResult
$Res call({
 bool darkMode, bool periodicSync, List<Filter> filters, FilterSelection? selectedFilter, UuidValue? currentRepository, List<RepositorySpecification> repositories
});


@override $FilterSelectionCopyWith<$Res>? get selectedFilter;

}
/// @nodoc
class __$SettingsCopyWithImpl<$Res>
    implements _$SettingsCopyWith<$Res> {
  __$SettingsCopyWithImpl(this._self, this._then);

  final _Settings _self;
  final $Res Function(_Settings) _then;

/// Create a copy of Settings
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? darkMode = null,Object? periodicSync = null,Object? filters = null,Object? selectedFilter = freezed,Object? currentRepository = freezed,Object? repositories = null,}) {
  return _then(_Settings(
darkMode: null == darkMode ? _self.darkMode : darkMode // ignore: cast_nullable_to_non_nullable
as bool,periodicSync: null == periodicSync ? _self.periodicSync : periodicSync // ignore: cast_nullable_to_non_nullable
as bool,filters: null == filters ? _self._filters : filters // ignore: cast_nullable_to_non_nullable
as List<Filter>,selectedFilter: freezed == selectedFilter ? _self.selectedFilter : selectedFilter // ignore: cast_nullable_to_non_nullable
as FilterSelection?,currentRepository: freezed == currentRepository ? _self.currentRepository : currentRepository // ignore: cast_nullable_to_non_nullable
as UuidValue?,repositories: null == repositories ? _self._repositories : repositories // ignore: cast_nullable_to_non_nullable
as List<RepositorySpecification>,
  ));
}

/// Create a copy of Settings
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$FilterSelectionCopyWith<$Res>? get selectedFilter {
    if (_self.selectedFilter == null) {
    return null;
  }

  return $FilterSelectionCopyWith<$Res>(_self.selectedFilter!, (value) {
    return _then(_self.copyWith(selectedFilter: value));
  });
}
}

// dart format on
