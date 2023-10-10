use std::time::SystemTime;

use automerge::{
    transaction::{CommitOptions, Transactable, Transaction as AutomergeTransaction},
    Prop,
};
use autosurgeon::{reconcile_prop, Hydrate, ReadDoc, Reconcile};

use crate::{create_table, find, get_table, Error, Key, Keyed, Mapped, Result};

/// A transaction which groups operations together.
///
/// This `struct` is created by the [`transact`] method on [`EntityManager`].
/// See its documentation for more.
///
/// [`transact`]: crate::EntityManager::transact
/// [`EntityManager`]: crate::EntityManager
#[derive(Debug)]
pub struct Transaction<'a> {
    tx: AutomergeTransaction<'a>,
}

impl<'a> Transaction<'a> {
    pub(crate) fn new(tx: AutomergeTransaction<'a>) -> Self {
        Self { tx }
    }

    /// Inserts a new object instance.
    ///
    /// The object will be inserted into the document as a result of the
    /// [`commit`] operation.
    ///
    /// [`commit`]: Transaction::commit
    ///
    /// # Examples
    ///
    /// ```
    /// # use automerge::ChangeHash;
    /// # use automerge_repo::{DocumentId, Repo, Storage, StorageError};
    /// # use futures::future::{self, BoxFuture, FutureExt};
    /// #
    /// # pub struct NoopStorage;
    /// #
    /// # impl Storage for NoopStorage {
    /// #     fn get(
    /// #         &self,
    /// #         _id: DocumentId
    /// #     ) -> BoxFuture<'static, Result<Option<Vec<u8>>, StorageError>> {
    /// #         future::ready(Ok(None)).boxed()
    /// #     }
    /// #
    /// #     fn list_all(
    /// #         &self
    /// #     ) -> BoxFuture<'static, Result<Vec<DocumentId>, StorageError>> {
    /// #         future::ready(Ok(Vec::new())).boxed()
    /// #     }
    /// #
    /// #     fn append(
    /// #         &self,
    /// #         _id: DocumentId,
    /// #         _chunk: Vec<u8>,
    /// #     ) -> BoxFuture<'static, Result<(), StorageError>> {
    /// #         future::ready(Ok(())).boxed()
    /// #     }
    /// #
    /// #     fn compact(
    /// #         &self,
    /// #         _id: DocumentId,
    /// #         _chunk: Vec<u8>,
    /// #         _new_heads: Vec<ChangeHash>,
    /// #     ) -> BoxFuture<'static, Result<(), StorageError>> {
    /// #         future::ready(Ok(())).boxed()
    /// #     }
    /// # }
    /// #
    /// use std::sync::Arc;
    ///
    /// use automerge::ScalarValue;
    /// use automerge_orm::{
    ///     Entity,
    ///     EntityManager,
    ///     Keyed,
    ///     Mapped,
    /// };
    /// use automerge_test::{assert_doc, map};
    /// use autosurgeon::{Hydrate, Reconcile};
    /// use uuid::Uuid;
    ///
    /// #[derive(Clone, Debug, Entity, Hydrate, Reconcile)]
    /// struct Book {
    ///     #[key]
    ///     id: Uuid,
    /// }
    ///
    /// impl Book {
    ///     pub fn new() -> Self {
    ///         Self { id: Uuid::new_v4() }
    ///     }
    /// }
    ///
    /// # let repo_handle = Repo::new(None, Box::new(NoopStorage)).run();
    /// # let doc_handle = repo_handle.new_document();
    /// let entity_manager = Arc::new(EntityManager::new(doc_handle.clone()));
    ///
    /// let book = Book::new();
    /// entity_manager.transact(|tx| {
    ///     tx.insert(&book)?;
    ///     automerge_orm::Result::Ok(())
    /// })?;
    ///
    /// doc_handle.with_doc(|doc| {
    ///     assert_doc!(
    ///         doc,
    ///         map!{
    ///             Book::table_name() => {
    ///                 map!{
    ///                     book.id() => {
    ///                         map!{
    ///                             "id" => { ScalarValue::from(book.id()) },
    ///                         },
    ///                     },
    ///                 },
    ///             },
    ///         }
    ///     );
    /// });
    /// # repo_handle.stop().unwrap();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn insert<T>(&mut self, entity: &T) -> Result<()>
    where
        T: Mapped + Keyed<Entity = T> + Reconcile,
    {
        let table_id = if let Some(table_id) = get_table::<_, T>(&self.tx)? {
            if self
                .tx
                .get(&table_id, Prop::Map(entity.id().to_string()))?
                .is_some()
            {
                return Err(Error::ObjectAlreadyExists {
                    table_name: <T as Mapped>::table_name(),
                    id: entity.id().into(),
                });
            }
            table_id
        } else {
            create_table::<_, T>(&mut self.tx)?
        };
        reconcile_prop(&mut self.tx, &table_id, &*entity.id().to_string(), entity)?;

        Ok(())
    }

