use stride_crdt::{
    actor::ActorId,
    change::{
        Change, Operation, RowId, Sequence,
        serialize::{operation_from_data, operation_to_data, operation_type},
    },
    hlc::Timestamp,
};
use stride_serialize::{FromBlob, ToBlob};
use uuid::Uuid;

pub(crate) fn change_to_data(change: &Change, blob: &mut Vec<u8>) {
    blob.push(0); // version

    change.sequence.to_blob(blob);
    change.timestamp.to_blob(blob);

    change.operations.len().to_blob(blob);
    for operation in &change.operations {
        match &operation.row_id {
            RowId::Uuid(uuid) => {
                blob.push(0);
                uuid.to_blob(blob);
            }
            RowId::String(value) => {
                blob.push(1);
                value.as_ref().to_blob(blob);
            }
        }

        let typ: u32 = operation_type(&operation.kind);
        typ.to_blob(blob);
        operation_to_data(&operation.kind, blob);
    }
}

pub(crate) fn change_from_data(
    actor_id: ActorId,
    mut input: &[u8],
) -> stride_serialize::Result<Change> {
    let version = u8::from_blob(&mut input)?;
    if version != 0 {
        todo!("unknown change version format: {version}");
    }
    let sequence = Sequence::from_blob(&mut input)?;
    let timestamp = Timestamp::from_blob(&mut input)?;
    let operation_len = usize::from_blob(&mut input)?;
    let mut operations = Vec::new();
    for _ in 0..operation_len {
        let row_id = match u8::from_blob(&mut input)? {
            0 => RowId::Uuid(Uuid::from_blob(&mut input)?),
            1 => RowId::String(<&str>::from_blob(&mut input)?.to_string().into_boxed_str()),
            value => todo!("row Id: {value}"),
        };
        let typ = u32::from_blob(&mut input)?;
        let kind = operation_from_data(typ, &mut input)?;
        operations.push(Operation { row_id, kind });
    }

    Ok(Change {
        actor_id,
        sequence,
        timestamp,
        operations,
    })
}
