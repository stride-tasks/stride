// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'event.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$PluginEvent {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Task task) taskCreate,
    required TResult Function(Task task) taskModify,
    required TResult Function(TaskQuery query) taskQuery,
    required TResult Function() taskSync,
    required TResult Function(NetworkRequestType ty, String host)
        networkRequest,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Task task)? taskCreate,
    TResult? Function(Task task)? taskModify,
    TResult? Function(TaskQuery query)? taskQuery,
    TResult? Function()? taskSync,
    TResult? Function(NetworkRequestType ty, String host)? networkRequest,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Task task)? taskCreate,
    TResult Function(Task task)? taskModify,
    TResult Function(TaskQuery query)? taskQuery,
    TResult Function()? taskSync,
    TResult Function(NetworkRequestType ty, String host)? networkRequest,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PluginEvent_TaskCreate value) taskCreate,
    required TResult Function(PluginEvent_TaskModify value) taskModify,
    required TResult Function(PluginEvent_TaskQuery value) taskQuery,
    required TResult Function(PluginEvent_TaskSync value) taskSync,
    required TResult Function(PluginEvent_NetworkRequest value) networkRequest,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PluginEvent_TaskCreate value)? taskCreate,
    TResult? Function(PluginEvent_TaskModify value)? taskModify,
    TResult? Function(PluginEvent_TaskQuery value)? taskQuery,
    TResult? Function(PluginEvent_TaskSync value)? taskSync,
    TResult? Function(PluginEvent_NetworkRequest value)? networkRequest,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PluginEvent_TaskCreate value)? taskCreate,
    TResult Function(PluginEvent_TaskModify value)? taskModify,
    TResult Function(PluginEvent_TaskQuery value)? taskQuery,
    TResult Function(PluginEvent_TaskSync value)? taskSync,
    TResult Function(PluginEvent_NetworkRequest value)? networkRequest,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $PluginEventCopyWith<$Res> {
  factory $PluginEventCopyWith(
          PluginEvent value, $Res Function(PluginEvent) then) =
      _$PluginEventCopyWithImpl<$Res, PluginEvent>;
}

/// @nodoc
class _$PluginEventCopyWithImpl<$Res, $Val extends PluginEvent>
    implements $PluginEventCopyWith<$Res> {
  _$PluginEventCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of PluginEvent
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$PluginEvent_TaskCreateImplCopyWith<$Res> {
  factory _$$PluginEvent_TaskCreateImplCopyWith(
          _$PluginEvent_TaskCreateImpl value,
          $Res Function(_$PluginEvent_TaskCreateImpl) then) =
      __$$PluginEvent_TaskCreateImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Task task});

  $TaskCopyWith<$Res> get task;
}

/// @nodoc
class __$$PluginEvent_TaskCreateImplCopyWithImpl<$Res>
    extends _$PluginEventCopyWithImpl<$Res, _$PluginEvent_TaskCreateImpl>
    implements _$$PluginEvent_TaskCreateImplCopyWith<$Res> {
  __$$PluginEvent_TaskCreateImplCopyWithImpl(
      _$PluginEvent_TaskCreateImpl _value,
      $Res Function(_$PluginEvent_TaskCreateImpl) _then)
      : super(_value, _then);

  /// Create a copy of PluginEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? task = null,
  }) {
    return _then(_$PluginEvent_TaskCreateImpl(
      task: null == task
          ? _value.task
          : task // ignore: cast_nullable_to_non_nullable
              as Task,
    ));
  }

  /// Create a copy of PluginEvent
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $TaskCopyWith<$Res> get task {
    return $TaskCopyWith<$Res>(_value.task, (value) {
      return _then(_value.copyWith(task: value));
    });
  }
}

/// @nodoc

class _$PluginEvent_TaskCreateImpl extends PluginEvent_TaskCreate {
  const _$PluginEvent_TaskCreateImpl({required this.task}) : super._();

  @override
  final Task task;

