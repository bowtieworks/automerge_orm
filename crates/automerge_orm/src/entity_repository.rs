use std::{collections::BTreeMap, marker::PhantomData, sync::Arc};

use autosurgeon::Hydrate;

use crate::{find, find_all, EntityManager, Key, Mapped, Result};

/// A default implementation for [`EntityRepository`].
#[derive(Clone, Debug)]
pub struct DefaultEntityRepository<T> {
    entity_manager: Arc<EntityManager>,
    phantom: PhantomData<fn(T) -> T>,
}

/// A repository where instances of an entity can be retrieved.
pub trait EntityRepository<T> {
    /// Finds an object by its key / identifier.
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
    /// };
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
    /// type BookRepository = DefaultEntityRepository<Book>;
    ///
    /// # let repo_handle = Repo::new(None, Box::new(NoopStorage)).run();
    /// # let doc_handle = repo_handle.new_document();
    /// let entity_manager = Arc::new(EntityManager::new(doc_handle));
    /// let book_repository = BookRepository::new(Arc::clone(&entity_manager));
    ///
    /// let book_in = Book::new();
    /// entity_manager.transact(|tx| {
    ///     tx.insert(&book_in)?;
    ///     automerge_orm::Result::Ok(())
    /// })?;
    /// let book = book_repository.find(book_in.id())?;
    /// assert!(book.is_some());
    /// let book = book.unwrap();
    /// assert_eq!(book.id(), book_in.id());
    /// # repo_handle.stop().unwrap();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn find(&self, id: Key<T>) -> Result<Option<T>>;

    /// Finds all objects in the repository.
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
    /// };
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
    /// type BookRepository = DefaultEntityRepository<Book>;
    ///
    /// # let repo_handle = Repo::new(None, Box::new(NoopStorage)).run();
    /// # let doc_handle = repo_handle.new_document();
    /// let entity_manager = Arc::new(EntityManager::new(doc_handle));
    /// let book_repository = BookRepository::new(Arc::clone(&entity_manager));
    ///
    /// let books_in = vec![Book::new(), Book::new()];
    /// entity_manager.transact(|tx| {
    ///     for book_in in &books_in {
    ///         tx.insert(book_in)?;
    ///     }
    ///     automerge_orm::Result::Ok(())
    /// })?;
    /// let books = book_repository.find_all()?;
    /// assert_eq!(books.len(), 2);
    /// assert!(books.get(&books_in[0].id().to_string()).is_some());
    /// assert!(books.get(&books_in[1].id().to_string()).is_some());
    /// # repo_handle.stop().unwrap();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn find_all(&self) -> Result<BTreeMap<String, T>>;
}

impl<T> EntityRepository<T> for DefaultEntityRepository<T>
where
    T: Mapped + Hydrate,
{
    fn find(&self, id: Key<T>) -> Result<Option<T>> {
        self.entity_manager.doc().with_doc(|doc| find(doc, id))
    }

    fn find_all(&self) -> Result<BTreeMap<String, T>> {
        self.entity_manager.doc().with_doc(|doc| find_all(doc))
    }
}

impl<T> DefaultEntityRepository<T> {
    /// Creates a new `DefaultEntityRepository` which uses the
    /// [`EntityManager`].
    pub fn new(entity_manager: Arc<EntityManager>) -> Self {
        Self {
            entity_manager,
            phantom: PhantomData,
        }
    }
}
