// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'task.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$Task {
  UuidValue get uuid => throw _privateConstructorUsedError;
  TaskStatus get status => throw _privateConstructorUsedError;
  String get title => throw _privateConstructorUsedError;
  bool get active => throw _privateConstructorUsedError;
  DateTime? get modified => throw _privateConstructorUsedError;
  DateTime? get due => throw _privateConstructorUsedError;
  String? get project => throw _privateConstructorUsedError;
  Uint32List get tags => throw _privateConstructorUsedError;
  List<Annotation> get annotations => throw _privateConstructorUsedError;
  TaskPriority? get priority => throw _privateConstructorUsedError;
  DateTime? get wait => throw _privateConstructorUsedError;
  List<UuidValue> get depends => throw _privateConstructorUsedError;
  Map<String, String> get uda => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(
            UuidValue uuid,
            TaskStatus status,
            String title,
            bool active,
            DateTime? modified,
            DateTime? due,
            String? project,
            Uint32List tags,
            List<Annotation> annotations,
            TaskPriority? priority,
            DateTime? wait,
            List<UuidValue> depends,
            Map<String, String> uda)
        raw,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(
            UuidValue uuid,
            TaskStatus status,
            String title,
            bool active,
            DateTime? modified,
            DateTime? due,
            String? project,
            Uint32List tags,
            List<Annotation> annotations,
            TaskPriority? priority,
            DateTime? wait,
            List<UuidValue> depends,
            Map<String, String> uda)?
        raw,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(
            UuidValue uuid,
            TaskStatus status,
            String title,
            bool active,
            DateTime? modified,
            DateTime? due,
            String? project,
            Uint32List tags,
            List<Annotation> annotations,
            TaskPriority? priority,
            DateTime? wait,
            List<UuidValue> depends,
            Map<String, String> uda)?
        raw,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(_Task value) raw,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(_Task value)? raw,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(_Task value)? raw,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;

  /// Create a copy of Task
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $TaskCopyWith<Task> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $TaskCopyWith<$Res> {
  factory $TaskCopyWith(Task value, $Res Function(Task) then) =
      _$TaskCopyWithImpl<$Res, Task>;
  @useResult
  $Res call(
      {UuidValue uuid,
      TaskStatus status,
      String title,
      bool active,
      DateTime? modified,
      DateTime? due,
      String? project,
      Uint32List tags,
      List<Annotation> annotations,
      TaskPriority? priority,
      DateTime? wait,
      List<UuidValue> depends,
      Map<String, String> uda});
}