  @override
  String toString() {
    return 'PluginEvent.taskCreate(task: $task)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PluginEvent_TaskCreateImpl &&
            (identical(other.task, task) || other.task == task));
  }

  @override
  int get hashCode => Object.hash(runtimeType, task);

  /// Create a copy of PluginEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$PluginEvent_TaskCreateImplCopyWith<_$PluginEvent_TaskCreateImpl>
      get copyWith => __$$PluginEvent_TaskCreateImplCopyWithImpl<
          _$PluginEvent_TaskCreateImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Task task) taskCreate,
    required TResult Function(Task task) taskModify,
    required TResult Function(TaskQuery query) taskQuery,
    required TResult Function() taskSync,
    required TResult Function(NetworkRequestType ty, String host)
        networkRequest,
  }) {
    return taskCreate(task);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Task task)? taskCreate,
    TResult? Function(Task task)? taskModify,
    TResult? Function(TaskQuery query)? taskQuery,
    TResult? Function()? taskSync,
    TResult? Function(NetworkRequestType ty, String host)? networkRequest,
  }) {
    return taskCreate?.call(task);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Task task)? taskCreate,
    TResult Function(Task task)? taskModify,
    TResult Function(TaskQuery query)? taskQuery,
    TResult Function()? taskSync,
    TResult Function(NetworkRequestType ty, String host)? networkRequest,
    required TResult orElse(),
  }) {
    if (taskCreate != null) {
      return taskCreate(task);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PluginEvent_TaskCreate value) taskCreate,
    required TResult Function(PluginEvent_TaskModify value) taskModify,
    required TResult Function(PluginEvent_TaskQuery value) taskQuery,
    required TResult Function(PluginEvent_TaskSync value) taskSync,
    required TResult Function(PluginEvent_NetworkRequest value) networkRequest,
  }) {
    return taskCreate(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PluginEvent_TaskCreate value)? taskCreate,
    TResult? Function(PluginEvent_TaskModify value)? taskModify,
    TResult? Function(PluginEvent_TaskQuery value)? taskQuery,
    TResult? Function(PluginEvent_TaskSync value)? taskSync,
    TResult? Function(PluginEvent_NetworkRequest value)? networkRequest,
  }) {
    return taskCreate?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PluginEvent_TaskCreate value)? taskCreate,
    TResult Function(PluginEvent_TaskModify value)? taskModify,
    TResult Function(PluginEvent_TaskQuery value)? taskQuery,
    TResult Function(PluginEvent_TaskSync value)? taskSync,
    TResult Function(PluginEvent_NetworkRequest value)? networkRequest,
    required TResult orElse(),
  }) {
    if (taskCreate != null) {
      return taskCreate(this);
    }
    return orElse();
  }
}

abstract class PluginEvent_TaskCreate extends PluginEvent {
  const factory PluginEvent_TaskCreate({required final Task task}) =
      _$PluginEvent_TaskCreateImpl;
  const PluginEvent_TaskCreate._() : super._();

  Task get task;

  /// Create a copy of PluginEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$PluginEvent_TaskCreateImplCopyWith<_$PluginEvent_TaskCreateImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$PluginEvent_TaskModifyImplCopyWith<$Res> {
  factory _$$PluginEvent_TaskModifyImplCopyWith(
          _$PluginEvent_TaskModifyImpl value,
          $Res Function(_$PluginEvent_TaskModifyImpl) then) =
      __$$PluginEvent_TaskModifyImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Task task});

  $TaskCopyWith<$Res> get task;
}

/// @nodoc
class __$$PluginEvent_TaskModifyImplCopyWithImpl<$Res>
    extends _$PluginEventCopyWithImpl<$Res, _$PluginEvent_TaskModifyImpl>
    implements _$$PluginEvent_TaskModifyImplCopyWith<$Res> {
  __$$PluginEvent_TaskModifyImplCopyWithImpl(
      _$PluginEvent_TaskModifyImpl _value,
      $Res Function(_$PluginEvent_TaskModifyImpl) _then)
      : super(_value, _then);

  /// Create a copy of PluginEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? task = null,
  }) {
    return _then(_$PluginEvent_TaskModifyImpl(
      task: null == task
          ? _value.task
          : task // ignore: cast_nullable_to_non_nullable
              as Task,
    ));
  }

  /// Create a copy of PluginEvent
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $TaskCopyWith<$Res> get task {
    return $TaskCopyWith<$Res>(_value.task, (value) {
      return _then(_value.copyWith(task: value));
    });
  }
}

