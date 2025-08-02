// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'event.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$PluginEvent {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PluginEvent);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'PluginEvent()';
}


}

/// @nodoc
class $PluginEventCopyWith<$Res>  {
$PluginEventCopyWith(PluginEvent _, $Res Function(PluginEvent) __);
}


/// Adds pattern-matching-related methods to [PluginEvent].
extension PluginEventPatterns on PluginEvent {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( PluginEvent_TaskCreate value)?  taskCreate,TResult Function( PluginEvent_TaskModify value)?  taskModify,TResult Function( PluginEvent_TaskQuery value)?  taskQuery,TResult Function( PluginEvent_TaskSync value)?  taskSync,TResult Function( PluginEvent_NetworkRequest value)?  networkRequest,required TResult orElse(),}){
final _that = this;
switch (_that) {
case PluginEvent_TaskCreate() when taskCreate != null:
return taskCreate(_that);case PluginEvent_TaskModify() when taskModify != null:
return taskModify(_that);case PluginEvent_TaskQuery() when taskQuery != null:
return taskQuery(_that);case PluginEvent_TaskSync() when taskSync != null:
return taskSync(_that);case PluginEvent_NetworkRequest() when networkRequest != null:
return networkRequest(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( PluginEvent_TaskCreate value)  taskCreate,required TResult Function( PluginEvent_TaskModify value)  taskModify,required TResult Function( PluginEvent_TaskQuery value)  taskQuery,required TResult Function( PluginEvent_TaskSync value)  taskSync,required TResult Function( PluginEvent_NetworkRequest value)  networkRequest,}){
final _that = this;
switch (_that) {
case PluginEvent_TaskCreate():
return taskCreate(_that);case PluginEvent_TaskModify():
return taskModify(_that);case PluginEvent_TaskQuery():
return taskQuery(_that);case PluginEvent_TaskSync():
return taskSync(_that);case PluginEvent_NetworkRequest():
return networkRequest(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( PluginEvent_TaskCreate value)?  taskCreate,TResult? Function( PluginEvent_TaskModify value)?  taskModify,TResult? Function( PluginEvent_TaskQuery value)?  taskQuery,TResult? Function( PluginEvent_TaskSync value)?  taskSync,TResult? Function( PluginEvent_NetworkRequest value)?  networkRequest,}){
final _that = this;
switch (_that) {
case PluginEvent_TaskCreate() when taskCreate != null:
return taskCreate(_that);case PluginEvent_TaskModify() when taskModify != null:
return taskModify(_that);case PluginEvent_TaskQuery() when taskQuery != null:
return taskQuery(_that);case PluginEvent_TaskSync() when taskSync != null:
return taskSync(_that);case PluginEvent_NetworkRequest() when networkRequest != null:
return networkRequest(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( Task task)?  taskCreate,TResult Function( Task task)?  taskModify,TResult Function( TaskQuery query)?  taskQuery,TResult Function()?  taskSync,TResult Function( NetworkRequestType ty,  String host)?  networkRequest,required TResult orElse(),}) {final _that = this;
switch (_that) {
case PluginEvent_TaskCreate() when taskCreate != null:
return taskCreate(_that.task);case PluginEvent_TaskModify() when taskModify != null:
return taskModify(_that.task);case PluginEvent_TaskQuery() when taskQuery != null:
return taskQuery(_that.query);case PluginEvent_TaskSync() when taskSync != null:
return taskSync();case PluginEvent_NetworkRequest() when networkRequest != null:
return networkRequest(_that.ty,_that.host);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( Task task)  taskCreate,required TResult Function( Task task)  taskModify,required TResult Function( TaskQuery query)  taskQuery,required TResult Function()  taskSync,required TResult Function( NetworkRequestType ty,  String host)  networkRequest,}) {final _that = this;
switch (_that) {
case PluginEvent_TaskCreate():
return taskCreate(_that.task);case PluginEvent_TaskModify():
return taskModify(_that.task);case PluginEvent_TaskQuery():
return taskQuery(_that.query);case PluginEvent_TaskSync():
return taskSync();case PluginEvent_NetworkRequest():
return networkRequest(_that.ty,_that.host);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( Task task)?  taskCreate,TResult? Function( Task task)?  taskModify,TResult? Function( TaskQuery query)?  taskQuery,TResult? Function()?  taskSync,TResult? Function( NetworkRequestType ty,  String host)?  networkRequest,}) {final _that = this;
switch (_that) {
case PluginEvent_TaskCreate() when taskCreate != null:
return taskCreate(_that.task);case PluginEvent_TaskModify() when taskModify != null:
return taskModify(_that.task);case PluginEvent_TaskQuery() when taskQuery != null:
return taskQuery(_that.query);case PluginEvent_TaskSync() when taskSync != null:
return taskSync();case PluginEvent_NetworkRequest() when networkRequest != null:
return networkRequest(_that.ty,_that.host);case _:
  return null;

}
}

}

/// @nodoc


class PluginEvent_TaskCreate extends PluginEvent {
  const PluginEvent_TaskCreate({required this.task}): super._();
  

 final  Task task;

/// Create a copy of PluginEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PluginEvent_TaskCreateCopyWith<PluginEvent_TaskCreate> get copyWith => _$PluginEvent_TaskCreateCopyWithImpl<PluginEvent_TaskCreate>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PluginEvent_TaskCreate&&(identical(other.task, task) || other.task == task));
}


@override
int get hashCode => Object.hash(runtimeType,task);

@override
String toString() {
  return 'PluginEvent.taskCreate(task: $task)';
}


}

/// @nodoc
abstract mixin class $PluginEvent_TaskCreateCopyWith<$Res> implements $PluginEventCopyWith<$Res> {
  factory $PluginEvent_TaskCreateCopyWith(PluginEvent_TaskCreate value, $Res Function(PluginEvent_TaskCreate) _then) = _$PluginEvent_TaskCreateCopyWithImpl;
@useResult
$Res call({
 Task task
});


$TaskCopyWith<$Res> get task;

}
/// @nodoc
class _$PluginEvent_TaskCreateCopyWithImpl<$Res>
    implements $PluginEvent_TaskCreateCopyWith<$Res> {
  _$PluginEvent_TaskCreateCopyWithImpl(this._self, this._then);

  final PluginEvent_TaskCreate _self;
  final $Res Function(PluginEvent_TaskCreate) _then;

/// Create a copy of PluginEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? task = null,}) {
  return _then(PluginEvent_TaskCreate(
task: null == task ? _self.task : task // ignore: cast_nullable_to_non_nullable
as Task,
  ));
}

/// Create a copy of PluginEvent
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$TaskCopyWith<$Res> get task {
  
  return $TaskCopyWith<$Res>(_self.task, (value) {
    return _then(_self.copyWith(task: value));
  });
}
}

