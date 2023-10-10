use crate::Key;

/// An entity which can be identified by a key.
pub trait Keyed {
    /// The specific entity type the key represents.
    type Entity;

    /// Returns the key which identifies this entity.
    fn id(&self) -> Key<Self::Entity>;
}