/// @nodoc

class _$PluginEvent_TaskModifyImpl extends PluginEvent_TaskModify {
  const _$PluginEvent_TaskModifyImpl({required this.task}) : super._();

  @override
  final Task task;

  @override
  String toString() {
    return 'PluginEvent.taskModify(task: $task)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PluginEvent_TaskModifyImpl &&
            (identical(other.task, task) || other.task == task));
  }

  @override
  int get hashCode => Object.hash(runtimeType, task);

  /// Create a copy of PluginEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$PluginEvent_TaskModifyImplCopyWith<_$PluginEvent_TaskModifyImpl>
      get copyWith => __$$PluginEvent_TaskModifyImplCopyWithImpl<
          _$PluginEvent_TaskModifyImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Task task) taskCreate,
    required TResult Function(Task task) taskModify,
    required TResult Function(TaskQuery query) taskQuery,
    required TResult Function() taskSync,
    required TResult Function(NetworkRequestType ty, String host)
        networkRequest,
  }) {
    return taskModify(task);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Task task)? taskCreate,
    TResult? Function(Task task)? taskModify,
    TResult? Function(TaskQuery query)? taskQuery,
    TResult? Function()? taskSync,
    TResult? Function(NetworkRequestType ty, String host)? networkRequest,
  }) {
    return taskModify?.call(task);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Task task)? taskCreate,
    TResult Function(Task task)? taskModify,
    TResult Function(TaskQuery query)? taskQuery,
    TResult Function()? taskSync,
    TResult Function(NetworkRequestType ty, String host)? networkRequest,
    required TResult orElse(),
  }) {
    if (taskModify != null) {
      return taskModify(task);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PluginEvent_TaskCreate value) taskCreate,
    required TResult Function(PluginEvent_TaskModify value) taskModify,
    required TResult Function(PluginEvent_TaskQuery value) taskQuery,
    required TResult Function(PluginEvent_TaskSync value) taskSync,
    required TResult Function(PluginEvent_NetworkRequest value) networkRequest,
  }) {
    return taskModify(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PluginEvent_TaskCreate value)? taskCreate,
    TResult? Function(PluginEvent_TaskModify value)? taskModify,
    TResult? Function(PluginEvent_TaskQuery value)? taskQuery,
    TResult? Function(PluginEvent_TaskSync value)? taskSync,
    TResult? Function(PluginEvent_NetworkRequest value)? networkRequest,
  }) {
    return taskModify?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PluginEvent_TaskCreate value)? taskCreate,
    TResult Function(PluginEvent_TaskModify value)? taskModify,
    TResult Function(PluginEvent_TaskQuery value)? taskQuery,
    TResult Function(PluginEvent_TaskSync value)? taskSync,
    TResult Function(PluginEvent_NetworkRequest value)? networkRequest,
    required TResult orElse(),
  }) {
    if (taskModify != null) {
      return taskModify(this);
    }
    return orElse();
  }
}

abstract class PluginEvent_TaskModify extends PluginEvent {
  const factory PluginEvent_TaskModify({required final Task task}) =
      _$PluginEvent_TaskModifyImpl;
  const PluginEvent_TaskModify._() : super._();

  Task get task;

  /// Create a copy of PluginEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$PluginEvent_TaskModifyImplCopyWith<_$PluginEvent_TaskModifyImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$PluginEvent_TaskQueryImplCopyWith<$Res> {
  factory _$$PluginEvent_TaskQueryImplCopyWith(
          _$PluginEvent_TaskQueryImpl value,
          $Res Function(_$PluginEvent_TaskQueryImpl) then) =
      __$$PluginEvent_TaskQueryImplCopyWithImpl<$Res>;
  @useResult
  $Res call({TaskQuery query});

  $TaskQueryCopyWith<$Res> get query;
}

