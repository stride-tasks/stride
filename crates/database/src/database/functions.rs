use std::borrow::Cow;

use rusqlite::{
    Connection,
    functions::{Context, FunctionFlags},
};
use stride_core::task::Annotation;

use crate::{
    Result,
    conversion::{FromBlob, ToBlob},
};

fn init_annotation_functions(db: &Connection) -> Result<()> {
    db.create_scalar_function(
        "stride_annotation_array_insert",
        2,
        FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
        move |ctx| {
            assert_eq!(ctx.len(), 2, "called with unexpected number of arguments");

            let mut array = ctx
                .get_raw(0)
                .as_blob_or_null()
                .map_err(|e| rusqlite::Error::UserFunctionError(e.into()))?
                .map(|mut blob| Vec::<Annotation>::from_blob(&mut blob))
                .transpose()
                .map_err(|e| rusqlite::Error::UserFunctionError(e.into()))?
                .unwrap_or_default();

            let mut annotation_blob = ctx
                .get_raw(1)
                .as_blob()
                .map_err(|e| rusqlite::Error::UserFunctionError(e.into()))?;
            let annotation = Annotation::from_blob(&mut annotation_blob)
                .map_err(|e| rusqlite::Error::UserFunctionError(e.into()))?;

            array.push(annotation);

            let mut blob = Vec::new();
            array.to_blob(&mut blob);
            Ok(blob)
        },
    )?;
    db.create_scalar_function(
        "stride_annotation_array_remove",
        2,
        FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
        move |ctx: &Context<'_>| -> Result<Option<Cow<'_, [u8]>>, rusqlite::Error> {
            assert_eq!(ctx.len(), 2, "called with unexpected number of arguments");

            let Some(mut annotation_array_blob) = ctx
                .get_raw(0)
                .as_blob_or_null()
                .map_err(|e| rusqlite::Error::UserFunctionError(e.into()))?
            else {
                return Ok(None);
            };
            let mut array = Vec::<Annotation>::from_blob(&mut annotation_array_blob)
                .map_err(|e| rusqlite::Error::UserFunctionError(e.into()))?;

            let mut annotation_blob = ctx
                .get_raw(1)
                .as_blob()
                .map_err(|e| rusqlite::Error::UserFunctionError(e.into()))?;
            let annotation = Annotation::from_blob(&mut annotation_blob)
                .map_err(|e| rusqlite::Error::UserFunctionError(e.into()))?;

            if let Some(index) = array.iter().position(|element| *element == annotation) {
                array.remove(index);
            }

            if array.is_empty() {
                return Ok(None);
            }

            let mut blob = Vec::new();
            array.to_blob(&mut blob);
            Ok(Some(blob.into()))
        },
    )?;
    Ok(())
}

pub(super) fn init_stride_functions(db: &Connection) -> Result<()> {
    init_annotation_functions(db)?;
    Ok(())
}
