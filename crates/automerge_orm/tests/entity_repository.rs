use std::sync::Arc;

use anyhow::Result;
use automerge_orm::{DefaultEntityRepository, Entity, EntityManager, EntityRepository, Keyed};
use automerge_repo::Repo;
use autosurgeon::{Hydrate, Reconcile};
use test_utils::automerge_repo::NoopStorage;
use uuid::Uuid;

#[test]
fn it_finds_entity_by_id() -> Result<()> {
    #[derive(Clone, Debug, Entity, Hydrate, Reconcile)]
    struct Book {
        #[key]
        id: Uuid,
    }

    type BookRepository = DefaultEntityRepository<Book>;

    impl Book {
        pub fn new() -> Self {
            Self { id: Uuid::new_v4() }
        }
    }

    let repo_handle = Repo::new(None, Box::new(NoopStorage)).run();
    let doc_handle = repo_handle.new_document();
    let entity_manager = Arc::new(EntityManager::new(doc_handle));
    let book_repository = BookRepository::new(Arc::clone(&entity_manager));

    let book_in = Book::new();
    entity_manager.transact(|tx| {
        tx.insert(&book_in)?;
        automerge_orm::Result::Ok(())
    })?;
    let book = book_repository.find(book_in.id())?;
    assert!(book.is_some());
    let book = book.unwrap();
    assert_eq!(book.id(), book_in.id());

    repo_handle.stop().unwrap();

    Ok(())
}

#[test]
fn it_returns_none_when_trying_to_find_entity_using_nonexistent_id() -> Result<()> {
    #[derive(Clone, Debug, Entity, Hydrate, Reconcile)]
    struct Book {
        #[key]
        id: Uuid,
    }

    type BookRepository = DefaultEntityRepository<Book>;

    impl Book {
        pub fn new() -> Self {
            Self { id: Uuid::new_v4() }
        }
    }

    let repo_handle = Repo::new(None, Box::new(NoopStorage)).run();
    let doc_handle = repo_handle.new_document();
    let entity_manager = Arc::new(EntityManager::new(doc_handle));
    let book_repository = BookRepository::new(Arc::clone(&entity_manager));

    let book_in = Book::new();
    entity_manager.transact(|tx| {
        tx.insert(&book_in)?;
        automerge_orm::Result::Ok(())
    })?;
    let book = book_repository.find(Uuid::new_v4().into())?;
    assert!(book.is_none());

    repo_handle.stop().unwrap();

    Ok(())
}

#[test]
fn it_returns_none_when_trying_to_find_entity_in_nonexistent_table() -> Result<()> {
    #[derive(Clone, Debug, Entity, Hydrate, Reconcile)]
    struct Book {
        #[key]
        id: Uuid,
    }

    type BookRepository = DefaultEntityRepository<Book>;

    let repo_handle = Repo::new(None, Box::new(NoopStorage)).run();
    let doc_handle = repo_handle.new_document();
    let entity_manager = Arc::new(EntityManager::new(doc_handle));
    let book_repository = BookRepository::new(Arc::clone(&entity_manager));

    let book = book_repository.find(Uuid::new_v4().into())?;
    assert!(book.is_none());

    repo_handle.stop().unwrap();

    Ok(())
}

#[test]
fn it_finds_all_entities_in_a_table() -> Result<()> {
    #[derive(Clone, Debug, Entity, Hydrate, Reconcile)]
    struct Book {
        #[key]
        id: Uuid,
    }

    type BookRepository = DefaultEntityRepository<Book>;

    impl Book {
        pub fn new() -> Self {
            Self { id: Uuid::new_v4() }
        }
    }

    let repo_handle = Repo::new(None, Box::new(NoopStorage)).run();
    let doc_handle = repo_handle.new_document();
    let entity_manager = Arc::new(EntityManager::new(doc_handle));
    let book_repository = BookRepository::new(Arc::clone(&entity_manager));

    let books_in = vec![Book::new(), Book::new()];
    entity_manager.transact(|tx| {
        for book_in in &books_in {
            tx.insert(book_in)?;
        }
        automerge_orm::Result::Ok(())
    })?;
    let books = book_repository.find_all()?;
    assert_eq!(books.len(), 2);
    assert!(books.get(&books_in[0].id().to_string()).is_some());
    assert!(books.get(&books_in[1].id().to_string()).is_some());

    repo_handle.stop().unwrap();

    Ok(())
}

#[test]
fn it_returns_empty_map_when_trying_to_find_all_entities_in_nonexistent_table() -> Result<()> {
    #[derive(Clone, Debug, Entity, Hydrate, Reconcile)]
    struct Book {
        #[key]
        id: Uuid,
    }

    type BookRepository = DefaultEntityRepository<Book>;

    let repo_handle = Repo::new(None, Box::new(NoopStorage)).run();
    let doc_handle = repo_handle.new_document();
    let entity_manager = Arc::new(EntityManager::new(doc_handle));
    let book_repository = BookRepository::new(Arc::clone(&entity_manager));

    let book = book_repository.find_all()?;
    assert!(book.is_empty());

    repo_handle.stop().unwrap();

    Ok(())
}