/// @nodoc


class PluginEvent_TaskModify extends PluginEvent {
  const PluginEvent_TaskModify({required this.task}): super._();
  

 final  Task task;

/// Create a copy of PluginEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PluginEvent_TaskModifyCopyWith<PluginEvent_TaskModify> get copyWith => _$PluginEvent_TaskModifyCopyWithImpl<PluginEvent_TaskModify>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PluginEvent_TaskModify&&(identical(other.task, task) || other.task == task));
}


@override
int get hashCode => Object.hash(runtimeType,task);

@override
String toString() {
  return 'PluginEvent.taskModify(task: $task)';
}


}

/// @nodoc
abstract mixin class $PluginEvent_TaskModifyCopyWith<$Res> implements $PluginEventCopyWith<$Res> {
  factory $PluginEvent_TaskModifyCopyWith(PluginEvent_TaskModify value, $Res Function(PluginEvent_TaskModify) _then) = _$PluginEvent_TaskModifyCopyWithImpl;
@useResult
$Res call({
 Task task
});


$TaskCopyWith<$Res> get task;

}
/// @nodoc
class _$PluginEvent_TaskModifyCopyWithImpl<$Res>
    implements $PluginEvent_TaskModifyCopyWith<$Res> {
  _$PluginEvent_TaskModifyCopyWithImpl(this._self, this._then);

  final PluginEvent_TaskModify _self;
  final $Res Function(PluginEvent_TaskModify) _then;

/// Create a copy of PluginEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? task = null,}) {
  return _then(PluginEvent_TaskModify(
task: null == task ? _self.task : task // ignore: cast_nullable_to_non_nullable
as Task,
  ));
}

/// Create a copy of PluginEvent
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$TaskCopyWith<$Res> get task {
  
  return $TaskCopyWith<$Res>(_self.task, (value) {
    return _then(_self.copyWith(task: value));
  });
}
}

/// @nodoc


class PluginEvent_TaskQuery extends PluginEvent {
  const PluginEvent_TaskQuery({required this.query}): super._();
  

 final  TaskQuery query;

/// Create a copy of PluginEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PluginEvent_TaskQueryCopyWith<PluginEvent_TaskQuery> get copyWith => _$PluginEvent_TaskQueryCopyWithImpl<PluginEvent_TaskQuery>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PluginEvent_TaskQuery&&(identical(other.query, query) || other.query == query));
}


@override
int get hashCode => Object.hash(runtimeType,query);

@override
String toString() {
  return 'PluginEvent.taskQuery(query: $query)';
}


}

