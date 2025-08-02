// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'task.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$Task {

 UuidValue get uuid; DateTime get entry; TaskStatus get status; String get title; bool get active; DateTime? get modified; DateTime? get due; String? get project; List<String> get tags; List<Annotation> get annotations; TaskPriority? get priority; DateTime? get wait; List<UuidValue> get depends; List<Uda> get udas;
/// Create a copy of Task
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$TaskCopyWith<Task> get copyWith => _$TaskCopyWithImpl<Task>(this as Task, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Task&&(identical(other.uuid, uuid) || other.uuid == uuid)&&(identical(other.entry, entry) || other.entry == entry)&&(identical(other.status, status) || other.status == status)&&(identical(other.title, title) || other.title == title)&&(identical(other.active, active) || other.active == active)&&(identical(other.modified, modified) || other.modified == modified)&&(identical(other.due, due) || other.due == due)&&(identical(other.project, project) || other.project == project)&&const DeepCollectionEquality().equals(other.tags, tags)&&const DeepCollectionEquality().equals(other.annotations, annotations)&&(identical(other.priority, priority) || other.priority == priority)&&(identical(other.wait, wait) || other.wait == wait)&&const DeepCollectionEquality().equals(other.depends, depends)&&const DeepCollectionEquality().equals(other.udas, udas));
}


@override
int get hashCode => Object.hash(runtimeType,uuid,entry,status,title,active,modified,due,project,const DeepCollectionEquality().hash(tags),const DeepCollectionEquality().hash(annotations),priority,wait,const DeepCollectionEquality().hash(depends),const DeepCollectionEquality().hash(udas));

@override
String toString() {
  return 'Task(uuid: $uuid, entry: $entry, status: $status, title: $title, active: $active, modified: $modified, due: $due, project: $project, tags: $tags, annotations: $annotations, priority: $priority, wait: $wait, depends: $depends, udas: $udas)';
}


}

/// @nodoc
abstract mixin class $TaskCopyWith<$Res>  {
  factory $TaskCopyWith(Task value, $Res Function(Task) _then) = _$TaskCopyWithImpl;
@useResult
$Res call({
 UuidValue uuid, DateTime entry, TaskStatus status, String title, bool active, DateTime? modified, DateTime? due, String? project, List<String> tags, List<Annotation> annotations, TaskPriority? priority, DateTime? wait, List<UuidValue> depends, List<Uda> udas
});




}
/// @nodoc
class _$TaskCopyWithImpl<$Res>
    implements $TaskCopyWith<$Res> {
  _$TaskCopyWithImpl(this._self, this._then);

  final Task _self;
  final $Res Function(Task) _then;

/// Create a copy of Task
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? uuid = null,Object? entry = null,Object? status = null,Object? title = null,Object? active = null,Object? modified = freezed,Object? due = freezed,Object? project = freezed,Object? tags = null,Object? annotations = null,Object? priority = freezed,Object? wait = freezed,Object? depends = null,Object? udas = null,}) {
  return _then(_self.copyWith(
uuid: null == uuid ? _self.uuid : uuid // ignore: cast_nullable_to_non_nullable
as UuidValue,entry: null == entry ? _self.entry : entry // ignore: cast_nullable_to_non_nullable
as DateTime,status: null == status ? _self.status : status // ignore: cast_nullable_to_non_nullable
as TaskStatus,title: null == title ? _self.title : title // ignore: cast_nullable_to_non_nullable
as String,active: null == active ? _self.active : active // ignore: cast_nullable_to_non_nullable
as bool,modified: freezed == modified ? _self.modified : modified // ignore: cast_nullable_to_non_nullable
as DateTime?,due: freezed == due ? _self.due : due // ignore: cast_nullable_to_non_nullable
as DateTime?,project: freezed == project ? _self.project : project // ignore: cast_nullable_to_non_nullable
as String?,tags: null == tags ? _self.tags : tags // ignore: cast_nullable_to_non_nullable
as List<String>,annotations: null == annotations ? _self.annotations : annotations // ignore: cast_nullable_to_non_nullable
as List<Annotation>,priority: freezed == priority ? _self.priority : priority // ignore: cast_nullable_to_non_nullable
as TaskPriority?,wait: freezed == wait ? _self.wait : wait // ignore: cast_nullable_to_non_nullable
as DateTime?,depends: null == depends ? _self.depends : depends // ignore: cast_nullable_to_non_nullable
as List<UuidValue>,udas: null == udas ? _self.udas : udas // ignore: cast_nullable_to_non_nullable
as List<Uda>,
  ));
}

}


