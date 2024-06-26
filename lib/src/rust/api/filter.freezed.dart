// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'filter.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$Filter {
  UuidValue get uuid => throw _privateConstructorUsedError;
  String get name => throw _privateConstructorUsedError;
  Set<TaskStatus> get status => throw _privateConstructorUsedError;
  String get search => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $FilterCopyWith<Filter> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $FilterCopyWith<$Res> {
  factory $FilterCopyWith(Filter value, $Res Function(Filter) then) =
      _$FilterCopyWithImpl<$Res, Filter>;
  @useResult
  $Res call(
      {UuidValue uuid, String name, Set<TaskStatus> status, String search});
}

/// @nodoc
class _$FilterCopyWithImpl<$Res, $Val extends Filter>
    implements $FilterCopyWith<$Res> {
  _$FilterCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? uuid = null,
    Object? name = null,
    Object? status = null,
    Object? search = null,
  }) {
    return _then(_value.copyWith(
      uuid: null == uuid
          ? _value.uuid
          : uuid // ignore: cast_nullable_to_non_nullable
              as UuidValue,
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      status: null == status
          ? _value.status
          : status // ignore: cast_nullable_to_non_nullable
              as Set<TaskStatus>,
      search: null == search
          ? _value.search
          : search // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$FilterImplCopyWith<$Res> implements $FilterCopyWith<$Res> {
  factory _$$FilterImplCopyWith(
          _$FilterImpl value, $Res Function(_$FilterImpl) then) =
      __$$FilterImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {UuidValue uuid, String name, Set<TaskStatus> status, String search});
}

/// @nodoc
class __$$FilterImplCopyWithImpl<$Res>
    extends _$FilterCopyWithImpl<$Res, _$FilterImpl>
    implements _$$FilterImplCopyWith<$Res> {
  __$$FilterImplCopyWithImpl(
      _$FilterImpl _value, $Res Function(_$FilterImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? uuid = null,
    Object? name = null,
    Object? status = null,
    Object? search = null,
  }) {
    return _then(_$FilterImpl(
      uuid: null == uuid
          ? _value.uuid
          : uuid // ignore: cast_nullable_to_non_nullable
              as UuidValue,
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      status: null == status
          ? _value._status
          : status // ignore: cast_nullable_to_non_nullable
              as Set<TaskStatus>,
      search: null == search
          ? _value.search
          : search // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$FilterImpl implements _Filter {
  const _$FilterImpl(
      {required this.uuid,
      required this.name,
      required final Set<TaskStatus> status,
      required this.search})
      : _status = status;

  @override
  final UuidValue uuid;
  @override
  final String name;
  final Set<TaskStatus> _status;
  @override
  Set<TaskStatus> get status {
    if (_status is EqualUnmodifiableSetView) return _status;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableSetView(_status);
  }

  @override
  final String search;

  @override
  String toString() {
    return 'Filter(uuid: $uuid, name: $name, status: $status, search: $search)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$FilterImpl &&
            (identical(other.uuid, uuid) || other.uuid == uuid) &&
            (identical(other.name, name) || other.name == name) &&
            const DeepCollectionEquality().equals(other._status, _status) &&
            (identical(other.search, search) || other.search == search));
  }

  @override
  int get hashCode => Object.hash(runtimeType, uuid, name,
      const DeepCollectionEquality().hash(_status), search);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$FilterImplCopyWith<_$FilterImpl> get copyWith =>
      __$$FilterImplCopyWithImpl<_$FilterImpl>(this, _$identity);
}

abstract class _Filter implements Filter {
  const factory _Filter(
      {required final UuidValue uuid,
      required final String name,
      required final Set<TaskStatus> status,
      required final String search}) = _$FilterImpl;

  @override
  UuidValue get uuid;
  @override
  String get name;
  @override
  Set<TaskStatus> get status;
  @override
  String get search;
  @override
  @JsonKey(ignore: true)
  _$$FilterImplCopyWith<_$FilterImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$FilterSelection {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(UuidValue uuid) predefined,
    required TResult Function(Filter filter) temporary,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(UuidValue uuid)? predefined,
    TResult? Function(Filter filter)? temporary,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(UuidValue uuid)? predefined,
    TResult Function(Filter filter)? temporary,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(FilterSelection_Predefined value) predefined,
    required TResult Function(FilterSelection_Temporary value) temporary,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(FilterSelection_Predefined value)? predefined,
    TResult? Function(FilterSelection_Temporary value)? temporary,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(FilterSelection_Predefined value)? predefined,
    TResult Function(FilterSelection_Temporary value)? temporary,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $FilterSelectionCopyWith<$Res> {
  factory $FilterSelectionCopyWith(
          FilterSelection value, $Res Function(FilterSelection) then) =
      _$FilterSelectionCopyWithImpl<$Res, FilterSelection>;
}

/// @nodoc
class _$FilterSelectionCopyWithImpl<$Res, $Val extends FilterSelection>
    implements $FilterSelectionCopyWith<$Res> {
  _$FilterSelectionCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$FilterSelection_PredefinedImplCopyWith<$Res> {
  factory _$$FilterSelection_PredefinedImplCopyWith(
          _$FilterSelection_PredefinedImpl value,
          $Res Function(_$FilterSelection_PredefinedImpl) then) =
      __$$FilterSelection_PredefinedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({UuidValue uuid});
}

/// @nodoc
class __$$FilterSelection_PredefinedImplCopyWithImpl<$Res>
    extends _$FilterSelectionCopyWithImpl<$Res,
        _$FilterSelection_PredefinedImpl>
    implements _$$FilterSelection_PredefinedImplCopyWith<$Res> {
  __$$FilterSelection_PredefinedImplCopyWithImpl(
      _$FilterSelection_PredefinedImpl _value,
      $Res Function(_$FilterSelection_PredefinedImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? uuid = null,
  }) {
    return _then(_$FilterSelection_PredefinedImpl(
      uuid: null == uuid
          ? _value.uuid
          : uuid // ignore: cast_nullable_to_non_nullable
              as UuidValue,
    ));
  }
}

/// @nodoc

class _$FilterSelection_PredefinedImpl extends FilterSelection_Predefined {
  const _$FilterSelection_PredefinedImpl({required this.uuid}) : super._();

  @override
  final UuidValue uuid;

  @override
  String toString() {
    return 'FilterSelection.predefined(uuid: $uuid)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$FilterSelection_PredefinedImpl &&
            (identical(other.uuid, uuid) || other.uuid == uuid));
  }

  @override
  int get hashCode => Object.hash(runtimeType, uuid);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$FilterSelection_PredefinedImplCopyWith<_$FilterSelection_PredefinedImpl>
      get copyWith => __$$FilterSelection_PredefinedImplCopyWithImpl<
          _$FilterSelection_PredefinedImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(UuidValue uuid) predefined,
    required TResult Function(Filter filter) temporary,
  }) {
    return predefined(uuid);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(UuidValue uuid)? predefined,
    TResult? Function(Filter filter)? temporary,
  }) {
    return predefined?.call(uuid);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(UuidValue uuid)? predefined,
    TResult Function(Filter filter)? temporary,
    required TResult orElse(),
  }) {
    if (predefined != null) {
      return predefined(uuid);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(FilterSelection_Predefined value) predefined,
    required TResult Function(FilterSelection_Temporary value) temporary,
  }) {
    return predefined(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(FilterSelection_Predefined value)? predefined,
    TResult? Function(FilterSelection_Temporary value)? temporary,
  }) {
    return predefined?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(FilterSelection_Predefined value)? predefined,
    TResult Function(FilterSelection_Temporary value)? temporary,
    required TResult orElse(),
  }) {
    if (predefined != null) {
      return predefined(this);
    }
    return orElse();
  }
}

