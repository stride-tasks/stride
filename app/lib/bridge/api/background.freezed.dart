// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'background.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$BackgroundResult {

 String get task;
/// Create a copy of BackgroundResult
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$BackgroundResultCopyWith<BackgroundResult> get copyWith => _$BackgroundResultCopyWithImpl<BackgroundResult>(this as BackgroundResult, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is BackgroundResult&&(identical(other.task, task) || other.task == task));
}


@override
int get hashCode => Object.hash(runtimeType,task);

@override
String toString() {
  return 'BackgroundResult(task: $task)';
}


}

/// @nodoc
abstract mixin class $BackgroundResultCopyWith<$Res>  {
  factory $BackgroundResultCopyWith(BackgroundResult value, $Res Function(BackgroundResult) _then) = _$BackgroundResultCopyWithImpl;
@useResult
$Res call({
 String task
});




}
/// @nodoc
class _$BackgroundResultCopyWithImpl<$Res>
    implements $BackgroundResultCopyWith<$Res> {
  _$BackgroundResultCopyWithImpl(this._self, this._then);

  final BackgroundResult _self;
  final $Res Function(BackgroundResult) _then;

/// Create a copy of BackgroundResult
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? task = null,}) {
  return _then(_self.copyWith(
task: null == task ? _self.task : task // ignore: cast_nullable_to_non_nullable
as String,
  ));
}

}


/// Adds pattern-matching-related methods to [BackgroundResult].
extension BackgroundResultPatterns on BackgroundResult {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( BackgroundResult_Start value)?  start,TResult Function( BackgroundResult_Done value)?  done,TResult Function( BackgroundResult_Error value)?  error,required TResult orElse(),}){
final _that = this;
switch (_that) {
case BackgroundResult_Start() when start != null:
return start(_that);case BackgroundResult_Done() when done != null:
return done(_that);case BackgroundResult_Error() when error != null:
return error(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( BackgroundResult_Start value)  start,required TResult Function( BackgroundResult_Done value)  done,required TResult Function( BackgroundResult_Error value)  error,}){
final _that = this;
switch (_that) {
case BackgroundResult_Start():
return start(_that);case BackgroundResult_Done():
return done(_that);case BackgroundResult_Error():
return error(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( BackgroundResult_Start value)?  start,TResult? Function( BackgroundResult_Done value)?  done,TResult? Function( BackgroundResult_Error value)?  error,}){
final _that = this;
switch (_that) {
case BackgroundResult_Start() when start != null:
return start(_that);case BackgroundResult_Done() when done != null:
return done(_that);case BackgroundResult_Error() when error != null:
return error(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( String task)?  start,TResult Function( String task,  bool success)?  done,TResult Function( String task,  RustError error)?  error,required TResult orElse(),}) {final _that = this;
switch (_that) {
case BackgroundResult_Start() when start != null:
return start(_that.task);case BackgroundResult_Done() when done != null:
return done(_that.task,_that.success);case BackgroundResult_Error() when error != null:
return error(_that.task,_that.error);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( String task)  start,required TResult Function( String task,  bool success)  done,required TResult Function( String task,  RustError error)  error,}) {final _that = this;
switch (_that) {
case BackgroundResult_Start():
return start(_that.task);case BackgroundResult_Done():
return done(_that.task,_that.success);case BackgroundResult_Error():
return error(_that.task,_that.error);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( String task)?  start,TResult? Function( String task,  bool success)?  done,TResult? Function( String task,  RustError error)?  error,}) {final _that = this;
switch (_that) {
case BackgroundResult_Start() when start != null:
return start(_that.task);case BackgroundResult_Done() when done != null:
return done(_that.task,_that.success);case BackgroundResult_Error() when error != null:
return error(_that.task,_that.error);case _:
  return null;

}
}

}

/// @nodoc


class BackgroundResult_Start extends BackgroundResult {
  const BackgroundResult_Start({required this.task}): super._();
  

@override final  String task;

/// Create a copy of BackgroundResult
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$BackgroundResult_StartCopyWith<BackgroundResult_Start> get copyWith => _$BackgroundResult_StartCopyWithImpl<BackgroundResult_Start>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is BackgroundResult_Start&&(identical(other.task, task) || other.task == task));
}


@override
int get hashCode => Object.hash(runtimeType,task);

@override
String toString() {
  return 'BackgroundResult.start(task: $task)';
}


}