/// Adds pattern-matching-related methods to [Task].
extension TaskPatterns on Task {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( _Task value)?  raw,required TResult orElse(),}){
final _that = this;
switch (_that) {
case _Task() when raw != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( _Task value)  raw,}){
final _that = this;
switch (_that) {
case _Task():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( _Task value)?  raw,}){
final _that = this;
switch (_that) {
case _Task() when raw != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( UuidValue uuid,  DateTime entry,  TaskStatus status,  String title,  bool active,  DateTime? modified,  DateTime? due,  String? project,  List<String> tags,  List<Annotation> annotations,  TaskPriority? priority,  DateTime? wait,  List<UuidValue> depends,  List<Uda> udas)?  raw,required TResult orElse(),}) {final _that = this;
switch (_that) {
case _Task() when raw != null:
return raw(_that.uuid,_that.entry,_that.status,_that.title,_that.active,_that.modified,_that.due,_that.project,_that.tags,_that.annotations,_that.priority,_that.wait,_that.depends,_that.udas);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( UuidValue uuid,  DateTime entry,  TaskStatus status,  String title,  bool active,  DateTime? modified,  DateTime? due,  String? project,  List<String> tags,  List<Annotation> annotations,  TaskPriority? priority,  DateTime? wait,  List<UuidValue> depends,  List<Uda> udas)  raw,}) {final _that = this;
switch (_that) {
case _Task():
return raw(_that.uuid,_that.entry,_that.status,_that.title,_that.active,_that.modified,_that.due,_that.project,_that.tags,_that.annotations,_that.priority,_that.wait,_that.depends,_that.udas);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( UuidValue uuid,  DateTime entry,  TaskStatus status,  String title,  bool active,  DateTime? modified,  DateTime? due,  String? project,  List<String> tags,  List<Annotation> annotations,  TaskPriority? priority,  DateTime? wait,  List<UuidValue> depends,  List<Uda> udas)?  raw,}) {final _that = this;
switch (_that) {
case _Task() when raw != null:
return raw(_that.uuid,_that.entry,_that.status,_that.title,_that.active,_that.modified,_that.due,_that.project,_that.tags,_that.annotations,_that.priority,_that.wait,_that.depends,_that.udas);case _:
  return null;

}
}

}

/// @nodoc


class _Task extends Task {
  const _Task({required this.uuid, required this.entry, required this.status, required this.title, required this.active, this.modified, this.due, this.project, required final  List<String> tags, required final  List<Annotation> annotations, this.priority, this.wait, required final  List<UuidValue> depends, required final  List<Uda> udas}): _tags = tags,_annotations = annotations,_depends = depends,_udas = udas,super._();
  

@override final  UuidValue uuid;
@override final  DateTime entry;
@override final  TaskStatus status;
@override final  String title;
@override final  bool active;
@override final  DateTime? modified;
@override final  DateTime? due;
@override final  String? project;
 final  List<String> _tags;
@override List<String> get tags {
  if (_tags is EqualUnmodifiableListView) return _tags;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_tags);
}

 final  List<Annotation> _annotations;
@override List<Annotation> get annotations {
  if (_annotations is EqualUnmodifiableListView) return _annotations;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_annotations);
}

@override final  TaskPriority? priority;
@override final  DateTime? wait;
 final  List<UuidValue> _depends;
@override List<UuidValue> get depends {
  if (_depends is EqualUnmodifiableListView) return _depends;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_depends);
}

 final  List<Uda> _udas;
@override List<Uda> get udas {
  if (_udas is EqualUnmodifiableListView) return _udas;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_udas);
}


