use automerge::ChangeHash;
use automerge_repo::{DocumentId, Storage, StorageError};
use futures::future::{self, BoxFuture, FutureExt};

pub struct NoopStorage;

impl Storage for NoopStorage {
    fn get(&self, _id: DocumentId) -> BoxFuture<'static, Result<Option<Vec<u8>>, StorageError>> {
        future::ready(Ok(None)).boxed()
    }

    fn list_all(&self) -> BoxFuture<'static, Result<Vec<DocumentId>, StorageError>> {
        future::ready(Ok(Vec::new())).boxed()
    }

    fn append(
        &self,
        _id: DocumentId,
        _chunk: Vec<u8>,
    ) -> BoxFuture<'static, Result<(), StorageError>> {
        future::ready(Ok(())).boxed()
    }

    fn compact(
        &self,
        _id: DocumentId,
        _chunk: Vec<u8>,
        _new_heads: Vec<ChangeHash>,
    ) -> BoxFuture<'static, Result<(), StorageError>> {
        future::ready(Ok(())).boxed()
    }
}
