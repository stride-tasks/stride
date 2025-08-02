// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'manifest.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$PluginAction {

 String get pluginName;
/// Create a copy of PluginAction
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PluginActionCopyWith<PluginAction> get copyWith => _$PluginActionCopyWithImpl<PluginAction>(this as PluginAction, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PluginAction&&(identical(other.pluginName, pluginName) || other.pluginName == pluginName));
}


@override
int get hashCode => Object.hash(runtimeType,pluginName);

@override
String toString() {
  return 'PluginAction(pluginName: $pluginName)';
}


}

/// @nodoc
abstract mixin class $PluginActionCopyWith<$Res>  {
  factory $PluginActionCopyWith(PluginAction value, $Res Function(PluginAction) _then) = _$PluginActionCopyWithImpl;
@useResult
$Res call({
 String pluginName
});




}
/// @nodoc
class _$PluginActionCopyWithImpl<$Res>
    implements $PluginActionCopyWith<$Res> {
  _$PluginActionCopyWithImpl(this._self, this._then);

  final PluginAction _self;
  final $Res Function(PluginAction) _then;

/// Create a copy of PluginAction
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? pluginName = null,}) {
  return _then(_self.copyWith(
pluginName: null == pluginName ? _self.pluginName : pluginName // ignore: cast_nullable_to_non_nullable
as String,
  ));
}

}


/// Adds pattern-matching-related methods to [PluginAction].
extension PluginActionPatterns on PluginAction {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( PluginAction_Event value)?  event,TResult Function( PluginAction_Disable value)?  disable,required TResult orElse(),}){
final _that = this;
switch (_that) {
case PluginAction_Event() when event != null:
return event(_that);case PluginAction_Disable() when disable != null:
return disable(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( PluginAction_Event value)  event,required TResult Function( PluginAction_Disable value)  disable,}){
final _that = this;
switch (_that) {
case PluginAction_Event():
return event(_that);case PluginAction_Disable():
return disable(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( PluginAction_Event value)?  event,TResult? Function( PluginAction_Disable value)?  disable,}){
final _that = this;
switch (_that) {
case PluginAction_Event() when event != null:
return event(_that);case PluginAction_Disable() when disable != null:
return disable(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( String pluginName,  PluginEvent event)?  event,TResult Function( String pluginName,  String reason)?  disable,required TResult orElse(),}) {final _that = this;
switch (_that) {
case PluginAction_Event() when event != null:
return event(_that.pluginName,_that.event);case PluginAction_Disable() when disable != null:
return disable(_that.pluginName,_that.reason);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( String pluginName,  PluginEvent event)  event,required TResult Function( String pluginName,  String reason)  disable,}) {final _that = this;
switch (_that) {
case PluginAction_Event():
return event(_that.pluginName,_that.event);case PluginAction_Disable():
return disable(_that.pluginName,_that.reason);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( String pluginName,  PluginEvent event)?  event,TResult? Function( String pluginName,  String reason)?  disable,}) {final _that = this;
switch (_that) {
case PluginAction_Event() when event != null:
return event(_that.pluginName,_that.event);case PluginAction_Disable() when disable != null:
return disable(_that.pluginName,_that.reason);case _:
  return null;

}
}

}

/// @nodoc


class PluginAction_Event extends PluginAction {
  const PluginAction_Event({required this.pluginName, required this.event}): super._();
  

@override final  String pluginName;
 final  PluginEvent event;

/// Create a copy of PluginAction
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PluginAction_EventCopyWith<PluginAction_Event> get copyWith => _$PluginAction_EventCopyWithImpl<PluginAction_Event>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PluginAction_Event&&(identical(other.pluginName, pluginName) || other.pluginName == pluginName)&&(identical(other.event, event) || other.event == event));
}


@override
int get hashCode => Object.hash(runtimeType,pluginName,event);

@override
String toString() {
  return 'PluginAction.event(pluginName: $pluginName, event: $event)';
}


}

/// @nodoc
abstract mixin class $PluginAction_EventCopyWith<$Res> implements $PluginActionCopyWith<$Res> {
  factory $PluginAction_EventCopyWith(PluginAction_Event value, $Res Function(PluginAction_Event) _then) = _$PluginAction_EventCopyWithImpl;
@override @useResult
$Res call({
 String pluginName, PluginEvent event
});


$PluginEventCopyWith<$Res> get event;

}
/// @nodoc
class _$PluginAction_EventCopyWithImpl<$Res>
    implements $PluginAction_EventCopyWith<$Res> {
  _$PluginAction_EventCopyWithImpl(this._self, this._then);

  final PluginAction_Event _self;
  final $Res Function(PluginAction_Event) _then;

/// Create a copy of PluginAction
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? pluginName = null,Object? event = null,}) {
  return _then(PluginAction_Event(
pluginName: null == pluginName ? _self.pluginName : pluginName // ignore: cast_nullable_to_non_nullable
as String,event: null == event ? _self.event : event // ignore: cast_nullable_to_non_nullable
as PluginEvent,
  ));
}

/// Create a copy of PluginAction
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$PluginEventCopyWith<$Res> get event {
  
  return $PluginEventCopyWith<$Res>(_self.event, (value) {
    return _then(_self.copyWith(event: value));
  });
}
}

/// @nodoc


class PluginAction_Disable extends PluginAction {
  const PluginAction_Disable({required this.pluginName, required this.reason}): super._();
  

@override final  String pluginName;
 final  String reason;

/// Create a copy of PluginAction
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PluginAction_DisableCopyWith<PluginAction_Disable> get copyWith => _$PluginAction_DisableCopyWithImpl<PluginAction_Disable>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PluginAction_Disable&&(identical(other.pluginName, pluginName) || other.pluginName == pluginName)&&(identical(other.reason, reason) || other.reason == reason));
}


@override
int get hashCode => Object.hash(runtimeType,pluginName,reason);

@override
String toString() {
  return 'PluginAction.disable(pluginName: $pluginName, reason: $reason)';
}


}

/// @nodoc
abstract mixin class $PluginAction_DisableCopyWith<$Res> implements $PluginActionCopyWith<$Res> {
  factory $PluginAction_DisableCopyWith(PluginAction_Disable value, $Res Function(PluginAction_Disable) _then) = _$PluginAction_DisableCopyWithImpl;
@override @useResult
$Res call({
 String pluginName, String reason
});




}
/// @nodoc
class _$PluginAction_DisableCopyWithImpl<$Res>
    implements $PluginAction_DisableCopyWith<$Res> {
  _$PluginAction_DisableCopyWithImpl(this._self, this._then);

  final PluginAction_Disable _self;
  final $Res Function(PluginAction_Disable) _then;

/// Create a copy of PluginAction
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? pluginName = null,Object? reason = null,}) {
  return _then(PluginAction_Disable(
pluginName: null == pluginName ? _self.pluginName : pluginName // ignore: cast_nullable_to_non_nullable
as String,reason: null == reason ? _self.reason : reason // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

// dart format on
