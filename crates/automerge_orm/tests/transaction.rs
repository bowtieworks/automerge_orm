use std::sync::Arc;

use anyhow::Result;
use automerge::ScalarValue;
use automerge_orm::{
    DefaultEntityRepository, Entity, EntityManager, EntityRepository, Keyed, Mapped,
};
use automerge_repo::Repo;
use automerge_test::{assert_doc, map};
use autosurgeon::{Hydrate, Reconcile};
use test_utils::automerge_repo::NoopStorage;
use uuid::Uuid;

#[test]
fn it_inserts_new_entity() -> Result<()> {
    #[derive(Clone, Debug, Entity, Hydrate, Reconcile)]
    struct Book {
        #[key]
        id: Uuid,
    }

    impl Book {
        pub fn new() -> Self {
            Self { id: Uuid::new_v4() }
        }
    }

    let repo_handle = Repo::new(None, Box::new(NoopStorage)).run();
    let doc_handle = repo_handle.new_document();
    let entity_manager = Arc::new(EntityManager::new(doc_handle.clone()));

    let book = Book::new();
    entity_manager.transact(|tx| {
        tx.insert(&book)?;
        automerge_orm::Result::Ok(())
    })?;

    doc_handle.with_doc(|doc| {
        assert_doc!(
            doc,
            map! {
                Book::table_name() => {
                    map!{
                        book.id() => {
                            map!{
                                "id" => { ScalarValue::from(book.id()) },
                            },
                        },
                    },
                },
            }
        );
    });

    repo_handle.stop().unwrap();

    Ok(())
}

#[test]
fn it_fails_to_insert_new_entity_when_entity_with_same_id_already_exists() -> Result<()> {
    #[derive(Clone, Debug, Entity, Hydrate, Reconcile)]
    struct Book {
        #[key]
        id: Uuid,
    }

    impl Book {
        pub fn new() -> Self {
            Self { id: Uuid::new_v4() }
        }
    }

    let repo_handle = Repo::new(None, Box::new(NoopStorage)).run();
    let doc_handle = repo_handle.new_document();
    let entity_manager = Arc::new(EntityManager::new(doc_handle));

    let book = Book::new();
    entity_manager.transact(|tx| {
        tx.insert(&book)?;
        automerge_orm::Result::Ok(())
    })?;
    let result = entity_manager.transact(|tx| {
        tx.insert(&book)?;
        automerge_orm::Result::Ok(())
    });
    assert!(result.is_err());

    repo_handle.stop().unwrap();

    Ok(())
}

#[test]
fn it_gets_or_inserts_existing_entity() -> Result<()> {
    #[derive(Clone, Debug, Entity, Hydrate, Reconcile)]
    struct Book {
        #[key]
        id: Uuid,
        author: String,
    }

    impl Book {
        pub fn new(id: Uuid, author: &str) -> Self {
            Self {
                id,
                author: author.to_owned(),
            }
        }

        pub fn author(&self) -> &str {
            &self.author
        }
    }

    let repo_handle = Repo::new(None, Box::new(NoopStorage)).run();
    let doc_handle = repo_handle.new_document();
    let entity_manager = Arc::new(EntityManager::new(doc_handle.clone()));

    let book_id = Uuid::new_v4();
    let book_in = Book::new(book_id, "Miyazaki Hayao");
    entity_manager.transact(|tx| {
        tx.insert(&book_in)?;
        automerge_orm::Result::Ok(())
    })?;
    let book = entity_manager.transact(|tx| {
        let book = tx.get_or_insert(book_id.into(), || Book::new(book_id, "Shinkai Makoto"))?;
        automerge_orm::Result::Ok(book)
    })?;
    assert_eq!(book.id(), book_in.id());
    assert_eq!(book.author(), book_in.author());

    doc_handle.with_doc(|doc| {
        assert_doc!(
            doc,
            map! {
                Book::table_name() => {
                    map!{
                        book_id => {
                            map!{
                                "id" => { ScalarValue::from(book.id()) },
                                "author" => { book_in.author() },
                            },
                        },
                    },
                },
            }
        );
    });

    repo_handle.stop().unwrap();

    Ok(())
}

