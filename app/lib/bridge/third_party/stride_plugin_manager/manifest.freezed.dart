// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'manifest.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$PluginAction {
  String get pluginName => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String pluginName, PluginEvent event) event,
    required TResult Function(String pluginName, String reason) disable,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String pluginName, PluginEvent event)? event,
    TResult? Function(String pluginName, String reason)? disable,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String pluginName, PluginEvent event)? event,
    TResult Function(String pluginName, String reason)? disable,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PluginAction_Event value) event,
    required TResult Function(PluginAction_Disable value) disable,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PluginAction_Event value)? event,
    TResult? Function(PluginAction_Disable value)? disable,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PluginAction_Event value)? event,
    TResult Function(PluginAction_Disable value)? disable,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;

  /// Create a copy of PluginAction
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $PluginActionCopyWith<PluginAction> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $PluginActionCopyWith<$Res> {
  factory $PluginActionCopyWith(
          PluginAction value, $Res Function(PluginAction) then) =
      _$PluginActionCopyWithImpl<$Res, PluginAction>;
  @useResult
  $Res call({String pluginName});
}

/// @nodoc
class _$PluginActionCopyWithImpl<$Res, $Val extends PluginAction>
    implements $PluginActionCopyWith<$Res> {
  _$PluginActionCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of PluginAction
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? pluginName = null,
  }) {
    return _then(_value.copyWith(
      pluginName: null == pluginName
          ? _value.pluginName
          : pluginName // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$PluginAction_EventImplCopyWith<$Res>
    implements $PluginActionCopyWith<$Res> {
  factory _$$PluginAction_EventImplCopyWith(_$PluginAction_EventImpl value,
          $Res Function(_$PluginAction_EventImpl) then) =
      __$$PluginAction_EventImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String pluginName, PluginEvent event});

  $PluginEventCopyWith<$Res> get event;
}

/// @nodoc
class __$$PluginAction_EventImplCopyWithImpl<$Res>
    extends _$PluginActionCopyWithImpl<$Res, _$PluginAction_EventImpl>
    implements _$$PluginAction_EventImplCopyWith<$Res> {
  __$$PluginAction_EventImplCopyWithImpl(_$PluginAction_EventImpl _value,
      $Res Function(_$PluginAction_EventImpl) _then)
      : super(_value, _then);

  /// Create a copy of PluginAction
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? pluginName = null,
    Object? event = null,
  }) {
    return _then(_$PluginAction_EventImpl(
      pluginName: null == pluginName
          ? _value.pluginName
          : pluginName // ignore: cast_nullable_to_non_nullable
              as String,
      event: null == event
          ? _value.event
          : event // ignore: cast_nullable_to_non_nullable
              as PluginEvent,
    ));
  }

  /// Create a copy of PluginAction
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $PluginEventCopyWith<$Res> get event {
    return $PluginEventCopyWith<$Res>(_value.event, (value) {
      return _then(_value.copyWith(event: value));
    });
  }
}

/// @nodoc

class _$PluginAction_EventImpl extends PluginAction_Event {
  const _$PluginAction_EventImpl(
      {required this.pluginName, required this.event})
      : super._();

  @override
  final String pluginName;
  @override
  final PluginEvent event;

  @override
  String toString() {
    return 'PluginAction.event(pluginName: $pluginName, event: $event)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PluginAction_EventImpl &&
            (identical(other.pluginName, pluginName) ||
                other.pluginName == pluginName) &&
            (identical(other.event, event) || other.event == event));
  }

  @override
  int get hashCode => Object.hash(runtimeType, pluginName, event);

  /// Create a copy of PluginAction
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$PluginAction_EventImplCopyWith<_$PluginAction_EventImpl> get copyWith =>
      __$$PluginAction_EventImplCopyWithImpl<_$PluginAction_EventImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String pluginName, PluginEvent event) event,
    required TResult Function(String pluginName, String reason) disable,
  }) {
    return event(pluginName, this.event);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String pluginName, PluginEvent event)? event,
    TResult? Function(String pluginName, String reason)? disable,
  }) {
    return event?.call(pluginName, this.event);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String pluginName, PluginEvent event)? event,
    TResult Function(String pluginName, String reason)? disable,
    required TResult orElse(),
  }) {
    if (event != null) {
      return event(pluginName, this.event);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PluginAction_Event value) event,
    required TResult Function(PluginAction_Disable value) disable,
  }) {
    return event(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PluginAction_Event value)? event,
    TResult? Function(PluginAction_Disable value)? disable,
  }) {
    return event?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PluginAction_Event value)? event,
    TResult Function(PluginAction_Disable value)? disable,
    required TResult orElse(),
  }) {
    if (event != null) {
      return event(this);
    }
    return orElse();
  }
}

