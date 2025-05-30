use crate::task::TaskStatus;

#[test]
fn conversion_task_status() -> anyhow::Result<()> {
    assert_eq!(
        serde_json::to_string(&TaskStatus::Complete)?,
        "\"complete\""
    );
    assert_eq!(serde_json::to_string(&TaskStatus::Deleted)?, "\"deleted\"");
    assert_eq!(serde_json::to_string(&TaskStatus::Pending)?, "\"pending\"");
    assert_eq!(
        serde_json::to_string(&TaskStatus::Recurring)?,
        "\"recurring\""
    );
    assert_eq!(serde_json::to_string(&TaskStatus::Waiting)?, "\"waiting\"");
    Ok(())
}