    /// Inserts a new object instance computed from `f` if an object with the
    /// same `id` does not exist, then returns the object identified by `id`.
    ///
    /// The new object will be inserted into the document as a result of the
    /// [`commit`] operation.
    ///
    /// [`commit`]: Transaction::commit
    ///
    /// # Examples
    ///
    /// Get an existing object instance:
    ///
    /// ```
    /// # use automerge::ChangeHash;
    /// # use automerge_repo::{DocumentId, Repo, Storage, StorageError};
    /// # use futures::future::{self, BoxFuture, FutureExt};
    /// #
    /// # pub struct NoopStorage;
    /// #
    /// # impl Storage for NoopStorage {
    /// #     fn get(
    /// #         &self,
    /// #         _id: DocumentId
    /// #     ) -> BoxFuture<'static, Result<Option<Vec<u8>>, StorageError>> {
    /// #         future::ready(Ok(None)).boxed()
    /// #     }
    /// #
    /// #     fn list_all(
    /// #         &self
    /// #     ) -> BoxFuture<'static, Result<Vec<DocumentId>, StorageError>> {
    /// #         future::ready(Ok(Vec::new())).boxed()
    /// #     }
    /// #
    /// #     fn append(
    /// #         &self,
    /// #         _id: DocumentId,
    /// #         _chunk: Vec<u8>,
    /// #     ) -> BoxFuture<'static, Result<(), StorageError>> {
    /// #         future::ready(Ok(())).boxed()
    /// #     }
    /// #
    /// #     fn compact(
    /// #         &self,
    /// #         _id: DocumentId,
    /// #         _chunk: Vec<u8>,
    /// #         _new_heads: Vec<ChangeHash>,
    /// #     ) -> BoxFuture<'static, Result<(), StorageError>> {
    /// #         future::ready(Ok(())).boxed()
    /// #     }
    /// # }
    /// #
    /// use std::sync::Arc;
    ///
    /// use automerge::ScalarValue;
    /// use automerge_orm::{
    ///     Entity,
    ///     EntityManager,
    ///     Keyed,
    ///     Mapped,
    /// };
    /// use automerge_test::{assert_doc, map};
    /// use autosurgeon::{Hydrate, Reconcile};
    /// use uuid::Uuid;
    ///
    /// #[derive(Clone, Debug, Entity, Hydrate, Reconcile)]
    /// struct Book {
    ///     #[key]
    ///     id: Uuid,
    ///     author: String,
    /// }
    ///
    /// impl Book {
    ///     pub fn new(id: Uuid, author: &str) -> Self {
    ///         Self {
    ///             id,
    ///             author: author.to_owned(),
    ///         }
    ///     }
    ///
    ///     pub fn author(&self) -> &str {
    ///         &self.author
    ///     }
    /// }
    ///
    /// # let repo_handle = Repo::new(None, Box::new(NoopStorage)).run();
    /// # let doc_handle = repo_handle.new_document();
    /// let entity_manager = Arc::new(EntityManager::new(doc_handle.clone()));
    ///
    /// let book_id = Uuid::new_v4();
    /// let book_in = Book::new(book_id, "Miyazaki Hayao");
    /// entity_manager.transact(|tx| {
    ///     tx.insert(&book_in)?;
    ///     automerge_orm::Result::Ok(())
    /// })?;
    /// let book = entity_manager.transact(|tx| {
    ///     let book = tx.get_or_insert(book_id.into(), || {
    ///         Book::new(book_id, "Shinkai Makoto")
    ///     })?;
    ///     automerge_orm::Result::Ok(book)
    /// })?;
    /// assert_eq!(book.id(), book_in.id());
    /// assert_eq!(book.author(), book_in.author());
    ///
    /// doc_handle.with_doc(|doc| {
    ///     assert_doc!(
    ///         doc,
    ///         map! {
    ///             Book::table_name() => {
    ///                 map!{
    ///                     book_id => {
    ///                         map!{
    ///                             "id" => { ScalarValue::from(book.id()) },
    ///                             "author" => { book_in.author() },
    ///                         },
    ///                     },
    ///                 },
    ///             },
    ///         }
    ///     );
    /// });
    /// # repo_handle.stop().unwrap();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// Insert a new object instance:
    ///
    /// ```
    /// # use automerge::ChangeHash;
    /// # use automerge_repo::{DocumentId, Repo, Storage, StorageError};
    /// # use futures::future::{self, BoxFuture, FutureExt};
    /// #
    /// # pub struct NoopStorage;
    /// #
    /// # impl Storage for NoopStorage {
    /// #     fn get(
    /// #         &self,
    /// #         _id: DocumentId
    /// #     ) -> BoxFuture<'static, Result<Option<Vec<u8>>, StorageError>> {
    /// #         future::ready(Ok(None)).boxed()
    /// #     }
    /// #
    /// #     fn list_all(
    /// #         &self
    /// #     ) -> BoxFuture<'static, Result<Vec<DocumentId>, StorageError>> {
    /// #         future::ready(Ok(Vec::new())).boxed()
    /// #     }
    /// #
    /// #     fn append(
    /// #         &self,
    /// #         _id: DocumentId,
    /// #         _chunk: Vec<u8>,
    /// #     ) -> BoxFuture<'static, Result<(), StorageError>> {
    /// #         future::ready(Ok(())).boxed()
    /// #     }
    /// #
    /// #     fn compact(
    /// #         &self,
    /// #         _id: DocumentId,
    /// #         _chunk: Vec<u8>,
    /// #         _new_heads: Vec<ChangeHash>,
    /// #     ) -> BoxFuture<'static, Result<(), StorageError>> {
    /// #         future::ready(Ok(())).boxed()
    /// #     }
    /// # }
    /// #
    /// use std::sync::Arc;
    ///
    /// use automerge::ScalarValue;
    /// use automerge_orm::{
    ///     Entity,
    ///     EntityManager,
    ///     Keyed,
    ///     Mapped,
    /// };
    /// use automerge_test::{assert_doc, map};
    /// use autosurgeon::{Hydrate, Reconcile};
    /// use uuid::Uuid;
    ///
    /// #[derive(Clone, Debug, Entity, Hydrate, Reconcile)]
    /// struct Book {
    ///     #[key]
    ///     id: Uuid,
    /// }
    ///
    /// impl Book {
    ///     pub fn new(id: Uuid) -> Self {
    ///         Self { id }
    ///     }
    /// }
    ///
    /// # let repo_handle = Repo::new(None, Box::new(NoopStorage)).run();
    /// # let doc_handle = repo_handle.new_document();
    /// let entity_manager = Arc::new(EntityManager::new(doc_handle.clone()));
    ///
    /// let book_id = Uuid::new_v4();
    /// let book = entity_manager.transact(|tx| {
    ///     let book = tx.get_or_insert(book_id.into(), || Book::new(book_id))?;
    ///     automerge_orm::Result::Ok(book)
    /// })?;
    /// assert_eq!(book.id(), book_id.into());
    ///
    /// doc_handle.with_doc(|doc| {
    ///     assert_doc!(
    ///         doc,
    ///         map!{
    ///             Book::table_name() => {
    ///                 map!{
    ///                     book_id => {
    ///                         map!{
    ///                             "id" => { ScalarValue::from(book.id()) },
    ///                         },
    ///                     },
    ///                 },
    ///             },
    ///         }
    ///     );
    /// });
    /// # repo_handle.stop().unwrap();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn get_or_insert<T, F>(&mut self, id: Key<T>, f: F) -> Result<T>
    where
        T: Mapped + Keyed<Entity = T> + Hydrate + Reconcile,
        F: FnOnce() -> T,
    {
        let entity = find(&self.tx, id)?;
        let entity = if let Some(entity) = entity {
            entity
        } else {
            let entity = f();
            if entity.id() != id {
                return Err(Error::KeyMismatch {
                    actual: entity.id().into(),
                    expected: id.into(),
                    msg: format!(
                        "key obtained from `<{} as automerge_orm::Keyed>::id()` does not match \
                        provided `id` key",
                        std::any::type_name::<T>()
                    ),
                });
            }
            self.insert(&entity)?;
            entity
        };

        Ok(entity)
    }