/// @nodoc
class __$$PluginEvent_TaskQueryImplCopyWithImpl<$Res>
    extends _$PluginEventCopyWithImpl<$Res, _$PluginEvent_TaskQueryImpl>
    implements _$$PluginEvent_TaskQueryImplCopyWith<$Res> {
  __$$PluginEvent_TaskQueryImplCopyWithImpl(_$PluginEvent_TaskQueryImpl _value,
      $Res Function(_$PluginEvent_TaskQueryImpl) _then)
      : super(_value, _then);

  /// Create a copy of PluginEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? query = null,
  }) {
    return _then(_$PluginEvent_TaskQueryImpl(
      query: null == query
          ? _value.query
          : query // ignore: cast_nullable_to_non_nullable
              as TaskQuery,
    ));
  }

  /// Create a copy of PluginEvent
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $TaskQueryCopyWith<$Res> get query {
    return $TaskQueryCopyWith<$Res>(_value.query, (value) {
      return _then(_value.copyWith(query: value));
    });
  }
}

/// @nodoc

class _$PluginEvent_TaskQueryImpl extends PluginEvent_TaskQuery {
  const _$PluginEvent_TaskQueryImpl({required this.query}) : super._();

  @override
  final TaskQuery query;

  @override
  String toString() {
    return 'PluginEvent.taskQuery(query: $query)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PluginEvent_TaskQueryImpl &&
            (identical(other.query, query) || other.query == query));
  }

  @override
  int get hashCode => Object.hash(runtimeType, query);

  /// Create a copy of PluginEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$PluginEvent_TaskQueryImplCopyWith<_$PluginEvent_TaskQueryImpl>
      get copyWith => __$$PluginEvent_TaskQueryImplCopyWithImpl<
          _$PluginEvent_TaskQueryImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Task task) taskCreate,
    required TResult Function(Task task) taskModify,
    required TResult Function(TaskQuery query) taskQuery,
    required TResult Function() taskSync,
    required TResult Function(NetworkRequestType ty, String host)
        networkRequest,
  }) {
    return taskQuery(query);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Task task)? taskCreate,
    TResult? Function(Task task)? taskModify,
    TResult? Function(TaskQuery query)? taskQuery,
    TResult? Function()? taskSync,
    TResult? Function(NetworkRequestType ty, String host)? networkRequest,
  }) {
    return taskQuery?.call(query);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Task task)? taskCreate,
    TResult Function(Task task)? taskModify,
    TResult Function(TaskQuery query)? taskQuery,
    TResult Function()? taskSync,
    TResult Function(NetworkRequestType ty, String host)? networkRequest,
    required TResult orElse(),
  }) {
    if (taskQuery != null) {
      return taskQuery(query);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PluginEvent_TaskCreate value) taskCreate,
    required TResult Function(PluginEvent_TaskModify value) taskModify,
    required TResult Function(PluginEvent_TaskQuery value) taskQuery,
    required TResult Function(PluginEvent_TaskSync value) taskSync,
    required TResult Function(PluginEvent_NetworkRequest value) networkRequest,
  }) {
    return taskQuery(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PluginEvent_TaskCreate value)? taskCreate,
    TResult? Function(PluginEvent_TaskModify value)? taskModify,
    TResult? Function(PluginEvent_TaskQuery value)? taskQuery,
    TResult? Function(PluginEvent_TaskSync value)? taskSync,
    TResult? Function(PluginEvent_NetworkRequest value)? networkRequest,
  }) {
    return taskQuery?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PluginEvent_TaskCreate value)? taskCreate,
    TResult Function(PluginEvent_TaskModify value)? taskModify,
    TResult Function(PluginEvent_TaskQuery value)? taskQuery,
    TResult Function(PluginEvent_TaskSync value)? taskSync,
    TResult Function(PluginEvent_NetworkRequest value)? networkRequest,
    required TResult orElse(),
  }) {
    if (taskQuery != null) {
      return taskQuery(this);
    }
    return orElse();
  }
}

abstract class PluginEvent_TaskQuery extends PluginEvent {
  const factory PluginEvent_TaskQuery({required final TaskQuery query}) =
      _$PluginEvent_TaskQueryImpl;
  const PluginEvent_TaskQuery._() : super._();

  TaskQuery get query;

