//! Implementations of core functionalities of the Automerge ORM.
//!
//! These functions can be used in custom implementations of traits defined in
//! this crate.

use std::collections::BTreeMap;

use automerge::{AutomergeError, ObjId, ObjType, Prop, Value};
use autosurgeon::{hydrate_prop, Doc, Hydrate, ReadDoc};

use crate::{Key, Mapped, Result};

/// Finds an entity by key from the Automerge document.
pub fn find<D, T>(doc: &D, id: Key<T>) -> Result<Option<T>>
where
    D: ReadDoc,
    T: Mapped + Hydrate,
{
    let Some(table_id) = get_table::<D, T>(doc)? else {
        return Ok(None);
    };
    if doc.get(&table_id, Prop::Map(id.to_string()))?.is_none() {
        return Ok(None);
    }
    let entity = hydrate_prop(doc, table_id, &*id.to_string())?;

    Ok(Some(entity))
}

/// Finds all entities of a specific type from the Automerge document.
pub fn find_all<D, T>(doc: &D) -> Result<BTreeMap<String, T>>
where
    D: ReadDoc,
    T: Mapped + Hydrate,
{
    if get_table::<D, T>(doc)?.is_none() {
        return Ok(BTreeMap::new());
    };
    let entities = hydrate_prop(doc, automerge::ROOT, &*<T as Mapped>::table_name())?;

    Ok(entities)
}

/// Returns the Automerge object id of a table in the Automerge document.
pub fn get_table<D, T>(doc: &D) -> Result<Option<ObjId>>
where
    D: ReadDoc,
    T: Mapped,
{
    let Some((value, table_id)) =
        doc.get(&automerge::ROOT, Prop::Map(<T as Mapped>::table_name()))?
    else {
        return Ok(None);
    };
    let Value::Object(ObjType::Map) = value else {
        Err(AutomergeError::InvalidValueType {
            expected: format!("{}", Value::Object(ObjType::Map)),
            unexpected: format!("{value}"),
        })?
    };

    Ok(Some(table_id))
}

/// Creates a table in the Automerge document, and returns the Automerge object
/// id of the table.
pub fn create_table<D, T>(doc: &mut D) -> Result<ObjId>
where
    D: Doc,
    T: Mapped,
{
    let table_id = doc.put_object(
        automerge::ROOT,
        Prop::Map(<T as Mapped>::table_name()),
        ObjType::Map,
    )?;

    Ok(table_id)
}