    /// Updates an existing object instance.
    ///
    /// The object will be updated in the document as a result of the [`commit`]
    /// operation.
    ///
    /// [`commit`]: Transaction::commit
    ///
    /// # Examples
    ///
    /// ```
    /// # use automerge::ChangeHash;
    /// # use automerge_repo::{DocumentId, Repo, Storage, StorageError};
    /// # use futures::future::{self, BoxFuture, FutureExt};
    /// #
    /// # pub struct NoopStorage;
    /// #
    /// # impl Storage for NoopStorage {
    /// #     fn get(
    /// #         &self,
    /// #         _id: DocumentId
    /// #     ) -> BoxFuture<'static, Result<Option<Vec<u8>>, StorageError>> {
    /// #         future::ready(Ok(None)).boxed()
    /// #     }
    /// #
    /// #     fn list_all(
    /// #         &self
    /// #     ) -> BoxFuture<'static, Result<Vec<DocumentId>, StorageError>> {
    /// #         future::ready(Ok(Vec::new())).boxed()
    /// #     }
    /// #
    /// #     fn append(
    /// #         &self,
    /// #         _id: DocumentId,
    /// #         _chunk: Vec<u8>,
    /// #     ) -> BoxFuture<'static, Result<(), StorageError>> {
    /// #         future::ready(Ok(())).boxed()
    /// #     }
    /// #
    /// #     fn compact(
    /// #         &self,
    /// #         _id: DocumentId,
    /// #         _chunk: Vec<u8>,
    /// #         _new_heads: Vec<ChangeHash>,
    /// #     ) -> BoxFuture<'static, Result<(), StorageError>> {
    /// #         future::ready(Ok(())).boxed()
    /// #     }
    /// # }
    /// #
    /// use std::sync::Arc;
    ///
    /// use automerge::ScalarValue;
    /// use automerge_orm::{
    ///     Entity,
    ///     EntityManager,
    ///     Keyed,
    ///     Mapped,
    /// };
    /// use automerge_test::{assert_doc, map};
    /// use autosurgeon::{Hydrate, Reconcile};
    /// use uuid::Uuid;
    ///
    /// #[derive(Clone, Debug, Entity, Hydrate, Reconcile)]
    /// struct Book {
    ///     #[key]
    ///     id: Uuid,
    ///     author: String,
    /// }
    ///
    /// impl Book {
    ///     pub fn new(author: &str) -> Self {
    ///         Self {
    ///             id: Uuid::new_v4(),
    ///             author: author.to_owned(),
    ///         }
    ///     }
    ///
    ///     pub fn author(&self) -> &str {
    ///         &self.author
    ///     }
    ///
    ///     pub fn set_author(&mut self, author: &str) {
    ///         self.author = author.to_owned();
    ///     }
    /// }
    ///
    /// # let repo_handle = Repo::new(None, Box::new(NoopStorage)).run();
    /// # let doc_handle = repo_handle.new_document();
    /// let entity_manager = Arc::new(EntityManager::new(doc_handle.clone()));
    ///
    /// let mut book = Book::new("Miyazaki Hayao");
    /// entity_manager.transact(|tx| {
    ///     tx.insert(&book)?;
    ///     automerge_orm::Result::Ok(())
    /// })?;
    /// book.set_author("Shinkai Makoto");
    /// entity_manager.transact(|tx| {
    ///     tx.update(&book)?;
    ///     automerge_orm::Result::Ok(())
    /// })?;
    ///
    /// doc_handle.with_doc(|doc| {
    ///     assert_doc!(
    ///         doc,
    ///         map! {
    ///             Book::table_name() => {
    ///                 map!{
    ///                     book.id() => {
    ///                         map!{
    ///                             "id" => { ScalarValue::from(book.id()) },
    ///                             "author" => { book.author() },
    ///                         },
    ///                     },
    ///                 },
    ///             },
    ///         }
    ///     );
    /// });
    /// # repo_handle.stop().unwrap();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn update<T>(&mut self, entity: &T) -> Result<()>
    where
        T: Mapped + Keyed<Entity = T> + Reconcile,
    {
        let Some(table_id) = get_table::<_, T>(&self.tx)? else {
            return Err(Error::ObjectDoesNotExist {
                table_name: <T as Mapped>::table_name(),
                id: entity.id().into(),
            });
        };
        if self
            .tx
            .get(&table_id, Prop::Map(entity.id().to_string()))?
            .is_none()
        {
            return Err(Error::ObjectDoesNotExist {
                table_name: <T as Mapped>::table_name(),
                id: entity.id().into(),
            });
        }
        reconcile_prop(&mut self.tx, &table_id, &*entity.id().to_string(), entity)?;

        Ok(())
    }