/// @nodoc
abstract mixin class $PluginEvent_TaskQueryCopyWith<$Res> implements $PluginEventCopyWith<$Res> {
  factory $PluginEvent_TaskQueryCopyWith(PluginEvent_TaskQuery value, $Res Function(PluginEvent_TaskQuery) _then) = _$PluginEvent_TaskQueryCopyWithImpl;
@useResult
$Res call({
 TaskQuery query
});


$TaskQueryCopyWith<$Res> get query;

}
/// @nodoc
class _$PluginEvent_TaskQueryCopyWithImpl<$Res>
    implements $PluginEvent_TaskQueryCopyWith<$Res> {
  _$PluginEvent_TaskQueryCopyWithImpl(this._self, this._then);

  final PluginEvent_TaskQuery _self;
  final $Res Function(PluginEvent_TaskQuery) _then;

/// Create a copy of PluginEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? query = null,}) {
  return _then(PluginEvent_TaskQuery(
query: null == query ? _self.query : query // ignore: cast_nullable_to_non_nullable
as TaskQuery,
  ));
}

/// Create a copy of PluginEvent
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$TaskQueryCopyWith<$Res> get query {
  
  return $TaskQueryCopyWith<$Res>(_self.query, (value) {
    return _then(_self.copyWith(query: value));
  });
}
}

/// @nodoc


class PluginEvent_TaskSync extends PluginEvent {
  const PluginEvent_TaskSync(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PluginEvent_TaskSync);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'PluginEvent.taskSync()';
}


}




/// @nodoc


class PluginEvent_NetworkRequest extends PluginEvent {
  const PluginEvent_NetworkRequest({required this.ty, required this.host}): super._();
  

 final  NetworkRequestType ty;
 final  String host;

/// Create a copy of PluginEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PluginEvent_NetworkRequestCopyWith<PluginEvent_NetworkRequest> get copyWith => _$PluginEvent_NetworkRequestCopyWithImpl<PluginEvent_NetworkRequest>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PluginEvent_NetworkRequest&&(identical(other.ty, ty) || other.ty == ty)&&(identical(other.host, host) || other.host == host));
}


@override
int get hashCode => Object.hash(runtimeType,ty,host);

@override
String toString() {
  return 'PluginEvent.networkRequest(ty: $ty, host: $host)';
}


}

/// @nodoc
abstract mixin class $PluginEvent_NetworkRequestCopyWith<$Res> implements $PluginEventCopyWith<$Res> {
  factory $PluginEvent_NetworkRequestCopyWith(PluginEvent_NetworkRequest value, $Res Function(PluginEvent_NetworkRequest) _then) = _$PluginEvent_NetworkRequestCopyWithImpl;
@useResult
$Res call({
 NetworkRequestType ty, String host
});




}
/// @nodoc
class _$PluginEvent_NetworkRequestCopyWithImpl<$Res>
    implements $PluginEvent_NetworkRequestCopyWith<$Res> {
  _$PluginEvent_NetworkRequestCopyWithImpl(this._self, this._then);

  final PluginEvent_NetworkRequest _self;
  final $Res Function(PluginEvent_NetworkRequest) _then;

/// Create a copy of PluginEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? ty = null,Object? host = null,}) {
  return _then(PluginEvent_NetworkRequest(
ty: null == ty ? _self.ty : ty // ignore: cast_nullable_to_non_nullable
as NetworkRequestType,host: null == host ? _self.host : host // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc
mixin _$TaskQuery {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is TaskQuery);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'TaskQuery()';
}


}

/// @nodoc
class $TaskQueryCopyWith<$Res>  {
$TaskQueryCopyWith(TaskQuery _, $Res Function(TaskQuery) __);
}


/// Adds pattern-matching-related methods to [TaskQuery].
extension TaskQueryPatterns on TaskQuery {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( TaskQuery_Uuid value)?  uuid,TResult Function( TaskQuery_Title value)?  title,required TResult orElse(),}){
final _that = this;
switch (_that) {
case TaskQuery_Uuid() when uuid != null:
return uuid(_that);case TaskQuery_Title() when title != null:
return title(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( TaskQuery_Uuid value)  uuid,required TResult Function( TaskQuery_Title value)  title,}){
final _that = this;
switch (_that) {
case TaskQuery_Uuid():
return uuid(_that);case TaskQuery_Title():
return title(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( TaskQuery_Uuid value)?  uuid,TResult? Function( TaskQuery_Title value)?  title,}){
final _that = this;
switch (_that) {
case TaskQuery_Uuid() when uuid != null:
return uuid(_that);case TaskQuery_Title() when title != null:
return title(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( UuidValue uuid)?  uuid,TResult Function( String title,  Set<TaskStatus> status,  int? limit)?  title,required TResult orElse(),}) {final _that = this;
switch (_that) {
case TaskQuery_Uuid() when uuid != null:
return uuid(_that.uuid);case TaskQuery_Title() when title != null:
return title(_that.title,_that.status,_that.limit);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( UuidValue uuid)  uuid,required TResult Function( String title,  Set<TaskStatus> status,  int? limit)  title,}) {final _that = this;
switch (_that) {
case TaskQuery_Uuid():
return uuid(_that.uuid);case TaskQuery_Title():
return title(_that.title,_that.status,_that.limit);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( UuidValue uuid)?  uuid,TResult? Function( String title,  Set<TaskStatus> status,  int? limit)?  title,}) {final _that = this;
switch (_that) {
case TaskQuery_Uuid() when uuid != null:
return uuid(_that.uuid);case TaskQuery_Title() when title != null:
return title(_that.title,_that.status,_that.limit);case _:
  return null;

}
}

}

/// @nodoc


class TaskQuery_Uuid extends TaskQuery {
  const TaskQuery_Uuid({required this.uuid}): super._();
  

 final  UuidValue uuid;

/// Create a copy of TaskQuery
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$TaskQuery_UuidCopyWith<TaskQuery_Uuid> get copyWith => _$TaskQuery_UuidCopyWithImpl<TaskQuery_Uuid>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is TaskQuery_Uuid&&(identical(other.uuid, uuid) || other.uuid == uuid));
}


@override
int get hashCode => Object.hash(runtimeType,uuid);

@override
String toString() {
  return 'TaskQuery.uuid(uuid: $uuid)';
}


}

