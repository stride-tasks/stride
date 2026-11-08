use super::*;

#[test]
fn key_validation() {
    for key in ["a", "app.dark-mode", "plugin.my_plugin.key1"] {
        assert!(validate_key(key).is_ok(), "{key}");
    }
    for key in ["", ".", "a.", ".a", "a..b", "a b", "a/b", "é"] {
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
    storage.set("app.b", &boolean)?;
    storage.set("app.a", &ConfigValue::string("x"))?;
    storage.set("app.a", &int)?;

    let items = storage.list()?;
    assert_eq!(
        items,
        vec![
            StorageItem {
                key: "app.a".into(),
                value: int.clone()
            },
            StorageItem {
                key: "app.b".into(),
                value: boolean
            },
        ]
    );

    let items = storage.get(&["app.a", "missing"])?;
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].value, int);

    let removed = storage.remove(&["app.a", "missing"])?;
    assert_eq!(removed, vec!["app.a".into()]);
    assert_eq!(storage.list()?.len(), 1);

    assert!(storage.set("bad key", &int).is_err());
    Ok(())
}