/// @nodoc
class _$TaskCopyWithImpl<$Res, $Val extends Task>
    implements $TaskCopyWith<$Res> {
  _$TaskCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of Task
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? uuid = null,
    Object? status = null,
    Object? title = null,
    Object? active = null,
    Object? modified = freezed,
    Object? due = freezed,
    Object? project = freezed,
    Object? tags = null,
    Object? annotations = null,
    Object? priority = freezed,
    Object? wait = freezed,
    Object? depends = null,
    Object? uda = null,
  }) {
    return _then(_value.copyWith(
      uuid: null == uuid
          ? _value.uuid
          : uuid // ignore: cast_nullable_to_non_nullable
              as UuidValue,
      status: null == status
          ? _value.status
          : status // ignore: cast_nullable_to_non_nullable
              as TaskStatus,
      title: null == title
          ? _value.title
          : title // ignore: cast_nullable_to_non_nullable
              as String,
      active: null == active
          ? _value.active
          : active // ignore: cast_nullable_to_non_nullable
              as bool,
      modified: freezed == modified
          ? _value.modified
          : modified // ignore: cast_nullable_to_non_nullable
              as DateTime?,
      due: freezed == due
          ? _value.due
          : due // ignore: cast_nullable_to_non_nullable
              as DateTime?,
      project: freezed == project
          ? _value.project
          : project // ignore: cast_nullable_to_non_nullable
              as String?,
      tags: null == tags
          ? _value.tags
          : tags // ignore: cast_nullable_to_non_nullable
              as Uint32List,
      annotations: null == annotations
          ? _value.annotations
          : annotations // ignore: cast_nullable_to_non_nullable
              as List<Annotation>,
      priority: freezed == priority
          ? _value.priority
          : priority // ignore: cast_nullable_to_non_nullable
              as TaskPriority?,
      wait: freezed == wait
          ? _value.wait
          : wait // ignore: cast_nullable_to_non_nullable
              as DateTime?,
      depends: null == depends
          ? _value.depends
          : depends // ignore: cast_nullable_to_non_nullable
              as List<UuidValue>,
      uda: null == uda
          ? _value.uda
          : uda // ignore: cast_nullable_to_non_nullable
              as Map<String, String>,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$TaskImplCopyWith<$Res> implements $TaskCopyWith<$Res> {
  factory _$$TaskImplCopyWith(
          _$TaskImpl value, $Res Function(_$TaskImpl) then) =
      __$$TaskImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {UuidValue uuid,
      TaskStatus status,
      String title,
      bool active,
      DateTime? modified,
      DateTime? due,
      String? project,
      Uint32List tags,
      List<Annotation> annotations,
      TaskPriority? priority,
      DateTime? wait,
      List<UuidValue> depends,
      Map<String, String> uda});
}

/// @nodoc
class __$$TaskImplCopyWithImpl<$Res>
    extends _$TaskCopyWithImpl<$Res, _$TaskImpl>
    implements _$$TaskImplCopyWith<$Res> {
  __$$TaskImplCopyWithImpl(_$TaskImpl _value, $Res Function(_$TaskImpl) _then)
      : super(_value, _then);

  /// Create a copy of Task
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? uuid = null,
    Object? status = null,
    Object? title = null,
    Object? active = null,
    Object? modified = freezed,
    Object? due = freezed,
    Object? project = freezed,
    Object? tags = null,
    Object? annotations = null,
    Object? priority = freezed,
    Object? wait = freezed,
    Object? depends = null,
    Object? uda = null,
  }) {
    return _then(_$TaskImpl(
      uuid: null == uuid
          ? _value.uuid
          : uuid // ignore: cast_nullable_to_non_nullable
              as UuidValue,
      status: null == status
          ? _value.status
          : status // ignore: cast_nullable_to_non_nullable
              as TaskStatus,
      title: null == title
          ? _value.title
          : title // ignore: cast_nullable_to_non_nullable
              as String,
      active: null == active
          ? _value.active
          : active // ignore: cast_nullable_to_non_nullable
              as bool,
      modified: freezed == modified
          ? _value.modified
          : modified // ignore: cast_nullable_to_non_nullable
              as DateTime?,
      due: freezed == due
          ? _value.due
          : due // ignore: cast_nullable_to_non_nullable
              as DateTime?,
      project: freezed == project
          ? _value.project
          : project // ignore: cast_nullable_to_non_nullable
              as String?,
      tags: null == tags
          ? _value.tags
          : tags // ignore: cast_nullable_to_non_nullable
              as Uint32List,
      annotations: null == annotations
          ? _value._annotations
          : annotations // ignore: cast_nullable_to_non_nullable
              as List<Annotation>,
      priority: freezed == priority
          ? _value.priority
          : priority // ignore: cast_nullable_to_non_nullable
              as TaskPriority?,
      wait: freezed == wait
          ? _value.wait
          : wait // ignore: cast_nullable_to_non_nullable
              as DateTime?,
      depends: null == depends
          ? _value._depends
          : depends // ignore: cast_nullable_to_non_nullable
              as List<UuidValue>,
      uda: null == uda
          ? _value._uda
          : uda // ignore: cast_nullable_to_non_nullable
              as Map<String, String>,
    ));
  }
}

/// @nodoc

class _$TaskImpl extends _Task {
  const _$TaskImpl(
      {required this.uuid,
      required this.status,
      required this.title,
      required this.active,
      this.modified,
      this.due,
      this.project,
      required this.tags,
      required final List<Annotation> annotations,
      this.priority,
      this.wait,
      required final List<UuidValue> depends,
      required final Map<String, String> uda})
      : _annotations = annotations,
        _depends = depends,
        _uda = uda,
        super._();

