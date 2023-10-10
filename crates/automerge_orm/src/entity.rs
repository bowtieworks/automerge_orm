use crate::{Keyed, Mapped};

/// An entity represents an object which instances can be stored in an Automerge
/// document.
pub trait Entity: Mapped + Keyed {}
