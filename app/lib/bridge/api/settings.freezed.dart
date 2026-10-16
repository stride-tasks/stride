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
mixin _$Settings {

 bool get darkMode; bool get periodicSync; UuidValue? get currentRepository;
/// Create a copy of Settings
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$SettingsCopyWith<Settings> get copyWith => _$SettingsCopyWithImpl<Settings>(this as Settings, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Settings&&(identical(other.darkMode, darkMode) || other.darkMode == darkMode)&&(identical(other.periodicSync, periodicSync) || other.periodicSync == periodicSync)&&(identical(other.currentRepository, currentRepository) || other.currentRepository == currentRepository));
}


@override
int get hashCode => Object.hash(runtimeType,darkMode,periodicSync,currentRepository);

@override
String toString() {
  return 'Settings(darkMode: $darkMode, periodicSync: $periodicSync, currentRepository: $currentRepository)';
}


}

/// @nodoc
abstract mixin class $SettingsCopyWith<$Res>  {
  factory $SettingsCopyWith(Settings value, $Res Function(Settings) _then) = _$SettingsCopyWithImpl;
@useResult
$Res call({
 bool darkMode, bool periodicSync, UuidValue? currentRepository
});




}
/// @nodoc
class _$SettingsCopyWithImpl<$Res>
    implements $SettingsCopyWith<$Res> {
  _$SettingsCopyWithImpl(this._self, this._then);

  final Settings _self;
  final $Res Function(Settings) _then;

/// Create a copy of Settings
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? darkMode = null,Object? periodicSync = null,Object? currentRepository = freezed,}) {
  return _then(_self.copyWith(
darkMode: null == darkMode ? _self.darkMode : darkMode // ignore: cast_nullable_to_non_nullable
as bool,periodicSync: null == periodicSync ? _self.periodicSync : periodicSync // ignore: cast_nullable_to_non_nullable
as bool,currentRepository: freezed == currentRepository ? _self.currentRepository : currentRepository // ignore: cast_nullable_to_non_nullable
as UuidValue?,
  ));
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( bool darkMode,  bool periodicSync,  UuidValue? currentRepository)?  raw,required TResult orElse(),}) {final _that = this;
switch (_that) {
case _Settings() when raw != null:
return raw(_that.darkMode,_that.periodicSync,_that.currentRepository);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( bool darkMode,  bool periodicSync,  UuidValue? currentRepository)  raw,}) {final _that = this;
switch (_that) {
case _Settings():
return raw(_that.darkMode,_that.periodicSync,_that.currentRepository);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( bool darkMode,  bool periodicSync,  UuidValue? currentRepository)?  raw,}) {final _that = this;
switch (_that) {
case _Settings() when raw != null:
return raw(_that.darkMode,_that.periodicSync,_that.currentRepository);case _:
  return null;

}
}

}

/// @nodoc


class _Settings extends Settings {
  const _Settings({required this.darkMode, required this.periodicSync, this.currentRepository}): super._();
  

@override final  bool darkMode;
@override final  bool periodicSync;
@override final  UuidValue? currentRepository;

/// Create a copy of Settings
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$SettingsCopyWith<_Settings> get copyWith => __$SettingsCopyWithImpl<_Settings>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _Settings&&(identical(other.darkMode, darkMode) || other.darkMode == darkMode)&&(identical(other.periodicSync, periodicSync) || other.periodicSync == periodicSync)&&(identical(other.currentRepository, currentRepository) || other.currentRepository == currentRepository));
}


@override
int get hashCode => Object.hash(runtimeType,darkMode,periodicSync,currentRepository);

@override
String toString() {
  return 'Settings.raw(darkMode: $darkMode, periodicSync: $periodicSync, currentRepository: $currentRepository)';
}


}

/// @nodoc
abstract mixin class _$SettingsCopyWith<$Res> implements $SettingsCopyWith<$Res> {
  factory _$SettingsCopyWith(_Settings value, $Res Function(_Settings) _then) = __$SettingsCopyWithImpl;
@override @useResult
$Res call({
 bool darkMode, bool periodicSync, UuidValue? currentRepository
});




}
/// @nodoc
class __$SettingsCopyWithImpl<$Res>
    implements _$SettingsCopyWith<$Res> {
  __$SettingsCopyWithImpl(this._self, this._then);

  final _Settings _self;
  final $Res Function(_Settings) _then;

/// Create a copy of Settings
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? darkMode = null,Object? periodicSync = null,Object? currentRepository = freezed,}) {
  return _then(_Settings(
darkMode: null == darkMode ? _self.darkMode : darkMode // ignore: cast_nullable_to_non_nullable
as bool,periodicSync: null == periodicSync ? _self.periodicSync : periodicSync // ignore: cast_nullable_to_non_nullable
as bool,currentRepository: freezed == currentRepository ? _self.currentRepository : currentRepository // ignore: cast_nullable_to_non_nullable
as UuidValue?,
  ));
}


}

// dart format on