  /// Create a copy of PluginEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$PluginEvent_TaskQueryImplCopyWith<_$PluginEvent_TaskQueryImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$PluginEvent_TaskSyncImplCopyWith<$Res> {
  factory _$$PluginEvent_TaskSyncImplCopyWith(_$PluginEvent_TaskSyncImpl value,
          $Res Function(_$PluginEvent_TaskSyncImpl) then) =
      __$$PluginEvent_TaskSyncImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$PluginEvent_TaskSyncImplCopyWithImpl<$Res>
    extends _$PluginEventCopyWithImpl<$Res, _$PluginEvent_TaskSyncImpl>
    implements _$$PluginEvent_TaskSyncImplCopyWith<$Res> {
  __$$PluginEvent_TaskSyncImplCopyWithImpl(_$PluginEvent_TaskSyncImpl _value,
      $Res Function(_$PluginEvent_TaskSyncImpl) _then)
      : super(_value, _then);

  /// Create a copy of PluginEvent
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$PluginEvent_TaskSyncImpl extends PluginEvent_TaskSync {
  const _$PluginEvent_TaskSyncImpl() : super._();

  @override
  String toString() {
    return 'PluginEvent.taskSync()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PluginEvent_TaskSyncImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Task task) taskCreate,
    required TResult Function(Task task) taskModify,
    required TResult Function(TaskQuery query) taskQuery,
    required TResult Function() taskSync,
    required TResult Function(NetworkRequestType ty, String host)
        networkRequest,
  }) {
    return taskSync();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Task task)? taskCreate,
    TResult? Function(Task task)? taskModify,
    TResult? Function(TaskQuery query)? taskQuery,
    TResult? Function()? taskSync,
    TResult? Function(NetworkRequestType ty, String host)? networkRequest,
  }) {
    return taskSync?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Task task)? taskCreate,
    TResult Function(Task task)? taskModify,
    TResult Function(TaskQuery query)? taskQuery,
    TResult Function()? taskSync,
    TResult Function(NetworkRequestType ty, String host)? networkRequest,
    required TResult orElse(),
  }) {
    if (taskSync != null) {
      return taskSync();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PluginEvent_TaskCreate value) taskCreate,
    required TResult Function(PluginEvent_TaskModify value) taskModify,
    required TResult Function(PluginEvent_TaskQuery value) taskQuery,
    required TResult Function(PluginEvent_TaskSync value) taskSync,
    required TResult Function(PluginEvent_NetworkRequest value) networkRequest,
  }) {
    return taskSync(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PluginEvent_TaskCreate value)? taskCreate,
    TResult? Function(PluginEvent_TaskModify value)? taskModify,
    TResult? Function(PluginEvent_TaskQuery value)? taskQuery,
    TResult? Function(PluginEvent_TaskSync value)? taskSync,
    TResult? Function(PluginEvent_NetworkRequest value)? networkRequest,
  }) {
    return taskSync?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PluginEvent_TaskCreate value)? taskCreate,
    TResult Function(PluginEvent_TaskModify value)? taskModify,
    TResult Function(PluginEvent_TaskQuery value)? taskQuery,
    TResult Function(PluginEvent_TaskSync value)? taskSync,
    TResult Function(PluginEvent_NetworkRequest value)? networkRequest,
    required TResult orElse(),
  }) {
    if (taskSync != null) {
      return taskSync(this);
    }
    return orElse();
  }
}

abstract class PluginEvent_TaskSync extends PluginEvent {
  const factory PluginEvent_TaskSync() = _$PluginEvent_TaskSyncImpl;
  const PluginEvent_TaskSync._() : super._();
}

/// @nodoc
abstract class _$$PluginEvent_NetworkRequestImplCopyWith<$Res> {
  factory _$$PluginEvent_NetworkRequestImplCopyWith(
          _$PluginEvent_NetworkRequestImpl value,
          $Res Function(_$PluginEvent_NetworkRequestImpl) then) =
      __$$PluginEvent_NetworkRequestImplCopyWithImpl<$Res>;
  @useResult
  $Res call({NetworkRequestType ty, String host});
}

