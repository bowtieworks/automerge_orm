# automerge_orm

An ORM for [Automerge] to compliment [Autosurgeon]. It is most helpful in building structs from nested objects.

Here is a partial example

Automerge Document:

    {
        "contacts": {
            "99A86DA1-78D5-4CE2-A429-395458F055EE": { "name": "ringo" },
            "3A52D467-8BC9-4A81-BEDD-AB3768096351": { "name": "george" }, 
        },
        "bands": {
            ...
        }
    }

Rust:

    #[derive(Entity, Hydrate, Reconcile)]

    pub struct Contact {
        pub id: uuid::Uuid,
        pub name: String,
    }

It should be noted, that automerge_orm does not assist with any ACID type guarantees. This crate primairly manages the relationship between the document and your rust structs, not between two objects inside or between a document. 
    
[Automerge]: https://crates.io/crates/automerge
[Autosurgeon]: https://crates.io/crates/autosurgeon

## Concepts

* **Entity manager** ([`EntityManager`]): The central access point to ORM
  functionality. Each entity manager wraps an Automerge document.

* **Entity repository** ([`EntityRepository<T>`]): A **repository** where
  instances of an entity can be retrieved. Queries to be performed on the set of
  entities should be done through the repository.

[`EntityManager`]: https://docs.rs/automerge_orm/latest/automerge_orm/struct.EntityManager.html
[`EntityRepository<T>`]: https://docs.rs/automerge_orm/latest/automerge_orm/trait.EntityRepository.html

## Derives

* [`Entity`][`derive@Entity`]: Implements the [`Entity`] trait for the type.

[`derive@Entity`]: https://docs.rs/automerge_orm/latest/automerge_orm/derive.Entity.html
[`Entity`]: https://docs.rs/automerge_orm/latest/automerge_orm/trait.Entity.html

## License

Licensed under

Apache License, Version 2.0
([LICENSE](LICENSE) or https://www.apache.org/licenses/LICENSE-2.0)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.