abstract class PluginAction_Event extends PluginAction {
  const factory PluginAction_Event(
      {required final String pluginName,
      required final PluginEvent event}) = _$PluginAction_EventImpl;
  const PluginAction_Event._() : super._();

  @override
  String get pluginName;
  PluginEvent get event;

  /// Create a copy of PluginAction
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$PluginAction_EventImplCopyWith<_$PluginAction_EventImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$PluginAction_DisableImplCopyWith<$Res>
    implements $PluginActionCopyWith<$Res> {
  factory _$$PluginAction_DisableImplCopyWith(_$PluginAction_DisableImpl value,
          $Res Function(_$PluginAction_DisableImpl) then) =
      __$$PluginAction_DisableImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String pluginName, String reason});
}

/// @nodoc
class __$$PluginAction_DisableImplCopyWithImpl<$Res>
    extends _$PluginActionCopyWithImpl<$Res, _$PluginAction_DisableImpl>
    implements _$$PluginAction_DisableImplCopyWith<$Res> {
  __$$PluginAction_DisableImplCopyWithImpl(_$PluginAction_DisableImpl _value,
      $Res Function(_$PluginAction_DisableImpl) _then)
      : super(_value, _then);

  /// Create a copy of PluginAction
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? pluginName = null,
    Object? reason = null,
  }) {
    return _then(_$PluginAction_DisableImpl(
      pluginName: null == pluginName
          ? _value.pluginName
          : pluginName // ignore: cast_nullable_to_non_nullable
              as String,
      reason: null == reason
          ? _value.reason
          : reason // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$PluginAction_DisableImpl extends PluginAction_Disable {
  const _$PluginAction_DisableImpl(
      {required this.pluginName, required this.reason})
      : super._();

  @override
  final String pluginName;
  @override
  final String reason;

  @override
  String toString() {
    return 'PluginAction.disable(pluginName: $pluginName, reason: $reason)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PluginAction_DisableImpl &&
            (identical(other.pluginName, pluginName) ||
                other.pluginName == pluginName) &&
            (identical(other.reason, reason) || other.reason == reason));
  }

  @override
  int get hashCode => Object.hash(runtimeType, pluginName, reason);

  /// Create a copy of PluginAction
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$PluginAction_DisableImplCopyWith<_$PluginAction_DisableImpl>
      get copyWith =>
          __$$PluginAction_DisableImplCopyWithImpl<_$PluginAction_DisableImpl>(
              this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String pluginName, PluginEvent event) event,
    required TResult Function(String pluginName, String reason) disable,
  }) {
    return disable(pluginName, reason);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String pluginName, PluginEvent event)? event,
    TResult? Function(String pluginName, String reason)? disable,
  }) {
    return disable?.call(pluginName, reason);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String pluginName, PluginEvent event)? event,
    TResult Function(String pluginName, String reason)? disable,
    required TResult orElse(),
  }) {
    if (disable != null) {
      return disable(pluginName, reason);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PluginAction_Event value) event,
    required TResult Function(PluginAction_Disable value) disable,
  }) {
    return disable(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PluginAction_Event value)? event,
    TResult? Function(PluginAction_Disable value)? disable,
  }) {
    return disable?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PluginAction_Event value)? event,
    TResult Function(PluginAction_Disable value)? disable,
    required TResult orElse(),
  }) {
    if (disable != null) {
      return disable(this);
    }
    return orElse();
  }
}

abstract class PluginAction_Disable extends PluginAction {
  const factory PluginAction_Disable(
      {required final String pluginName,
      required final String reason}) = _$PluginAction_DisableImpl;
  const PluginAction_Disable._() : super._();

  @override
  String get pluginName;
  String get reason;

  /// Create a copy of PluginAction
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$PluginAction_DisableImplCopyWith<_$PluginAction_DisableImpl>
      get copyWith => throw _privateConstructorUsedError;
}
