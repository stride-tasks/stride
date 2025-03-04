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
    required TResult Function() taskSync,
    required TResult Function(NetworkRequestType ty, String host)
        networkRequest,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Task task)? taskCreate,
    TResult? Function(Task task)? taskModify,
    TResult? Function()? taskSync,
    TResult? Function(NetworkRequestType ty, String host)? networkRequest,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Task task)? taskCreate,
    TResult Function(Task task)? taskModify,
    TResult Function()? taskSync,
    TResult Function(NetworkRequestType ty, String host)? networkRequest,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PluginEvent_TaskCreate value) taskCreate,
    required TResult Function(PluginEvent_TaskModify value) taskModify,
    required TResult Function(PluginEvent_TaskSync value) taskSync,
    required TResult Function(PluginEvent_NetworkRequest value) networkRequest,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PluginEvent_TaskCreate value)? taskCreate,
    TResult? Function(PluginEvent_TaskModify value)? taskModify,
    TResult? Function(PluginEvent_TaskSync value)? taskSync,
    TResult? Function(PluginEvent_NetworkRequest value)? networkRequest,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PluginEvent_TaskCreate value)? taskCreate,
    TResult Function(PluginEvent_TaskModify value)? taskModify,
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