abstract class FilterSelection_Predefined extends FilterSelection {
  const factory FilterSelection_Predefined({required final UuidValue uuid}) =
      _$FilterSelection_PredefinedImpl;
  const FilterSelection_Predefined._() : super._();

  UuidValue get uuid;
  @JsonKey(ignore: true)
  _$$FilterSelection_PredefinedImplCopyWith<_$FilterSelection_PredefinedImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$FilterSelection_TemporaryImplCopyWith<$Res> {
  factory _$$FilterSelection_TemporaryImplCopyWith(
          _$FilterSelection_TemporaryImpl value,
          $Res Function(_$FilterSelection_TemporaryImpl) then) =
      __$$FilterSelection_TemporaryImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Filter filter});

  $FilterCopyWith<$Res> get filter;
}

/// @nodoc
class __$$FilterSelection_TemporaryImplCopyWithImpl<$Res>
    extends _$FilterSelectionCopyWithImpl<$Res, _$FilterSelection_TemporaryImpl>
    implements _$$FilterSelection_TemporaryImplCopyWith<$Res> {
  __$$FilterSelection_TemporaryImplCopyWithImpl(
      _$FilterSelection_TemporaryImpl _value,
      $Res Function(_$FilterSelection_TemporaryImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? filter = null,
  }) {
    return _then(_$FilterSelection_TemporaryImpl(
      filter: null == filter
          ? _value.filter
          : filter // ignore: cast_nullable_to_non_nullable
              as Filter,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $FilterCopyWith<$Res> get filter {
    return $FilterCopyWith<$Res>(_value.filter, (value) {
      return _then(_value.copyWith(filter: value));
    });
  }
}

/// @nodoc

class _$FilterSelection_TemporaryImpl extends FilterSelection_Temporary {
  const _$FilterSelection_TemporaryImpl({required this.filter}) : super._();

  @override
  final Filter filter;

  @override
  String toString() {
    return 'FilterSelection.temporary(filter: $filter)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$FilterSelection_TemporaryImpl &&
            (identical(other.filter, filter) || other.filter == filter));
  }

  @override
  int get hashCode => Object.hash(runtimeType, filter);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$FilterSelection_TemporaryImplCopyWith<_$FilterSelection_TemporaryImpl>
      get copyWith => __$$FilterSelection_TemporaryImplCopyWithImpl<
          _$FilterSelection_TemporaryImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(UuidValue uuid) predefined,
    required TResult Function(Filter filter) temporary,
  }) {
    return temporary(filter);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(UuidValue uuid)? predefined,
    TResult? Function(Filter filter)? temporary,
  }) {
    return temporary?.call(filter);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(UuidValue uuid)? predefined,
    TResult Function(Filter filter)? temporary,
    required TResult orElse(),
  }) {
    if (temporary != null) {
      return temporary(filter);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(FilterSelection_Predefined value) predefined,
    required TResult Function(FilterSelection_Temporary value) temporary,
  }) {
    return temporary(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(FilterSelection_Predefined value)? predefined,
    TResult? Function(FilterSelection_Temporary value)? temporary,
  }) {
    return temporary?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(FilterSelection_Predefined value)? predefined,
    TResult Function(FilterSelection_Temporary value)? temporary,
    required TResult orElse(),
  }) {
    if (temporary != null) {
      return temporary(this);
    }
    return orElse();
  }
}

abstract class FilterSelection_Temporary extends FilterSelection {
  const factory FilterSelection_Temporary({required final Filter filter}) =
      _$FilterSelection_TemporaryImpl;
  const FilterSelection_Temporary._() : super._();

  Filter get filter;
  @JsonKey(ignore: true)
  _$$FilterSelection_TemporaryImplCopyWith<_$FilterSelection_TemporaryImpl>
      get copyWith => throw _privateConstructorUsedError;
}
