use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use automerge::ScalarValue;
use uuid::Uuid;

use crate::{Error, Result};

/// A key which identifies an entity.
pub struct Key<T: ?Sized>(Uuid, PhantomData<fn(T) -> T>);

impl<T: ?Sized> Copy for Key<T> {}

impl<T: ?Sized> Clone for Key<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Eq for Key<T> {}

impl<T: ?Sized> PartialEq for Key<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: ?Sized> Ord for Key<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: ?Sized> PartialOrd for Key<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: ?Sized> Hash for Key<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: ?Sized> fmt::Debug for Key<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(&format!("Key<{}>", std::any::type_name::<T>()))
            .field(&self.0)
            .finish()
    }
}

impl<T: ?Sized> fmt::Display for Key<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: ?Sized> AsRef<Uuid> for Key<T> {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl<T: ?Sized> From<Uuid> for Key<T> {
    fn from(uuid: Uuid) -> Self {
        Self::new(uuid)
    }
}

impl<T: ?Sized> TryFrom<&str> for Key<T> {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        let uuid = Uuid::try_from(s).map_err(|e| Error::InvalidKey {
            key: s.to_owned(),
            source: e,
        })?;

        Ok(Self::new(uuid))
    }
}

impl<T: ?Sized> From<Key<T>> for Uuid {
    fn from(key: Key<T>) -> Self {
        key.0
    }
}

impl<T: ?Sized> From<Key<T>> for ScalarValue {
    fn from(key: Key<T>) -> Self {
        ScalarValue::Bytes(key.0.as_bytes().to_vec())
    }
}

impl<T: ?Sized> Key<T> {
    /// Creates a new `Key` from a [`Uuid`].
    ///
    /// The key is specific to the entity type `T`.
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid, PhantomData)
    }
}
