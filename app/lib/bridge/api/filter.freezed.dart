// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'filter.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$Filter {

 UuidValue get uuid; String get name; Set<TaskStatus> get status; String get search;
/// Create a copy of Filter
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$FilterCopyWith<Filter> get copyWith => _$FilterCopyWithImpl<Filter>(this as Filter, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Filter&&(identical(other.uuid, uuid) || other.uuid == uuid)&&(identical(other.name, name) || other.name == name)&&const DeepCollectionEquality().equals(other.status, status)&&(identical(other.search, search) || other.search == search));
}


@override
int get hashCode => Object.hash(runtimeType,uuid,name,const DeepCollectionEquality().hash(status),search);

@override
String toString() {
  return 'Filter(uuid: $uuid, name: $name, status: $status, search: $search)';
}


}

/// @nodoc
abstract mixin class $FilterCopyWith<$Res>  {
  factory $FilterCopyWith(Filter value, $Res Function(Filter) _then) = _$FilterCopyWithImpl;
@useResult
$Res call({
 UuidValue uuid, String name, Set<TaskStatus> status, String search
});




}
/// @nodoc
class _$FilterCopyWithImpl<$Res>
    implements $FilterCopyWith<$Res> {
  _$FilterCopyWithImpl(this._self, this._then);

  final Filter _self;
  final $Res Function(Filter) _then;

/// Create a copy of Filter
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? uuid = null,Object? name = null,Object? status = null,Object? search = null,}) {
  return _then(_self.copyWith(
uuid: null == uuid ? _self.uuid : uuid // ignore: cast_nullable_to_non_nullable
as UuidValue,name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,status: null == status ? _self.status : status // ignore: cast_nullable_to_non_nullable
as Set<TaskStatus>,search: null == search ? _self.search : search // ignore: cast_nullable_to_non_nullable
as String,
  ));
}

}


/// Adds pattern-matching-related methods to [Filter].
extension FilterPatterns on Filter {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _Filter value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _Filter() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _Filter value)  $default,){
final _that = this;
switch (_that) {
case _Filter():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _Filter value)?  $default,){
final _that = this;
switch (_that) {
case _Filter() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( UuidValue uuid,  String name,  Set<TaskStatus> status,  String search)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _Filter() when $default != null:
return $default(_that.uuid,_that.name,_that.status,_that.search);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( UuidValue uuid,  String name,  Set<TaskStatus> status,  String search)  $default,) {final _that = this;
switch (_that) {
case _Filter():
return $default(_that.uuid,_that.name,_that.status,_that.search);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( UuidValue uuid,  String name,  Set<TaskStatus> status,  String search)?  $default,) {final _that = this;
switch (_that) {
case _Filter() when $default != null:
return $default(_that.uuid,_that.name,_that.status,_that.search);case _:
  return null;

}
}

}

/// @nodoc


class _Filter extends Filter {
  const _Filter({required this.uuid, required this.name, required final  Set<TaskStatus> status, required this.search}): _status = status,super._();
  

@override final  UuidValue uuid;
@override final  String name;
 final  Set<TaskStatus> _status;
@override Set<TaskStatus> get status {
  if (_status is EqualUnmodifiableSetView) return _status;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableSetView(_status);
}

@override final  String search;

/// Create a copy of Filter
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$FilterCopyWith<_Filter> get copyWith => __$FilterCopyWithImpl<_Filter>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _Filter&&(identical(other.uuid, uuid) || other.uuid == uuid)&&(identical(other.name, name) || other.name == name)&&const DeepCollectionEquality().equals(other._status, _status)&&(identical(other.search, search) || other.search == search));
}


@override
int get hashCode => Object.hash(runtimeType,uuid,name,const DeepCollectionEquality().hash(_status),search);

@override
String toString() {
  return 'Filter(uuid: $uuid, name: $name, status: $status, search: $search)';
}


}