/// @nodoc
class __$$PluginEvent_NetworkRequestImplCopyWithImpl<$Res>
    extends _$PluginEventCopyWithImpl<$Res, _$PluginEvent_NetworkRequestImpl>
    implements _$$PluginEvent_NetworkRequestImplCopyWith<$Res> {
  __$$PluginEvent_NetworkRequestImplCopyWithImpl(
      _$PluginEvent_NetworkRequestImpl _value,
      $Res Function(_$PluginEvent_NetworkRequestImpl) _then)
      : super(_value, _then);

  /// Create a copy of PluginEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? ty = null,
    Object? host = null,
  }) {
    return _then(_$PluginEvent_NetworkRequestImpl(
      ty: null == ty
          ? _value.ty
          : ty // ignore: cast_nullable_to_non_nullable
              as NetworkRequestType,
      host: null == host
          ? _value.host
          : host // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$PluginEvent_NetworkRequestImpl extends PluginEvent_NetworkRequest {
  const _$PluginEvent_NetworkRequestImpl({required this.ty, required this.host})
      : super._();

  @override
  final NetworkRequestType ty;
  @override
  final String host;

  @override
  String toString() {
    return 'PluginEvent.networkRequest(ty: $ty, host: $host)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PluginEvent_NetworkRequestImpl &&
            (identical(other.ty, ty) || other.ty == ty) &&
            (identical(other.host, host) || other.host == host));
  }

  @override
  int get hashCode => Object.hash(runtimeType, ty, host);

  /// Create a copy of PluginEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$PluginEvent_NetworkRequestImplCopyWith<_$PluginEvent_NetworkRequestImpl>
      get copyWith => __$$PluginEvent_NetworkRequestImplCopyWithImpl<
          _$PluginEvent_NetworkRequestImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Task task) taskCreate,
    required TResult Function(Task task) taskModify,
    required TResult Function(TaskQuery query) taskQuery,
    required TResult Function() taskSync,
    required TResult Function(NetworkRequestType ty, String host)
        networkRequest,
  }) {
    return networkRequest(ty, host);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Task task)? taskCreate,
    TResult? Function(Task task)? taskModify,
    TResult? Function(TaskQuery query)? taskQuery,
    TResult? Function()? taskSync,
    TResult? Function(NetworkRequestType ty, String host)? networkRequest,
  }) {
    return networkRequest?.call(ty, host);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Task task)? taskCreate,
    TResult Function(Task task)? taskModify,
    TResult Function(TaskQuery query)? taskQuery,
    TResult Function()? taskSync,
    TResult Function(NetworkRequestType ty, String host)? networkRequest,
    required TResult orElse(),
  }) {
    if (networkRequest != null) {
      return networkRequest(ty, host);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PluginEvent_TaskCreate value) taskCreate,
    required TResult Function(PluginEvent_TaskModify value) taskModify,
    required TResult Function(PluginEvent_TaskQuery value) taskQuery,
    required TResult Function(PluginEvent_TaskSync value) taskSync,
    required TResult Function(PluginEvent_NetworkRequest value) networkRequest,
  }) {
    return networkRequest(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PluginEvent_TaskCreate value)? taskCreate,
    TResult? Function(PluginEvent_TaskModify value)? taskModify,
    TResult? Function(PluginEvent_TaskQuery value)? taskQuery,
    TResult? Function(PluginEvent_TaskSync value)? taskSync,
    TResult? Function(PluginEvent_NetworkRequest value)? networkRequest,
  }) {
    return networkRequest?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PluginEvent_TaskCreate value)? taskCreate,
    TResult Function(PluginEvent_TaskModify value)? taskModify,
    TResult Function(PluginEvent_TaskQuery value)? taskQuery,
    TResult Function(PluginEvent_TaskSync value)? taskSync,
    TResult Function(PluginEvent_NetworkRequest value)? networkRequest,
    required TResult orElse(),
  }) {
    if (networkRequest != null) {
      return networkRequest(this);
    }
    return orElse();
  }
}

abstract class PluginEvent_NetworkRequest extends PluginEvent {
  const factory PluginEvent_NetworkRequest(
      {required final NetworkRequestType ty,
      required final String host}) = _$PluginEvent_NetworkRequestImpl;
  const PluginEvent_NetworkRequest._() : super._();

  NetworkRequestType get ty;
  String get host;