#[test]
fn it_gets_or_inserts_new_entity() -> Result<()> {
    #[derive(Clone, Debug, Entity, Hydrate, Reconcile)]
    struct Book {
        #[key]
        id: Uuid,
    }

    impl Book {
        pub fn new(id: Uuid) -> Self {
            Self { id }
        }
    }

    let repo_handle = Repo::new(None, Box::new(NoopStorage)).run();
    let doc_handle = repo_handle.new_document();
    let entity_manager = Arc::new(EntityManager::new(doc_handle.clone()));

    let book_id = Uuid::new_v4();
    let book = entity_manager.transact(|tx| {
        let book = tx.get_or_insert(book_id.into(), || Book::new(book_id))?;
        automerge_orm::Result::Ok(book)
    })?;
    assert_eq!(book.id(), book_id.into());

    doc_handle.with_doc(|doc| {
        assert_doc!(
            doc,
            map! {
                Book::table_name() => {
                    map!{
                        book_id => {
                            map!{
                                "id" => { ScalarValue::from(book.id()) },
                            },
                        },
                    },
                },
            }
        );
    });

    repo_handle.stop().unwrap();

    Ok(())
}

#[test]
fn it_fails_to_get_or_insert_new_entity_with_mismatched_id() -> Result<()> {
    #[derive(Clone, Debug, Entity, Hydrate, Reconcile)]
    struct Book {
        #[key]
        id: Uuid,
    }

    impl Book {
        pub fn new(id: Uuid) -> Self {
            Self { id }
        }
    }

    let repo_handle = Repo::new(None, Box::new(NoopStorage)).run();
    let doc_handle = repo_handle.new_document();
    let entity_manager = Arc::new(EntityManager::new(doc_handle));

    let book_id = Uuid::new_v4();
    let result = entity_manager.transact(|tx| {
        let book = tx.get_or_insert(book_id.into(), || Book::new(Uuid::new_v4()))?;
        automerge_orm::Result::Ok(book)
    });
    assert!(result.is_err());

    repo_handle.stop().unwrap();

    Ok(())
}

#[test]
fn it_updates_existing_entity() -> Result<()> {
    #[derive(Clone, Debug, Entity, Hydrate, Reconcile)]
    struct Book {
        #[key]
        id: Uuid,
        author: String,
    }

    impl Book {
        pub fn new(author: &str) -> Self {
            Self {
                id: Uuid::new_v4(),
                author: author.to_owned(),
            }
        }

        pub fn author(&self) -> &str {
            &self.author
        }

        pub fn set_author(&mut self, author: &str) {
            self.author = author.to_owned();
        }
    }

    let repo_handle = Repo::new(None, Box::new(NoopStorage)).run();
    let doc_handle = repo_handle.new_document();
    let entity_manager = Arc::new(EntityManager::new(doc_handle.clone()));

    let mut book = Book::new("Miyazaki Hayao");
    entity_manager.transact(|tx| {
        tx.insert(&book)?;
        automerge_orm::Result::Ok(())
    })?;
    book.set_author("Shinkai Makoto");
    entity_manager.transact(|tx| {
        tx.update(&book)?;
        automerge_orm::Result::Ok(())
    })?;

    doc_handle.with_doc(|doc| {
        assert_doc!(
            doc,
            map! {
                Book::table_name() => {
                    map!{
                        book.id() => {
                            map!{
                                "id" => { ScalarValue::from(book.id()) },
                                "author" => { book.author() },
                            },
                        },
                    },
                },
            }
        );
    });

    repo_handle.stop().unwrap();

    Ok(())
}

#[test]
fn it_fails_to_update_entity_which_does_not_exist() -> Result<()> {
    #[derive(Clone, Debug, Entity, Hydrate, Reconcile)]
    struct Book {
        #[key]
        id: Uuid,
    }

    impl Book {
        pub fn new() -> Self {
            Self { id: Uuid::new_v4() }
        }
    }

    let repo_handle = Repo::new(None, Box::new(NoopStorage)).run();
    let doc_handle = repo_handle.new_document();
    let entity_manager = Arc::new(EntityManager::new(doc_handle));

    let book = Book::new();
    let result = entity_manager.transact(|tx| {
        tx.update(&book)?;
        automerge_orm::Result::Ok(())
    });
    assert!(result.is_err());

    repo_handle.stop().unwrap();

    Ok(())
}

#[test]
fn it_upserts_new_entity() -> Result<()> {
    #[derive(Clone, Debug, Entity, Hydrate, Reconcile)]
    struct Book {
        #[key]
        id: Uuid,
    }

    impl Book {
        pub fn new() -> Self {
            Self { id: Uuid::new_v4() }
        }
    }

    let repo_handle = Repo::new(None, Box::new(NoopStorage)).run();
    let doc_handle = repo_handle.new_document();
    let entity_manager = Arc::new(EntityManager::new(doc_handle.clone()));

    let book = Book::new();
    entity_manager.transact(|tx| {
        tx.upsert(&book)?;
        automerge_orm::Result::Ok(())
    })?;

    doc_handle.with_doc(|doc| {
        assert_doc!(
            doc,
            map! {
                Book::table_name() => {
                    map!{
                        book.id() => {
                            map!{
                                "id" => { ScalarValue::from(book.id()) },
                            },
                        },
                    },
                },
            }
        );
    });

    repo_handle.stop().unwrap();

    Ok(())
}