/// @nodoc
abstract mixin class $TaskQuery_UuidCopyWith<$Res> implements $TaskQueryCopyWith<$Res> {
  factory $TaskQuery_UuidCopyWith(TaskQuery_Uuid value, $Res Function(TaskQuery_Uuid) _then) = _$TaskQuery_UuidCopyWithImpl;
@useResult
$Res call({
 UuidValue uuid
});




}
/// @nodoc
class _$TaskQuery_UuidCopyWithImpl<$Res>
    implements $TaskQuery_UuidCopyWith<$Res> {
  _$TaskQuery_UuidCopyWithImpl(this._self, this._then);

  final TaskQuery_Uuid _self;
  final $Res Function(TaskQuery_Uuid) _then;

/// Create a copy of TaskQuery
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? uuid = null,}) {
  return _then(TaskQuery_Uuid(
uuid: null == uuid ? _self.uuid : uuid // ignore: cast_nullable_to_non_nullable
as UuidValue,
  ));
}


}

/// @nodoc


class TaskQuery_Title extends TaskQuery {
  const TaskQuery_Title({required this.title, required final  Set<TaskStatus> status, this.limit}): _status = status,super._();
  

 final  String title;
 final  Set<TaskStatus> _status;
 Set<TaskStatus> get status {
  if (_status is EqualUnmodifiableSetView) return _status;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableSetView(_status);
}

 final  int? limit;

/// Create a copy of TaskQuery
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$TaskQuery_TitleCopyWith<TaskQuery_Title> get copyWith => _$TaskQuery_TitleCopyWithImpl<TaskQuery_Title>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is TaskQuery_Title&&(identical(other.title, title) || other.title == title)&&const DeepCollectionEquality().equals(other._status, _status)&&(identical(other.limit, limit) || other.limit == limit));
}


@override
int get hashCode => Object.hash(runtimeType,title,const DeepCollectionEquality().hash(_status),limit);

@override
String toString() {
  return 'TaskQuery.title(title: $title, status: $status, limit: $limit)';
}


}

/// @nodoc
abstract mixin class $TaskQuery_TitleCopyWith<$Res> implements $TaskQueryCopyWith<$Res> {
  factory $TaskQuery_TitleCopyWith(TaskQuery_Title value, $Res Function(TaskQuery_Title) _then) = _$TaskQuery_TitleCopyWithImpl;
@useResult
$Res call({
 String title, Set<TaskStatus> status, int? limit
});




}
/// @nodoc
class _$TaskQuery_TitleCopyWithImpl<$Res>
    implements $TaskQuery_TitleCopyWith<$Res> {
  _$TaskQuery_TitleCopyWithImpl(this._self, this._then);

  final TaskQuery_Title _self;
  final $Res Function(TaskQuery_Title) _then;

/// Create a copy of TaskQuery
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? title = null,Object? status = null,Object? limit = freezed,}) {
  return _then(TaskQuery_Title(
title: null == title ? _self.title : title // ignore: cast_nullable_to_non_nullable
as String,status: null == status ? _self._status : status // ignore: cast_nullable_to_non_nullable
as Set<TaskStatus>,limit: freezed == limit ? _self.limit : limit // ignore: cast_nullable_to_non_nullable
as int?,
  ));
}


}

// dart format on