  /// Create a copy of PluginEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$PluginEvent_NetworkRequestImplCopyWith<_$PluginEvent_NetworkRequestImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$TaskQuery {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(UuidValue uuid) uuid,
    required TResult Function(String title, Set<TaskStatus> status, int? limit)
        title,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(UuidValue uuid)? uuid,
    TResult? Function(String title, Set<TaskStatus> status, int? limit)? title,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(UuidValue uuid)? uuid,
    TResult Function(String title, Set<TaskStatus> status, int? limit)? title,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(TaskQuery_Uuid value) uuid,
    required TResult Function(TaskQuery_Title value) title,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(TaskQuery_Uuid value)? uuid,
    TResult? Function(TaskQuery_Title value)? title,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(TaskQuery_Uuid value)? uuid,
    TResult Function(TaskQuery_Title value)? title,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $TaskQueryCopyWith<$Res> {
  factory $TaskQueryCopyWith(TaskQuery value, $Res Function(TaskQuery) then) =
      _$TaskQueryCopyWithImpl<$Res, TaskQuery>;
}

/// @nodoc
class _$TaskQueryCopyWithImpl<$Res, $Val extends TaskQuery>
    implements $TaskQueryCopyWith<$Res> {
  _$TaskQueryCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of TaskQuery
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$TaskQuery_UuidImplCopyWith<$Res> {
  factory _$$TaskQuery_UuidImplCopyWith(_$TaskQuery_UuidImpl value,
          $Res Function(_$TaskQuery_UuidImpl) then) =
      __$$TaskQuery_UuidImplCopyWithImpl<$Res>;
  @useResult
  $Res call({UuidValue uuid});
}

/// @nodoc
class __$$TaskQuery_UuidImplCopyWithImpl<$Res>
    extends _$TaskQueryCopyWithImpl<$Res, _$TaskQuery_UuidImpl>
    implements _$$TaskQuery_UuidImplCopyWith<$Res> {
  __$$TaskQuery_UuidImplCopyWithImpl(
      _$TaskQuery_UuidImpl _value, $Res Function(_$TaskQuery_UuidImpl) _then)
      : super(_value, _then);

  /// Create a copy of TaskQuery
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? uuid = null,
  }) {
    return _then(_$TaskQuery_UuidImpl(
      uuid: null == uuid
          ? _value.uuid
          : uuid // ignore: cast_nullable_to_non_nullable
              as UuidValue,
    ));
  }
}

/// @nodoc

class _$TaskQuery_UuidImpl extends TaskQuery_Uuid {
  const _$TaskQuery_UuidImpl({required this.uuid}) : super._();

  @override
  final UuidValue uuid;

  @override
  String toString() {
    return 'TaskQuery.uuid(uuid: $uuid)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$TaskQuery_UuidImpl &&
            (identical(other.uuid, uuid) || other.uuid == uuid));
  }

  @override
  int get hashCode => Object.hash(runtimeType, uuid);

  /// Create a copy of TaskQuery
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$TaskQuery_UuidImplCopyWith<_$TaskQuery_UuidImpl> get copyWith =>
      __$$TaskQuery_UuidImplCopyWithImpl<_$TaskQuery_UuidImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(UuidValue uuid) uuid,
    required TResult Function(String title, Set<TaskStatus> status, int? limit)
        title,
  }) {
    return uuid(this.uuid);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(UuidValue uuid)? uuid,
    TResult? Function(String title, Set<TaskStatus> status, int? limit)? title,
  }) {
    return uuid?.call(this.uuid);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(UuidValue uuid)? uuid,
    TResult Function(String title, Set<TaskStatus> status, int? limit)? title,
    required TResult orElse(),
  }) {
    if (uuid != null) {
      return uuid(this.uuid);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(TaskQuery_Uuid value) uuid,
    required TResult Function(TaskQuery_Title value) title,
  }) {
    return uuid(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(TaskQuery_Uuid value)? uuid,
    TResult? Function(TaskQuery_Title value)? title,
  }) {
    return uuid?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(TaskQuery_Uuid value)? uuid,
    TResult Function(TaskQuery_Title value)? title,
    required TResult orElse(),
  }) {
    if (uuid != null) {
      return uuid(this);
    }
    return orElse();
  }
}

abstract class TaskQuery_Uuid extends TaskQuery {
  const factory TaskQuery_Uuid({required final UuidValue uuid}) =
      _$TaskQuery_UuidImpl;
  const TaskQuery_Uuid._() : super._();