  @override
  final UuidValue uuid;
  @override
  final TaskStatus status;
  @override
  final String title;
  @override
  final bool active;
  @override
  final DateTime? modified;
  @override
  final DateTime? due;
  @override
  final String? project;
  @override
  final Uint32List tags;
  final List<Annotation> _annotations;
  @override
  List<Annotation> get annotations {
    if (_annotations is EqualUnmodifiableListView) return _annotations;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_annotations);
  }

  @override
  final TaskPriority? priority;
  @override
  final DateTime? wait;
  final List<UuidValue> _depends;
  @override
  List<UuidValue> get depends {
    if (_depends is EqualUnmodifiableListView) return _depends;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_depends);
  }

  final Map<String, String> _uda;
  @override
  Map<String, String> get uda {
    if (_uda is EqualUnmodifiableMapView) return _uda;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableMapView(_uda);
  }

  @override
  String toString() {
    return 'Task.raw(uuid: $uuid, status: $status, title: $title, active: $active, modified: $modified, due: $due, project: $project, tags: $tags, annotations: $annotations, priority: $priority, wait: $wait, depends: $depends, uda: $uda)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$TaskImpl &&
            (identical(other.uuid, uuid) || other.uuid == uuid) &&
            (identical(other.status, status) || other.status == status) &&
            (identical(other.title, title) || other.title == title) &&
            (identical(other.active, active) || other.active == active) &&
            (identical(other.modified, modified) ||
                other.modified == modified) &&
            (identical(other.due, due) || other.due == due) &&
            (identical(other.project, project) || other.project == project) &&
            const DeepCollectionEquality().equals(other.tags, tags) &&
            const DeepCollectionEquality()
                .equals(other._annotations, _annotations) &&
            (identical(other.priority, priority) ||
                other.priority == priority) &&
            (identical(other.wait, wait) || other.wait == wait) &&
            const DeepCollectionEquality().equals(other._depends, _depends) &&
            const DeepCollectionEquality().equals(other._uda, _uda));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType,
      uuid,
      status,
      title,
      active,
      modified,
      due,
      project,
      const DeepCollectionEquality().hash(tags),
      const DeepCollectionEquality().hash(_annotations),
      priority,
      wait,
      const DeepCollectionEquality().hash(_depends),
      const DeepCollectionEquality().hash(_uda));

  /// Create a copy of Task
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$TaskImplCopyWith<_$TaskImpl> get copyWith =>
      __$$TaskImplCopyWithImpl<_$TaskImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(
            UuidValue uuid,
            TaskStatus status,
            String title,
            bool active,
            DateTime? modified,
            DateTime? due,
            String? project,
            Uint32List tags,
            List<Annotation> annotations,
            TaskPriority? priority,
            DateTime? wait,
            List<UuidValue> depends,
            Map<String, String> uda)
        raw,
  }) {
    return raw(uuid, status, title, active, modified, due, project, tags,
        annotations, priority, wait, depends, uda);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(
            UuidValue uuid,
            TaskStatus status,
            String title,
            bool active,
            DateTime? modified,
            DateTime? due,
            String? project,
            Uint32List tags,
            List<Annotation> annotations,
            TaskPriority? priority,
            DateTime? wait,
            List<UuidValue> depends,
            Map<String, String> uda)?
        raw,
  }) {
    return raw?.call(uuid, status, title, active, modified, due, project, tags,
        annotations, priority, wait, depends, uda);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(
            UuidValue uuid,
            TaskStatus status,
            String title,
            bool active,
            DateTime? modified,
            DateTime? due,
            String? project,
            Uint32List tags,
            List<Annotation> annotations,
            TaskPriority? priority,
            DateTime? wait,
            List<UuidValue> depends,
            Map<String, String> uda)?
        raw,
    required TResult orElse(),
  }) {
    if (raw != null) {
      return raw(uuid, status, title, active, modified, due, project, tags,
          annotations, priority, wait, depends, uda);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(_Task value) raw,
  }) {
    return raw(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(_Task value)? raw,
  }) {
    return raw?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(_Task value)? raw,
    required TResult orElse(),
  }) {
    if (raw != null) {
      return raw(this);
    }
    return orElse();
  }
}

abstract class _Task extends Task {
  const factory _Task(
      {required final UuidValue uuid,
      required final TaskStatus status,
      required final String title,
      required final bool active,
      final DateTime? modified,
      final DateTime? due,
      final String? project,
      required final Uint32List tags,
      required final List<Annotation> annotations,
      final TaskPriority? priority,
      final DateTime? wait,
      required final List<UuidValue> depends,
      required final Map<String, String> uda}) = _$TaskImpl;
  const _Task._() : super._();

  @override
  UuidValue get uuid;
  @override
  TaskStatus get status;
  @override
  String get title;
  @override
  bool get active;
  @override
  DateTime? get modified;
  @override
  DateTime? get due;
  @override
  String? get project;
  @override
  Uint32List get tags;
  @override
  List<Annotation> get annotations;
  @override
  TaskPriority? get priority;
  @override
  DateTime? get wait;
  @override
  List<UuidValue> get depends;
  @override
  Map<String, String> get uda;

  /// Create a copy of Task
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$TaskImplCopyWith<_$TaskImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
