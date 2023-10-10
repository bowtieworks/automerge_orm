use std::sync::Arc;

use automerge_repo::DocHandle;

use crate::{Error, Result, Transaction};

/// The central access point to ORM functionality.
#[derive(Debug)]
pub struct EntityManager {
    doc: DocHandle,
}

impl EntityManager {
    /// Creates a new `EntityManager` for an Automerge document.
    pub fn new(doc: DocHandle) -> Self {
        Self { doc }
    }

    /// Performs a transaction, running the provided function `f` within the
    /// context of the [`Transaction`], and returns its result.
    ///
    /// # Performance
    ///
    /// Within the scope of the function `f`, a write lock is held on the
    /// document. Do not perform expensive operations within the function `f`.
    pub fn transact<F, O, E>(&self, f: F) -> Result<O>
    where
        F: FnOnce(&mut Transaction<'_>) -> std::result::Result<O, E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        self.doc.with_doc_mut(|doc| {
            let mut tx = Transaction::new(doc.transaction());
            let result = f(&mut tx);
            match result {
                Ok(result) => {
                    tx.commit()?;
                    Ok(result)
                },
                Err(e) => {
                    tx.rollback();
                    Err(Error::TransactionAborted(Arc::new(e)))?
                },
            }
        })
    }

    /// Returns a handle to the Automerge document.
    pub fn doc(&self) -> DocHandle {
        self.doc.clone()
    }
}
