use std::{cell::Cell, future::Future, pin::Pin, sync::Mutex};

use futures_lite::future;
use wasmi::{
    AsContext, AsContextMut, Caller, Error, Func, FuncType, TypedFunc, Val, WasmParams,
    WasmResults, errors::HostError,
};

#[cfg(test)]
mod tests;

type AsyncHostFuture = Pin<Box<dyn Future<Output = Result<Vec<Val>, Error>> + Send + 'static>>;

struct AsyncFuturePayload {
    future: Mutex<Option<AsyncHostFuture>>,
}
impl std::fmt::Debug for AsyncFuturePayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("AsyncFuturePayload")
    }
}
impl std::fmt::Display for AsyncFuturePayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("asynchronous host function")
    }
}
impl HostError for AsyncFuturePayload {}

thread_local! {
    static ASYNC_CALL_ACTIVE: Cell<bool> = const { Cell::new(false) };
}

struct AsyncCallScope {
    previous: bool,
}

impl AsyncCallScope {
    fn enter() -> Self {
        Self {
            previous: ASYNC_CALL_ACTIVE.with(|active| active.replace(true)),
        }
    }

    fn is_active() -> bool {
        ASYNC_CALL_ACTIVE.with(Cell::get)
    }
}

impl Drop for AsyncCallScope {
    fn drop(&mut self) {
        ASYNC_CALL_ACTIVE.with(|active| active.set(self.previous));
    }
}

const FUEL_PER_CHECKPOINT: u64 = 100_000;

/// An asynchronous-call wrapper around a Wasmi [`Func`].
#[derive(Debug, Clone, Copy)]
pub struct AsyncFunc(Func);

impl AsyncFunc {
    /// Creates an async host function with the given signature.
    ///
    /// The callback receives owned parameter values because its future may outlive the call.
    pub fn wrap<T, F, Fut>(store: impl AsContextMut<Data = T>, ty: FuncType, f: F) -> Self
    where
        F: for<'a> Fn(Caller<'a, T>, Vec<Val>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Vec<Val>, Error>> + Send + 'static,
    {
        Self(Func::new(store, ty, move |caller, params, output| {
            let fut = f(caller, params.to_vec());
            if AsyncCallScope::is_active() {
                let future: AsyncHostFuture = Box::pin(fut);
                Err(Error::host(AsyncFuturePayload {
                    future: Mutex::new(Some(future)),
                }))
            } else {
                let result = future::block_on(fut)?;
                output.clone_from_slice(&result);
                Ok(())
            }
        }))
    }

    #[must_use]
    pub const fn new(func: Func) -> Self {
        Self(func)
    }

    /// Checks the signature and creates a typed async function wrapper.
    pub fn typed<Params, Results>(
        &self,
        context: impl AsContext,
    ) -> Result<AsyncTypedFunc<Params, Results>, Error>
    where
        Params: WasmParams,
        Results: WasmResults,
    {
        Ok(AsyncTypedFunc(self.0.typed::<Params, Results>(context)?))
    }

    /// Calls this function with typed inputs and returns typed results.
    pub async fn call<Params, Results>(
        &self,
        store: &mut impl AsContextMut,
        inputs: Params,
    ) -> Result<Results, Error>
    where
        Params: WasmParams,
        Results: WasmResults,
    {
        self.typed::<Params, Results>(&*store)?
            .call(store, inputs)
            .await
    }

    #[must_use]
    pub const fn as_func(&self) -> &Func {
        &self.0
    }

    #[must_use]
    pub fn into_func(self) -> Func {
        self.0
    }
}

impl From<Func> for AsyncFunc {
    fn from(func: Func) -> Self {
        Self::new(func)
    }
}

/// A typed asynchronous-call wrapper around a Wasmi [`TypedFunc`].
pub struct AsyncTypedFunc<Params, Results>(TypedFunc<Params, Results>);

impl<Params, Results> std::fmt::Debug for AsyncTypedFunc<Params, Results>
where
    TypedFunc<Params, Results>: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AsyncTypedFunc").field(&self.0).finish()
    }
}

impl<Params, Results> Copy for AsyncTypedFunc<Params, Results> {}

impl<Params, Results> Clone for AsyncTypedFunc<Params, Results> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Params, Results> AsyncTypedFunc<Params, Results>
where
    Params: WasmParams,
    Results: WasmResults,
{
    /// Calls this function with typed inputs and returns typed results.
    ///
    /// Fuel metering must be enabled on the engine. Execution yields and replenishes fuel
    /// whenever the current 100,000-fuel checkpoint is exhausted.
    pub async fn call(
        &self,
        store: &mut impl AsContextMut,
        inputs: Params,
    ) -> Result<Results, Error> {
        store.as_context_mut().set_fuel(FUEL_PER_CHECKPOINT)?;
        let mut next_call = {
            let _scope = AsyncCallScope::enter();
            self.0.call_resumable(&mut *store, inputs)?
        };

        loop {
            match next_call {
                wasmi::TypedResumableCall::Finished(results) => return Ok(results),
                wasmi::TypedResumableCall::OutOfFuel(resumability) => {
                    // Out of fuel, yield to allow other tasks to run and replenish fuel.
                    future::yield_now().await;

                    store
                        .as_context_mut()
                        .set_fuel(FUEL_PER_CHECKPOINT.max(resumability.required_fuel()))?;
                    next_call = {
                        let _scope = AsyncCallScope::enter();
                        resumability.resume(&mut *store)?
                    };
                }
                wasmi::TypedResumableCall::HostTrap(resumability) => {
                    let Some(payload) = resumability
                        .host_error()
                        .downcast_ref::<AsyncFuturePayload>()
                    else {
                        return Err(Error::new(resumability.host_error().to_string()));
                    };

                    let future = payload
                        .future
                        .lock()
                        .map_err(|_| Error::new("async host future was poisoned"))?
                        .take()
                        .ok_or_else(|| Error::new("async host future was already awaited"))?;
                    let outputs = future.await?;
                    next_call = {
                        let _scope = AsyncCallScope::enter();
                        resumability.resume(&mut *store, &outputs)?
                    };
                }
            }
        }
    }

    #[must_use]
    pub const fn as_typed_func(&self) -> &TypedFunc<Params, Results> {
        &self.0
    }

    #[must_use]
    pub fn into_typed_func(self) -> TypedFunc<Params, Results> {
        self.0
    }
}

impl<Params, Results> From<TypedFunc<Params, Results>> for AsyncTypedFunc<Params, Results> {
    fn from(func: TypedFunc<Params, Results>) -> Self {
        Self(func)
    }
}
