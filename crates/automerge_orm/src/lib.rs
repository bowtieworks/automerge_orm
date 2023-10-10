//! An ORM for [Automerge].
//!
//! [Automerge]: https://crates.io/crates/automerge
//!
//! ## Concepts
//!
//! * **Entity manager** ([`EntityManager`]): The central access point to ORM
//!   functionality. Each entity manager wraps an Automerge document.
//!
//! * **Entity repository** ([`EntityRepository<T>`]): A **repository** where
//!   instances of an entity can be retrieved. Queries to be performed on the
//!   set of entities should be done through the repository.
//!
//! ## Derives
//!
//! * [`derive@Entity`]: Implements the [`Entity`] trait for the type.

/// Implements the [`Entity`] trait for the type.
pub use automerge_orm_macros::Entity;

pub use self::entity::Entity;
pub use self::entity_manager::EntityManager;
pub use self::entity_repository::{DefaultEntityRepository, EntityRepository};
pub use self::error::{Error, Result};
pub use self::impls::{create_table, find, find_all, get_table};
pub use self::key::Key;
pub use self::keyed::Keyed;
pub use self::mapped::Mapped;
pub use self::transaction::Transaction;

mod entity;
mod entity_manager;
mod entity_repository;
mod error;
pub mod impls;
mod key;
mod keyed;
mod mapped;
mod transaction;

#[doc(hidden)]
pub mod __macro_support {
    pub use std::{borrow::ToOwned, convert::Into, string::String};
}