    /// Updates an existing object instance, or inserts a new object instance if
    /// it does not already exist.
    ///
    /// The object will be updated in / inserted into the document as a result
    /// of the [`commit`] operation.
    ///
    /// [`commit`]: Transaction::commit
    ///
    /// # Examples
    ///
    /// Update an existing object instance:
    ///
    /// ```
    /// # use automerge::ChangeHash;
    /// # use automerge_repo::{DocumentId, Repo, Storage, StorageError};
    /// # use futures::future::{self, BoxFuture, FutureExt};
    /// #
    /// # pub struct NoopStorage;
    /// #
    /// # impl Storage for NoopStorage {
    /// #     fn get(
    /// #         &self,
    /// #         _id: DocumentId
    /// #     ) -> BoxFuture<'static, Result<Option<Vec<u8>>, StorageError>> {
    /// #         future::ready(Ok(None)).boxed()
    /// #     }
    /// #
    /// #     fn list_all(
    /// #         &self
    /// #     ) -> BoxFuture<'static, Result<Vec<DocumentId>, StorageError>> {
    /// #         future::ready(Ok(Vec::new())).boxed()
    /// #     }
    /// #
    /// #     fn append(
    /// #         &self,
    /// #         _id: DocumentId,
    /// #         _chunk: Vec<u8>,
    /// #     ) -> BoxFuture<'static, Result<(), StorageError>> {
    /// #         future::ready(Ok(())).boxed()
    /// #     }
    /// #
    /// #     fn compact(
    /// #         &self,
    /// #         _id: DocumentId,
    /// #         _chunk: Vec<u8>,
    /// #         _new_heads: Vec<ChangeHash>,
    /// #     ) -> BoxFuture<'static, Result<(), StorageError>> {
    /// #         future::ready(Ok(())).boxed()
    /// #     }
    /// # }
    /// #
    /// use std::sync::Arc;
    ///
    /// use automerge::ScalarValue;
    /// use automerge_orm::{
    ///     Entity,
    ///     EntityManager,
    ///     Keyed,
    ///     Mapped,
    /// };
    /// use automerge_test::{assert_doc, map};
    /// use autosurgeon::{Hydrate, Reconcile};
    /// use uuid::Uuid;
    ///
    /// #[derive(Clone, Debug, Entity, Hydrate, Reconcile)]
    /// struct Book {
    ///     #[key]
    ///     id: Uuid,
    ///     author: String,
    /// }
    ///
    /// impl Book {
    ///     pub fn new(author: &str) -> Self {
    ///         Self {
    ///             id: Uuid::new_v4(),
    ///             author: author.to_owned(),
    ///         }
    ///     }
    ///
    ///     pub fn author(&self) -> &str {
    ///         &self.author
    ///     }
    ///
    ///     pub fn set_author(&mut self, author: &str) {
    ///         self.author = author.to_owned();
    ///     }
    /// }
    ///
    /// # let repo_handle = Repo::new(None, Box::new(NoopStorage)).run();
    /// # let doc_handle = repo_handle.new_document();
    /// let entity_manager = Arc::new(EntityManager::new(doc_handle.clone()));
    ///
    /// let mut book = Book::new("Miyazaki Hayao");
    /// entity_manager.transact(|tx| {
    ///     tx.upsert(&book)?;
    ///     automerge_orm::Result::Ok(())
    /// })?;
    /// book.set_author("Shinkai Makoto");
    /// entity_manager.transact(|tx| {
    ///     tx.upsert(&book)?;
    ///     automerge_orm::Result::Ok(())
    /// })?;
    ///
    /// doc_handle.with_doc(|doc| {
    ///     assert_doc!(
    ///         doc,
    ///         map! {
    ///             Book::table_name() => {
    ///                 map!{
    ///                     book.id() => {
    ///                         map!{
    ///                             "id" => { ScalarValue::from(book.id()) },
    ///                             "author" => { book.author() },
    ///                         },
    ///                     },
    ///                 },
    ///             },
    ///         }
    ///     );
    /// });
    /// # repo_handle.stop().unwrap();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// Insert a new object instance:
    ///
    /// ```
    /// # use automerge::ChangeHash;
    /// # use automerge_repo::{DocumentId, Repo, Storage, StorageError};
    /// # use futures::future::{self, BoxFuture, FutureExt};
    /// #
    /// # pub struct NoopStorage;
    /// #
    /// # impl Storage for NoopStorage {
    /// #     fn get(
    /// #         &self,
    /// #         _id: DocumentId
    /// #     ) -> BoxFuture<'static, Result<Option<Vec<u8>>, StorageError>> {
    /// #         future::ready(Ok(None)).boxed()
    /// #     }
    /// #
    /// #     fn list_all(
    /// #         &self
    /// #     ) -> BoxFuture<'static, Result<Vec<DocumentId>, StorageError>> {
    /// #         future::ready(Ok(Vec::new())).boxed()
    /// #     }
    /// #
    /// #     fn append(
    /// #         &self,
    /// #         _id: DocumentId,
    /// #         _chunk: Vec<u8>,
    /// #     ) -> BoxFuture<'static, Result<(), StorageError>> {
    /// #         future::ready(Ok(())).boxed()
    /// #     }
    /// #
    /// #     fn compact(
    /// #         &self,
    /// #         _id: DocumentId,
    /// #         _chunk: Vec<u8>,
    /// #         _new_heads: Vec<ChangeHash>,
    /// #     ) -> BoxFuture<'static, Result<(), StorageError>> {
    /// #         future::ready(Ok(())).boxed()
    /// #     }
    /// # }
    /// #
    /// use std::sync::Arc;
    ///
    /// use automerge::ScalarValue;
    /// use automerge_orm::{
    ///     Entity,
    ///     EntityManager,
    ///     Keyed,
    ///     Mapped,
    /// };
    /// use automerge_test::{assert_doc, map};
    /// use autosurgeon::{Hydrate, Reconcile};
    /// use uuid::Uuid;
    ///
    /// #[derive(Clone, Debug, Entity, Hydrate, Reconcile)]
    /// struct Book {
    ///     #[key]
    ///     id: Uuid,
    /// }
    ///
    /// impl Book {
    ///     pub fn new() -> Self {
    ///         Self { id: Uuid::new_v4() }
    ///     }
    /// }
    ///
    /// # let repo_handle = Repo::new(None, Box::new(NoopStorage)).run();
    /// # let doc_handle = repo_handle.new_document();
    /// let entity_manager = Arc::new(EntityManager::new(doc_handle.clone()));
    ///
    /// let book = Book::new();
    /// entity_manager.transact(|tx| {
    ///     tx.upsert(&book)?;
    ///     automerge_orm::Result::Ok(())
    /// })?;
    ///
    /// doc_handle.with_doc(|doc| {
    ///     assert_doc!(
    ///         doc,
    ///         map!{
    ///             Book::table_name() => {
    ///                 map!{
    ///                     book.id() => {
    ///                         map!{
    ///                             "id" => { ScalarValue::from(book.id()) },
    ///                         },
    ///                     },
    ///                 },
    ///             },
    ///         }
    ///     );
    /// });
    /// # repo_handle.stop().unwrap();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn upsert<T>(&mut self, entity: &T) -> Result<()>
    where
        T: Mapped + Keyed<Entity = T> + Reconcile,
    {
        let table_id = if let Some(table_id) = get_table::<_, T>(&self.tx)? {
            table_id
        } else {
            create_table::<_, T>(&mut self.tx)?
        };
        reconcile_prop(&mut self.tx, &table_id, &*entity.id().to_string(), entity)?;

        Ok(())
    }