/// @nodoc
abstract mixin class _$FilterCopyWith<$Res> implements $FilterCopyWith<$Res> {
  factory _$FilterCopyWith(_Filter value, $Res Function(_Filter) _then) = __$FilterCopyWithImpl;
@override @useResult
$Res call({
 UuidValue uuid, String name, Set<TaskStatus> status, String search
});




}
/// @nodoc
class __$FilterCopyWithImpl<$Res>
    implements _$FilterCopyWith<$Res> {
  __$FilterCopyWithImpl(this._self, this._then);

  final _Filter _self;
  final $Res Function(_Filter) _then;

/// Create a copy of Filter
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? uuid = null,Object? name = null,Object? status = null,Object? search = null,}) {
  return _then(_Filter(
uuid: null == uuid ? _self.uuid : uuid // ignore: cast_nullable_to_non_nullable
as UuidValue,name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,status: null == status ? _self._status : status // ignore: cast_nullable_to_non_nullable
as Set<TaskStatus>,search: null == search ? _self.search : search // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc
mixin _$FilterSelection {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is FilterSelection);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'FilterSelection()';
}


}

/// @nodoc
class $FilterSelectionCopyWith<$Res>  {
$FilterSelectionCopyWith(FilterSelection _, $Res Function(FilterSelection) __);
}


/// Adds pattern-matching-related methods to [FilterSelection].
extension FilterSelectionPatterns on FilterSelection {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( FilterSelection_Predefined value)?  predefined,TResult Function( FilterSelection_Temporary value)?  temporary,required TResult orElse(),}){
final _that = this;
switch (_that) {
case FilterSelection_Predefined() when predefined != null:
return predefined(_that);case FilterSelection_Temporary() when temporary != null:
return temporary(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( FilterSelection_Predefined value)  predefined,required TResult Function( FilterSelection_Temporary value)  temporary,}){
final _that = this;
switch (_that) {
case FilterSelection_Predefined():
return predefined(_that);case FilterSelection_Temporary():
return temporary(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( FilterSelection_Predefined value)?  predefined,TResult? Function( FilterSelection_Temporary value)?  temporary,}){
final _that = this;
switch (_that) {
case FilterSelection_Predefined() when predefined != null:
return predefined(_that);case FilterSelection_Temporary() when temporary != null:
return temporary(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( UuidValue uuid)?  predefined,TResult Function( Filter filter)?  temporary,required TResult orElse(),}) {final _that = this;
switch (_that) {
case FilterSelection_Predefined() when predefined != null:
return predefined(_that.uuid);case FilterSelection_Temporary() when temporary != null:
return temporary(_that.filter);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( UuidValue uuid)  predefined,required TResult Function( Filter filter)  temporary,}) {final _that = this;
switch (_that) {
case FilterSelection_Predefined():
return predefined(_that.uuid);case FilterSelection_Temporary():
return temporary(_that.filter);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( UuidValue uuid)?  predefined,TResult? Function( Filter filter)?  temporary,}) {final _that = this;
switch (_that) {
case FilterSelection_Predefined() when predefined != null:
return predefined(_that.uuid);case FilterSelection_Temporary() when temporary != null:
return temporary(_that.filter);case _:
  return null;

}
}

}

/// @nodoc


class FilterSelection_Predefined extends FilterSelection {
  const FilterSelection_Predefined({required this.uuid}): super._();
  

 final  UuidValue uuid;

/// Create a copy of FilterSelection
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$FilterSelection_PredefinedCopyWith<FilterSelection_Predefined> get copyWith => _$FilterSelection_PredefinedCopyWithImpl<FilterSelection_Predefined>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is FilterSelection_Predefined&&(identical(other.uuid, uuid) || other.uuid == uuid));
}


@override
int get hashCode => Object.hash(runtimeType,uuid);

@override
String toString() {
  return 'FilterSelection.predefined(uuid: $uuid)';
}


}

/// @nodoc
abstract mixin class $FilterSelection_PredefinedCopyWith<$Res> implements $FilterSelectionCopyWith<$Res> {
  factory $FilterSelection_PredefinedCopyWith(FilterSelection_Predefined value, $Res Function(FilterSelection_Predefined) _then) = _$FilterSelection_PredefinedCopyWithImpl;
@useResult
$Res call({
 UuidValue uuid
});




}
/// @nodoc
class _$FilterSelection_PredefinedCopyWithImpl<$Res>
    implements $FilterSelection_PredefinedCopyWith<$Res> {
  _$FilterSelection_PredefinedCopyWithImpl(this._self, this._then);

  final FilterSelection_Predefined _self;
  final $Res Function(FilterSelection_Predefined) _then;

/// Create a copy of FilterSelection
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? uuid = null,}) {
  return _then(FilterSelection_Predefined(
uuid: null == uuid ? _self.uuid : uuid // ignore: cast_nullable_to_non_nullable
as UuidValue,
  ));
}


}

/// @nodoc


class FilterSelection_Temporary extends FilterSelection {
  const FilterSelection_Temporary({required this.filter}): super._();
  

 final  Filter filter;

/// Create a copy of FilterSelection
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$FilterSelection_TemporaryCopyWith<FilterSelection_Temporary> get copyWith => _$FilterSelection_TemporaryCopyWithImpl<FilterSelection_Temporary>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is FilterSelection_Temporary&&(identical(other.filter, filter) || other.filter == filter));
}


@override
int get hashCode => Object.hash(runtimeType,filter);

@override
String toString() {
  return 'FilterSelection.temporary(filter: $filter)';
}


}

/// @nodoc
abstract mixin class $FilterSelection_TemporaryCopyWith<$Res> implements $FilterSelectionCopyWith<$Res> {
  factory $FilterSelection_TemporaryCopyWith(FilterSelection_Temporary value, $Res Function(FilterSelection_Temporary) _then) = _$FilterSelection_TemporaryCopyWithImpl;
@useResult
$Res call({
 Filter filter
});


$FilterCopyWith<$Res> get filter;

}
/// @nodoc
class _$FilterSelection_TemporaryCopyWithImpl<$Res>
    implements $FilterSelection_TemporaryCopyWith<$Res> {
  _$FilterSelection_TemporaryCopyWithImpl(this._self, this._then);

  final FilterSelection_Temporary _self;
  final $Res Function(FilterSelection_Temporary) _then;

/// Create a copy of FilterSelection
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? filter = null,}) {
  return _then(FilterSelection_Temporary(
filter: null == filter ? _self.filter : filter // ignore: cast_nullable_to_non_nullable
as Filter,
  ));
}

/// Create a copy of FilterSelection
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$FilterCopyWith<$Res> get filter {
  
  return $FilterCopyWith<$Res>(_self.filter, (value) {
    return _then(_self.copyWith(filter: value));
  });
}
}

// dart format on
