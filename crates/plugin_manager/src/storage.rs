use stride_api::{self as api, Method, Value};

pub(crate) fn is_valid_plugin_name(name: &str) -> bool {
    !name.is_empty()
        && name.split('.').all(|part| {
            !part.is_empty()
                && part
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
        })
}

pub(crate) fn sanitize_plugin_storage_call(
    plugin_name: &str,
    method: &str,
    call: &mut Value,
) -> Result<(), Box<str>> {
    let group = format!("plugin.{plugin_name}");
    if !is_valid_plugin_name(plugin_name) {
        return Err(format!("invalid plugin storage group '{group}'").into());
    }

    match method {
        api::StorageSetMethod::NAME => {
            let params = params_map(call)?;
            let key = string_field(params, "key")?;
            let key = qualify_key(&group, &key)?;
            params.insert("key".into(), Value::String(key));
        }
        api::StorageGetMethod::NAME | api::StorageRemoveMethod::NAME => {
            let params = params_map(call)?;
            let Value::Array(items) = params
                .get("items")
                .cloned()
                .ok_or_else(|| Box::<str>::from("storage request is missing 'items'"))?
            else {
                return Err("storage request 'items' must be an array".into());
            };
            let items = items
                .into_iter()
                .map(|item| {
                    let Value::String(key) = item else {
                        return Err(Box::<str>::from("storage request items must be strings"));
                    };
                    Ok(Value::String(qualify_key(&group, &key)?))
                })
                .collect::<Result<Vec<_>, Box<str>>>()?;
            params.insert("items".into(), Value::Array(items));
        }
        api::StorageListMethod::NAME => {
            let params = params_map(call)?;
            match params.get("group") {
                None | Some(Value::Null) => {}
                Some(Value::String(requested_group)) if requested_group.as_ref() == group => {}
                Some(Value::String(requested_group)) => {
                    return Err(format!(
                        "plugin storage group '{requested_group}' is not owned by this plugin"
                    )
                    .into());
                }
                Some(_) => return Err("storage request 'group' must be a string".into()),
            }
            params.insert("group".into(), Value::String(group.into()));
        }
        _ => {}
    }

    Ok(())
}

fn params_map(call: &mut Value) -> Result<&mut api::Map, Box<str>> {
    match call {
        Value::Map(params) => Ok(params),
        _ => Err("storage method parameters must be an object".into()),
    }
}

fn string_field(params: &api::Map, field: &str) -> Result<Box<str>, Box<str>> {
    match params.get(field) {
        Some(Value::String(value)) => Ok(value.clone()),
        Some(_) => Err(format!("storage request '{field}' must be a string").into()),
        None => Err(format!("storage request is missing '{field}'").into()),
    }
}

fn qualify_key(group: &str, query: &str) -> Result<Box<str>, Box<str>> {
    let key = match query.split_once(':') {
        Some((requested_group, key)) if requested_group == group => key,
        Some((requested_group, _)) => {
            return Err(format!(
                "plugin storage group '{requested_group}' is not owned by this plugin"
            )
            .into());
        }
        None => query,
    };

    if key.is_empty()
        || !key.split('.').all(|part| {
            !part.is_empty()
                && part
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
        })
    {
        return Err(format!("invalid plugin storage key '{query}'").into());
    }

    Ok(format!("{group}:{key}").into())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params(entries: &[(&str, Value)]) -> Value {
        let mut map = api::Map::new();
        for (key, value) in entries {
            map.insert((*key).into(), value.clone());
        }
        Value::Map(map)
    }

    #[test]
    fn unqualified_storage_keys_are_scoped_to_the_plugin() {
        let mut call = params(&[("key", Value::String("settings.theme".into()))]);
        sanitize_plugin_storage_call("example_plugin", api::StorageSetMethod::NAME, &mut call)
            .unwrap();
        assert_eq!(
            call,
            params(&[(
                "key",
                Value::String("plugin.example_plugin:settings.theme".into())
            )])
        );
    }

    #[test]
    fn only_the_own_storage_group_is_accepted() {
        let mut call = params(&[(
            "items",
            Value::Array(vec![Value::String("other:key".into())]),
        )]);
        assert!(
            sanitize_plugin_storage_call("example", api::StorageGetMethod::NAME, &mut call)
                .is_err()
        );

        let mut call = params(&[(
            "items",
            Value::Array(vec![Value::String("plugin.example:key".into())]),
        )]);
        sanitize_plugin_storage_call("example", api::StorageGetMethod::NAME, &mut call).unwrap();
        assert_eq!(
            call,
            params(&[(
                "items",
                Value::Array(vec![Value::String("plugin.example:key".into())])
            )])
        );
    }

    #[test]
    fn storage_list_is_restricted_to_the_plugin_group() {
        let mut call = params(&[]);
        sanitize_plugin_storage_call("example", api::StorageListMethod::NAME, &mut call).unwrap();
        assert_eq!(
            call,
            params(&[("group", Value::String("plugin.example".into()))])
        );

        let mut call = params(&[("group", Value::String("other".into()))]);
        assert!(
            sanitize_plugin_storage_call("example", api::StorageListMethod::NAME, &mut call)
                .is_err()
        );
    }

    #[test]
    fn storage_keys_and_plugin_names_are_validated() {
        let mut call = params(&[("key", Value::String("bad key".into()))]);
        assert!(
            sanitize_plugin_storage_call("example", api::StorageSetMethod::NAME, &mut call)
                .is_err()
        );
        assert!(
            sanitize_plugin_storage_call("../example", api::StorageSetMethod::NAME, &mut call)
                .is_err()
        );
    }
}