/// @nodoc
abstract mixin class $BackgroundResult_StartCopyWith<$Res> implements $BackgroundResultCopyWith<$Res> {
  factory $BackgroundResult_StartCopyWith(BackgroundResult_Start value, $Res Function(BackgroundResult_Start) _then) = _$BackgroundResult_StartCopyWithImpl;
@override @useResult
$Res call({
 String task
});




}
/// @nodoc
class _$BackgroundResult_StartCopyWithImpl<$Res>
    implements $BackgroundResult_StartCopyWith<$Res> {
  _$BackgroundResult_StartCopyWithImpl(this._self, this._then);

  final BackgroundResult_Start _self;
  final $Res Function(BackgroundResult_Start) _then;

/// Create a copy of BackgroundResult
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? task = null,}) {
  return _then(BackgroundResult_Start(
task: null == task ? _self.task : task // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class BackgroundResult_Done extends BackgroundResult {
  const BackgroundResult_Done({required this.task, required this.success}): super._();
  

@override final  String task;
 final  bool success;

/// Create a copy of BackgroundResult
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$BackgroundResult_DoneCopyWith<BackgroundResult_Done> get copyWith => _$BackgroundResult_DoneCopyWithImpl<BackgroundResult_Done>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is BackgroundResult_Done&&(identical(other.task, task) || other.task == task)&&(identical(other.success, success) || other.success == success));
}


@override
int get hashCode => Object.hash(runtimeType,task,success);

@override
String toString() {
  return 'BackgroundResult.done(task: $task, success: $success)';
}


}

/// @nodoc
abstract mixin class $BackgroundResult_DoneCopyWith<$Res> implements $BackgroundResultCopyWith<$Res> {
  factory $BackgroundResult_DoneCopyWith(BackgroundResult_Done value, $Res Function(BackgroundResult_Done) _then) = _$BackgroundResult_DoneCopyWithImpl;
@override @useResult
$Res call({
 String task, bool success
});




}
/// @nodoc
class _$BackgroundResult_DoneCopyWithImpl<$Res>
    implements $BackgroundResult_DoneCopyWith<$Res> {
  _$BackgroundResult_DoneCopyWithImpl(this._self, this._then);

  final BackgroundResult_Done _self;
  final $Res Function(BackgroundResult_Done) _then;

/// Create a copy of BackgroundResult
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? task = null,Object? success = null,}) {
  return _then(BackgroundResult_Done(
task: null == task ? _self.task : task // ignore: cast_nullable_to_non_nullable
as String,success: null == success ? _self.success : success // ignore: cast_nullable_to_non_nullable
as bool,
  ));
}


}

/// @nodoc


class BackgroundResult_Error extends BackgroundResult {
  const BackgroundResult_Error({required this.task, required this.error}): super._();
  

@override final  String task;
 final  RustError error;

/// Create a copy of BackgroundResult
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$BackgroundResult_ErrorCopyWith<BackgroundResult_Error> get copyWith => _$BackgroundResult_ErrorCopyWithImpl<BackgroundResult_Error>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is BackgroundResult_Error&&(identical(other.task, task) || other.task == task)&&(identical(other.error, error) || other.error == error));
}


@override
int get hashCode => Object.hash(runtimeType,task,error);

@override
String toString() {
  return 'BackgroundResult.error(task: $task, error: $error)';
}


}

/// @nodoc
abstract mixin class $BackgroundResult_ErrorCopyWith<$Res> implements $BackgroundResultCopyWith<$Res> {
  factory $BackgroundResult_ErrorCopyWith(BackgroundResult_Error value, $Res Function(BackgroundResult_Error) _then) = _$BackgroundResult_ErrorCopyWithImpl;
@override @useResult
$Res call({
 String task, RustError error
});




}
/// @nodoc
class _$BackgroundResult_ErrorCopyWithImpl<$Res>
    implements $BackgroundResult_ErrorCopyWith<$Res> {
  _$BackgroundResult_ErrorCopyWithImpl(this._self, this._then);

  final BackgroundResult_Error _self;
  final $Res Function(BackgroundResult_Error) _then;

/// Create a copy of BackgroundResult
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? task = null,Object? error = null,}) {
  return _then(BackgroundResult_Error(
task: null == task ? _self.task : task // ignore: cast_nullable_to_non_nullable
as String,error: null == error ? _self.error : error // ignore: cast_nullable_to_non_nullable
as RustError,
  ));
}


}

// dart format on