  UuidValue get uuid;

  /// Create a copy of TaskQuery
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$TaskQuery_UuidImplCopyWith<_$TaskQuery_UuidImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$TaskQuery_TitleImplCopyWith<$Res> {
  factory _$$TaskQuery_TitleImplCopyWith(_$TaskQuery_TitleImpl value,
          $Res Function(_$TaskQuery_TitleImpl) then) =
      __$$TaskQuery_TitleImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String title, Set<TaskStatus> status, int? limit});
}

/// @nodoc
class __$$TaskQuery_TitleImplCopyWithImpl<$Res>
    extends _$TaskQueryCopyWithImpl<$Res, _$TaskQuery_TitleImpl>
    implements _$$TaskQuery_TitleImplCopyWith<$Res> {
  __$$TaskQuery_TitleImplCopyWithImpl(
      _$TaskQuery_TitleImpl _value, $Res Function(_$TaskQuery_TitleImpl) _then)
      : super(_value, _then);

  /// Create a copy of TaskQuery
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? title = null,
    Object? status = null,
    Object? limit = freezed,
  }) {
    return _then(_$TaskQuery_TitleImpl(
      title: null == title
          ? _value.title
          : title // ignore: cast_nullable_to_non_nullable
              as String,
      status: null == status
          ? _value._status
          : status // ignore: cast_nullable_to_non_nullable
              as Set<TaskStatus>,
      limit: freezed == limit
          ? _value.limit
          : limit // ignore: cast_nullable_to_non_nullable
              as int?,
    ));
  }
}

/// @nodoc

class _$TaskQuery_TitleImpl extends TaskQuery_Title {
  const _$TaskQuery_TitleImpl(
      {required this.title, required final Set<TaskStatus> status, this.limit})
      : _status = status,
        super._();

  @override
  final String title;
  final Set<TaskStatus> _status;
  @override
  Set<TaskStatus> get status {
    if (_status is EqualUnmodifiableSetView) return _status;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableSetView(_status);
  }

  @override
  final int? limit;

  @override
  String toString() {
    return 'TaskQuery.title(title: $title, status: $status, limit: $limit)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$TaskQuery_TitleImpl &&
            (identical(other.title, title) || other.title == title) &&
            const DeepCollectionEquality().equals(other._status, _status) &&
            (identical(other.limit, limit) || other.limit == limit));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, title, const DeepCollectionEquality().hash(_status), limit);

  /// Create a copy of TaskQuery
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$TaskQuery_TitleImplCopyWith<_$TaskQuery_TitleImpl> get copyWith =>
      __$$TaskQuery_TitleImplCopyWithImpl<_$TaskQuery_TitleImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(UuidValue uuid) uuid,
    required TResult Function(String title, Set<TaskStatus> status, int? limit)
        title,
  }) {
    return title(this.title, status, limit);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(UuidValue uuid)? uuid,
    TResult? Function(String title, Set<TaskStatus> status, int? limit)? title,
  }) {
    return title?.call(this.title, status, limit);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(UuidValue uuid)? uuid,
    TResult Function(String title, Set<TaskStatus> status, int? limit)? title,
    required TResult orElse(),
  }) {
    if (title != null) {
      return title(this.title, status, limit);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(TaskQuery_Uuid value) uuid,
    required TResult Function(TaskQuery_Title value) title,
  }) {
    return title(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(TaskQuery_Uuid value)? uuid,
    TResult? Function(TaskQuery_Title value)? title,
  }) {
    return title?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(TaskQuery_Uuid value)? uuid,
    TResult Function(TaskQuery_Title value)? title,
    required TResult orElse(),
  }) {
    if (title != null) {
      return title(this);
    }
    return orElse();
  }
}

abstract class TaskQuery_Title extends TaskQuery {
  const factory TaskQuery_Title(
      {required final String title,
      required final Set<TaskStatus> status,
      final int? limit}) = _$TaskQuery_TitleImpl;
  const TaskQuery_Title._() : super._();

  String get title;
  Set<TaskStatus> get status;
  int? get limit;

  /// Create a copy of TaskQuery
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$TaskQuery_TitleImplCopyWith<_$TaskQuery_TitleImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