#[test]
fn it_upserts_existing_entity() -> Result<()> {
    #[derive(Clone, Debug, Entity, Hydrate, Reconcile)]
    struct Book {
        #[key]
        id: Uuid,
        author: String,
    }

    impl Book {
        pub fn new(author: &str) -> Self {
            Self {
                id: Uuid::new_v4(),
                author: author.to_owned(),
            }
        }

        pub fn author(&self) -> &str {
            &self.author
        }

        pub fn set_author(&mut self, author: &str) {
            self.author = author.to_owned();
        }
    }

    let repo_handle = Repo::new(None, Box::new(NoopStorage)).run();
    let doc_handle = repo_handle.new_document();
    let entity_manager = Arc::new(EntityManager::new(doc_handle.clone()));

    let mut book = Book::new("Miyazaki Hayao");
    entity_manager.transact(|tx| {
        tx.upsert(&book)?;
        automerge_orm::Result::Ok(())
    })?;
    book.set_author("Shinkai Makoto");
    entity_manager.transact(|tx| {
        tx.upsert(&book)?;
        automerge_orm::Result::Ok(())
    })?;

    doc_handle.with_doc(|doc| {
        assert_doc!(
            doc,
            map! {
                Book::table_name() => {
                    map!{
                        book.id() => {
                            map!{
                                "id" => { ScalarValue::from(book.id()) },
                                "author" => { book.author() },
                            },
                        },
                    },
                },
            }
        );
    });

    repo_handle.stop().unwrap();

    Ok(())
}

#[test]
fn it_removes_entity_by_id() -> Result<()> {
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
    let entity_manager = Arc::new(EntityManager::new(doc_handle.clone()));
    let book_repository = BookRepository::new(Arc::clone(&entity_manager));

    let book = Book::new();
    entity_manager.transact(|tx| {
        tx.insert(&book)?;
        automerge_orm::Result::Ok(())
    })?;
    let book = book_repository.find(book.id())?;
    assert!(book.is_some());
    let book = book.unwrap();
    entity_manager.transact(|tx| {
        tx.remove(book.id())?;
        automerge_orm::Result::Ok(())
    })?;
    let book = book_repository.find(book.id())?;
    assert!(book.is_none());

    doc_handle.with_doc(|doc| {
        assert_doc!(
            doc,
            map! {
                Book::table_name() => {
                    map!{},
                },
            }
        );
    });

    repo_handle.stop().unwrap();

    Ok(())
}

#[test]
fn it_does_not_fail_when_trying_to_remove_entity_using_nonexistent_id() -> Result<()> {
    #[derive(Clone, Debug, Entity, Hydrate, Reconcile)]
    struct Book {
        #[key]
        id: Uuid,
    }

    impl Book {
        pub fn new() -> Self {
            Self { id: Uuid::new_v4() }
        }
    }

    let repo_handle = Repo::new(None, Box::new(NoopStorage)).run();
    let doc_handle = repo_handle.new_document();
    let entity_manager = Arc::new(EntityManager::new(doc_handle));

    let book_in = Book::new();
    entity_manager.transact(|tx| {
        tx.insert(&book_in)?;
        automerge_orm::Result::Ok(())
    })?;
    entity_manager.transact(|tx| {
        tx.remove::<Book>(Uuid::new_v4().into())?;
        automerge_orm::Result::Ok(())
    })?;

    repo_handle.stop().unwrap();

    Ok(())
}

#[test]
fn it_does_not_fail_when_trying_to_remove_entity_in_nonexistent_table() -> Result<()> {
    #[derive(Clone, Debug, Entity, Hydrate, Reconcile)]
    struct Book {
        #[key]
        id: Uuid,
    }

    let repo_handle = Repo::new(None, Box::new(NoopStorage)).run();
    let doc_handle = repo_handle.new_document();
    let entity_manager = Arc::new(EntityManager::new(doc_handle));

    entity_manager.transact(|tx| {
        tx.remove::<Book>(Uuid::new_v4().into())?;
        automerge_orm::Result::Ok(())
    })?;

    repo_handle.stop().unwrap();

    Ok(())
}
