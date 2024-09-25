import 'package:flutter/material.dart';

@immutable
abstract class Result<T, E> {
  const Result();

  bool get isOk;
  bool get isErr => !isOk;

  T? ok() => null;
  E? err() => null;

  R match<R>({
    required R Function(T value) ok,
    required R Function(E error) err,
  });

  T unwrapOr(T elseValue);

  Result<R, E> map<R>(R Function(T value) f);
  Result<T, R> mapErr<R>(R Function(E error) f);

  factory Result.ok(T value) => Ok(value);
  factory Result.err(E value) => Err(value);

  static Future<Result<T, (E, StackTrace)>> catch_<T, E>(
    Future<T> Function() f,
  ) async {
    assert(E != Null, 'type must not be Null');

    try {
      return Ok(await f());
      // ignore: nullable_type_in_catch_clause
    } on E catch (error, stackTrace) {
      return Err((error, stackTrace));
    }
  }
}

class Ok<T, E> extends Result<T, E> {
  final T value;
  const Ok(this.value);

  @override
  bool get isOk => true;

  @override
  T? ok() => value;

  @override
  R match<R>({
    required R Function(T value) ok,
    required R Function(E error) err,
  }) =>
      ok(value);

  @override
  T unwrapOr(T elseValue) => value;

  @override
  Result<R, E> map<R>(R Function(T value) f) => Result.ok(f(value));
  @override
  Result<T, R> mapErr<R>(R Function(E error) _) => Result.ok(value);
}

class Err<T, E> extends Result<T, E> {
  final E error;
  const Err(this.error);

  @override
  bool get isOk => false;

  @override
  E? err() => error;

  @override
  R match<R>({
    required R Function(T value) ok,
    required R Function(E error) err,
  }) =>
      err(error);

  @override
  T unwrapOr(T elseValue) => elseValue;

  @override
  Result<R, E> map<R>(R Function(T value) _) => Result.err(error);
  @override
  Result<T, R> mapErr<R>(R Function(E error) f) => Result.err(f(error));
}