/// Create a copy of Task
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$TaskCopyWith<_Task> get copyWith => __$TaskCopyWithImpl<_Task>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _Task&&(identical(other.uuid, uuid) || other.uuid == uuid)&&(identical(other.entry, entry) || other.entry == entry)&&(identical(other.status, status) || other.status == status)&&(identical(other.title, title) || other.title == title)&&(identical(other.active, active) || other.active == active)&&(identical(other.modified, modified) || other.modified == modified)&&(identical(other.due, due) || other.due == due)&&(identical(other.project, project) || other.project == project)&&const DeepCollectionEquality().equals(other._tags, _tags)&&const DeepCollectionEquality().equals(other._annotations, _annotations)&&(identical(other.priority, priority) || other.priority == priority)&&(identical(other.wait, wait) || other.wait == wait)&&const DeepCollectionEquality().equals(other._depends, _depends)&&const DeepCollectionEquality().equals(other._udas, _udas));
}


@override
int get hashCode => Object.hash(runtimeType,uuid,entry,status,title,active,modified,due,project,const DeepCollectionEquality().hash(_tags),const DeepCollectionEquality().hash(_annotations),priority,wait,const DeepCollectionEquality().hash(_depends),const DeepCollectionEquality().hash(_udas));

@override
String toString() {
  return 'Task.raw(uuid: $uuid, entry: $entry, status: $status, title: $title, active: $active, modified: $modified, due: $due, project: $project, tags: $tags, annotations: $annotations, priority: $priority, wait: $wait, depends: $depends, udas: $udas)';
}


}

/// @nodoc
abstract mixin class _$TaskCopyWith<$Res> implements $TaskCopyWith<$Res> {
  factory _$TaskCopyWith(_Task value, $Res Function(_Task) _then) = __$TaskCopyWithImpl;
@override @useResult
$Res call({
 UuidValue uuid, DateTime entry, TaskStatus status, String title, bool active, DateTime? modified, DateTime? due, String? project, List<String> tags, List<Annotation> annotations, TaskPriority? priority, DateTime? wait, List<UuidValue> depends, List<Uda> udas
});




}
/// @nodoc
class __$TaskCopyWithImpl<$Res>
    implements _$TaskCopyWith<$Res> {
  __$TaskCopyWithImpl(this._self, this._then);

  final _Task _self;
  final $Res Function(_Task) _then;

/// Create a copy of Task
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? uuid = null,Object? entry = null,Object? status = null,Object? title = null,Object? active = null,Object? modified = freezed,Object? due = freezed,Object? project = freezed,Object? tags = null,Object? annotations = null,Object? priority = freezed,Object? wait = freezed,Object? depends = null,Object? udas = null,}) {
  return _then(_Task(
uuid: null == uuid ? _self.uuid : uuid // ignore: cast_nullable_to_non_nullable
as UuidValue,entry: null == entry ? _self.entry : entry // ignore: cast_nullable_to_non_nullable
as DateTime,status: null == status ? _self.status : status // ignore: cast_nullable_to_non_nullable
as TaskStatus,title: null == title ? _self.title : title // ignore: cast_nullable_to_non_nullable
as String,active: null == active ? _self.active : active // ignore: cast_nullable_to_non_nullable
as bool,modified: freezed == modified ? _self.modified : modified // ignore: cast_nullable_to_non_nullable
as DateTime?,due: freezed == due ? _self.due : due // ignore: cast_nullable_to_non_nullable
as DateTime?,project: freezed == project ? _self.project : project // ignore: cast_nullable_to_non_nullable
as String?,tags: null == tags ? _self._tags : tags // ignore: cast_nullable_to_non_nullable
as List<String>,annotations: null == annotations ? _self._annotations : annotations // ignore: cast_nullable_to_non_nullable
as List<Annotation>,priority: freezed == priority ? _self.priority : priority // ignore: cast_nullable_to_non_nullable
as TaskPriority?,wait: freezed == wait ? _self.wait : wait // ignore: cast_nullable_to_non_nullable
as DateTime?,depends: null == depends ? _self._depends : depends // ignore: cast_nullable_to_non_nullable
as List<UuidValue>,udas: null == udas ? _self._udas : udas // ignore: cast_nullable_to_non_nullable
as List<Uda>,
  ));
}


}

// dart format on
