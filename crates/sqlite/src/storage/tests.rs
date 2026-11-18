use super::*;

#[test]
fn key_validation() {
    for key in ["a:b", "app:dark-mode", "plugin.my_plugin:key1"] {
        assert!(validate_key(key).is_ok(), "{key}");
    }
    for key in [
        "", ":", "a", "a:", ":a", "a:b:c", "a.:b", "a:.b", "a:b..c", "a b:c", "a:b c", "a/b:c",
        "é:a",
    ] {
        assert!(validate_key(key).is_err(), "{key}");
    }
}

#[test]
fn set_get_list_remove() -> Result<()> {
    let mut db = StorageDatabase::open_in_memory()?;
    let transaction = db.transaction()?;
    let storage = StorageTransaction::new(transaction);

    let int = ConfigValue::Integer { value: -5 };
    let boolean = ConfigValue::Boolean { value: true };
    storage.set("app:b", &boolean)?;
    storage.set("app:a", &ConfigValue::string("x"))?;
    storage.set("app:a", &int)?;

    let items = storage.list()?;
    assert_eq!(
        items,
        vec![
            StorageItem {
                key: "app:a".into(),
                value: int.clone()
            },
            StorageItem {
                key: "app:b".into(),
                value: boolean
            },
        ]
    );

    let items = storage.get(&["app:a", "app:missing"])?;
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].value, int);

    let removed = storage.remove(&["app:a", "app:missing"])?;
    assert_eq!(removed, vec!["app:a".into()]);
    assert_eq!(storage.list()?.len(), 1);

    assert!(storage.set("bad key", &int).is_err());
    Ok(())
}

#[test]
fn groups_are_independent_and_removed_when_empty() -> Result<()> {
    let mut db = StorageDatabase::open_in_memory()?;
    let storage = StorageTransaction::new(db.transaction()?);

    storage.set("a:key", &ConfigValue::Integer { value: 1 })?;
    storage.set("b:key", &ConfigValue::Integer { value: 2 })?;
    storage.set("a:other", &ConfigValue::Integer { value: 3 })?;

    let items = storage.get(&["b:key"])?;
    assert_eq!(items[0].value, ConfigValue::Integer { value: 2 });

    storage.remove(&["a:key", "a:other"])?;
    let groups: i64 = storage.transaction.query_row(
        "SELECT count(*) FROM storage_namespace_table",
        (),
        |row| row.get(0),
    )?;
    assert_eq!(groups, 1);

    storage.set("a:key", &ConfigValue::Integer { value: 4 })?;
    assert_eq!(storage.list()?.len(), 2);
    Ok(())
}

#[test]
fn list_group_returns_only_that_group() -> Result<()> {
    let mut db = StorageDatabase::open_in_memory()?;
    let storage = StorageTransaction::new(db.transaction()?);
    let value = ConfigValue::Integer { value: 1 };
    storage.set("plugin.one:key", &value)?;
    storage.set("plugin.two:key", &value)?;

    let items = storage.list_group("plugin.one")?;
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].key.as_ref(), "plugin.one:key");
    assert!(storage.list_group("plugin..one").is_err());
    Ok(())
}
