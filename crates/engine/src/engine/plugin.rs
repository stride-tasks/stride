use std::sync::Arc;

use stride_api as api;

use crate::Engine;

pub(crate) fn process_plugin_work(engine: &Arc<Engine>, name: &str, value: &api::Value) {
    engine.lock_plugin_manager().emit_notification(name, value);

    loop {
        let calls = {
            let mut manager = engine.lock_plugin_manager();
            loop {
                match manager.process_plugin_message() {
                    Ok(true) => {}
                    Ok(false) => break,
                    Err(error) => log::error!("plugin execution failed: {error}"),
                }
            }
            std::iter::from_fn(|| manager.process_plugin_call()).collect::<Vec<_>>()
        };

        if calls.is_empty() {
            break;
        }

        for (plugin_name, call) in calls {
            let (method, params) = match parse_plugin_method(call) {
                Ok(call) => call,
                Err(reason) => {
                    log::info!("disabling plugin {plugin_name} due to parse error: {reason}");
                    if let Err(error) = engine
                        .lock_plugin_manager()
                        .disable(&plugin_name, Some(reason.clone()))
                    {
                        log::error!(
                            "failed to disable plugin {plugin_name} with reason {reason}: {error}"
                        );
                    }
                    continue;
                }
            };

            let permission_error = {
                let manager = engine.lock_plugin_manager();
                if manager.plugin(&plugin_name).is_none() {
                    continue;
                }
                manager.method_permission_error(&plugin_name, &method)
            };
            if let Some(reason) = permission_error {
                log::info!("disabling plugin {plugin_name} due to permission error: {reason}");
                if let Err(error) = engine
                    .lock_plugin_manager()
                    .disable(&plugin_name, Some(reason.clone()))
                {
                    log::error!(
                        "failed to disable plugin {plugin_name} with reason {reason}: {error}"
                    );
                }
                continue;
            }

            let result = match engine.execute_erased(&method, params) {
                Ok(result) => result,
                Err(error) => {
                    log::error!("plugin {plugin_name} method {method} failed: {error}");
                    continue;
                }
            };

            let mut response = api::Map::default();
            response.insert("method".into(), api::Value::String(method.clone()));
            response.insert("result".into(), result);
            engine
                .lock_plugin_manager()
                .enqueue_plugin_message(&plugin_name, api::Value::Map(response));
        }
    }
}

fn parse_plugin_method(call: api::Value) -> Result<(Box<str>, api::Value), Box<str>> {
    let api::Value::Map(mut call) = call else {
        return Err("plugin call must be an API method object".into());
    };
    let Some(api::Value::String(method)) = call.shift_remove("method") else {
        return Err("plugin call is missing a string method name".into());
    };
    let Some(params) = call.shift_remove("params") else {
        return Err("plugin call is missing params".into());
    };
    Ok((method, params))
}
