use std::{
    future::Future,
    task::{Context, Poll, Waker},
};

use futures_lite::future::yield_now;

use super::AsyncFunc;
use wasmi::{Caller, Config, Engine, Error, FuncType, Linker, Module, Store, Val, ValType};

fn block_on<F: Future>(future: F) -> F::Output {
    let mut context = Context::from_waker(Waker::noop());
    let mut future = std::pin::pin!(future);

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

#[test]
fn async_host_functions_use_dynamic_parameters_and_results() {
    let mut config = Config::default();
    config.consume_fuel(true);
    let engine = Engine::new(&config);
    let module = Module::new(
        &engine,
        wat::parse_str(
            r#"
                (module
                    (import "env" "async_pair"
                        (func $async_pair (param i32 i64) (result i32 i64)))
                    (import "env" "async_fail"
                        (func $async_fail (param i32) (result i32)))
                    (func (export "call_pair") (param i32 i64) (result i32 i64)
                        local.get 0
                        local.get 1
                        call $async_pair)
                    (func (export "call_fail") (param i32) (result i32)
                        local.get 0
                        call $async_fail)
                    (func (export "work") (result i32)
                        (local $i i32)
                        (block $done
                            (loop $again
                                local.get $i
                                i32.const 100000
                                i32.ge_u
                                br_if $done
                                local.get $i
                                i32.const 1
                                i32.add
                                local.set $i
                                br $again))
                        local.get $i))
                "#,
        )
        .expect("valid WAT"),
    )
    .expect("valid Wasm");

    let mut store = Store::new(&engine, ());
    let mut linker = Linker::new(&engine);
    linker
        .define(
            "env",
            "async_pair",
            AsyncFunc::wrap(
                &mut store,
                FuncType::new([ValType::I32, ValType::I64], [ValType::I32, ValType::I64]),
                |_: Caller<'_, ()>, params| async move {
                    let left = params[0]
                        .i32()
                        .ok_or_else(|| Error::new("expected i32 parameter"))?;
                    let right = params[1]
                        .i64()
                        .ok_or_else(|| Error::new("expected i64 parameter"))?;
                    yield_now().await;
                    yield_now().await;
                    yield_now().await;
                    Ok(vec![(left + 1).into(), (right + 2).into()])
                },
            )
            .into_func(),
        )
        .expect("define async host function");
    linker
        .define(
            "env",
            "async_fail",
            AsyncFunc::wrap(
                &mut store,
                FuncType::new([ValType::I32], [ValType::I32]),
                |_: Caller<'_, ()>, _| async { Err(Error::new("async host failure")) },
            )
            .into_func(),
        )
        .expect("define fallible async host function");

    let instance = linker
        .instantiate_and_start(&mut store, &module)
        .expect("instantiate module");
    let pair = instance
        .get_func(&store, "call_pair")
        .expect("exported pair function");
    let pair = block_on(AsyncFunc::new(pair).call::<(i32, i64), (i32, i64)>(&mut store, (4, 9)))
        .expect("async host call");
    assert_eq!(pair, (5, 11));

    let fail = instance
        .get_func(&store, "call_fail")
        .expect("exported failure function");
    let error = block_on(AsyncFunc::new(fail).call::<i32, i32>(&mut store, 0))
        .expect_err("async host error should propagate");
    assert_eq!(error.to_string(), "async host failure");

    let work = instance
        .get_func(&store, "work")
        .expect("exported work function");
    let work = AsyncFunc::new(work)
        .typed::<(), i32>(&store)
        .expect("typed work function");
    let result = block_on(work.call(&mut store, ())).expect("resumed after fuel checkpoints");
    assert_eq!(result, 100_000);
}

#[test]
fn regular_func_call_blocks_on_async_host_future() {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let func = AsyncFunc::wrap(
        &mut store,
        FuncType::new([ValType::I32], [ValType::I32]),
        |_: Caller<'_, ()>, params| async move {
            yield_now().await;
            yield_now().await;
            yield_now().await;
            let value = params[0]
                .i32()
                .ok_or_else(|| Error::new("expected i32 parameter"))?;
            Ok(vec![(value + 1).into()])
        },
    )
    .into_func();

    let mut results = [Val::I32(0)];
    func.call(&mut store, &[Val::I32(41)], &mut results)
        .expect("regular Func::call should wait for async host future");

    assert_eq!(results[0].i32(), Some(42));
}