    /// Removes an object by its identifier.
    ///
    /// The object will be removed from the document as a result of the
    /// [`commit`] operation.
    ///
    /// [`commit`]: Transaction::commit
    ///
    /// # Examples
    ///
    /// ```
    /// # use automerge::ChangeHash;
    /// # use automerge_repo::{DocumentId, Repo, Storage, StorageError};
    /// # use futures::future::{self, BoxFuture, FutureExt};
    /// #
    /// # pub struct NoopStorage;
    /// #
    /// # impl Storage for NoopStorage {
    /// #     fn get(
    /// #         &self,
    /// #         _id: DocumentId
    /// #     ) -> BoxFuture<'static, Result<Option<Vec<u8>>, StorageError>> {
    /// #         future::ready(Ok(None)).boxed()
    /// #     }
    /// #
    /// #     fn list_all(
    /// #         &self
    /// #     ) -> BoxFuture<'static, Result<Vec<DocumentId>, StorageError>> {
    /// #         future::ready(Ok(Vec::new())).boxed()
    /// #     }
    /// #
    /// #     fn append(
    /// #         &self,
    /// #         _id: DocumentId,
    /// #         _chunk: Vec<u8>,
    /// #     ) -> BoxFuture<'static, Result<(), StorageError>> {
    /// #         future::ready(Ok(())).boxed()
    /// #     }
    /// #
    /// #     fn compact(
    /// #         &self,
    /// #         _id: DocumentId,
    /// #         _chunk: Vec<u8>,
    /// #         _new_heads: Vec<ChangeHash>,
    /// #     ) -> BoxFuture<'static, Result<(), StorageError>> {
    /// #         future::ready(Ok(())).boxed()
    /// #     }
    /// # }
    /// #
    /// use std::sync::Arc;
    ///
    /// use automerge_orm::{
    ///     DefaultEntityRepository,
    ///     Entity,
    ///     EntityManager,
    ///     EntityRepository,
    ///     Keyed,
    ///     Mapped,
    /// };
    /// use automerge_test::{assert_doc, map};
    /// use autosurgeon::{Hydrate, Reconcile};
    /// use uuid::Uuid;
    ///
    /// #[derive(Clone, Debug, Entity, Hydrate, Reconcile)]
    /// struct Book {
    ///     #[key]
    ///     id: Uuid,
    /// }
    ///
    /// type BookRepository = DefaultEntityRepository<Book>;
    ///
    /// impl Book {
    ///     pub fn new() -> Self {
    ///         Self { id: Uuid::new_v4() }
    ///     }
    /// }
    ///
    /// # let repo_handle = Repo::new(None, Box::new(NoopStorage)).run();
    /// # let doc_handle = repo_handle.new_document();
    /// let entity_manager = Arc::new(EntityManager::new(doc_handle.clone()));
    /// let book_repository = BookRepository::new(Arc::clone(&entity_manager));
    ///
    /// let book = Book::new();
    /// entity_manager.transact(|tx| {
    ///     tx.insert(&book)?;
    ///     automerge_orm::Result::Ok(())
    /// })?;
    /// let book = book_repository.find(book.id())?;
    /// assert!(book.is_some());
    /// let book = book.unwrap();
    /// entity_manager.transact(|tx| {
    ///     tx.remove(book.id())?;
    ///     automerge_orm::Result::Ok(())
    /// })?;
    /// let book = book_repository.find(book.id())?;
    /// assert!(book.is_none());
    ///
    /// doc_handle.with_doc(|doc| {
    ///     assert_doc!(
    ///         doc,
    ///         map!{
    ///             Book::table_name() => {
    ///                 map!{},
    ///             },
    ///         }
    ///     );
    /// });
    /// # repo_handle.stop().unwrap();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn remove<T>(&mut self, id: Key<T>) -> Result<()>
    where
        T: Mapped,
    {
        let Some(table_id) = get_table::<_, T>(&self.tx)? else {
            return Ok(());
        };
        self.tx.delete(&table_id, Prop::Map(id.to_string()))?;

        Ok(())
    }

    /// Commits all changes that have been queued up to now to the document.
    pub fn commit(self) -> Result<()> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        self.tx.commit_with(
            CommitOptions::default()
                .with_message("automerge_orm::Transaction::commit")
                .with_time(now.as_secs() as i64),
        );

        Ok(())
    }

    /// Rolls back all changes that have been queued up.
    pub fn rollback(self) {
        self.tx.rollback();
    }
}
