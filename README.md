# automerge_orm

An ORM for [Automerge].

[Automerge]: https://crates.io/crates/automerge

## Concepts

* **Entity manager** ([`EntityManager`]): The central access point to ORM
  functionality. Each entity manager wraps an Automerge document.

* **Entity repository** ([`EntityRepository<T>`]): A **repository** where
  instances of an entity can be retrieved. Queries to be performed on the
  set of entities should be done through the repository.

[`EntityManager`]: https://docs.rs/automerge_orm/latest/automerge_orm/struct.EntityManager.html
[`EntityRepository<T>`]: https://docs.rs/automerge_orm/latest/automerge_orm/trait.EntityRepository.html

## Derives

* [`Entity`][`derive@Entity`]: Implements the [`Entity`] trait for the type.

[`derive@Entity`]: https://docs.rs/automerge_orm/latest/automerge_orm/derive.Entity.html
[`Entity`]: https://docs.rs/automerge_orm/latest/automerge_orm/trait.Entity.html
