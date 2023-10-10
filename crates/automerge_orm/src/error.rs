use std::{
    any::TypeId,
    fmt::{self, Display},
    sync::Arc,
};

use automerge::AutomergeError;
use autosurgeon::{HydrateError, ReconcileError};
use uuid::Uuid;

/// An error in the Automerge ORM.
#[derive(Debug)]
pub enum Error {
    Automerge(AutomergeError),
    Autosurgeon(AutosurgeonError),
    InvalidKey {
        key: String,
        source: uuid::Error,
    },
    KeyMismatch {
        actual: Uuid,
        expected: Uuid,
        msg: String,
    },
    ObjectAlreadyExists {
        table_name: String,
        id: Uuid,
    },
    ObjectDoesNotExist {
        table_name: String,
        id: Uuid,
    },
    Observer(Arc<dyn std::error::Error + Send + Sync + 'static>),
    TransactionAborted(Arc<dyn std::error::Error + Send + Sync + 'static>),
    UnsupportedType {
        type_id: TypeId,
        msg: String,
    },
}

#[derive(Debug)]
pub enum AutosurgeonError {
    Hydrate(HydrateError),
    Reconcile(ReconcileError),
}

/// A specialized [`Result`] type for Automerge ORM errors.
///
/// [`Result`]: std::result::Result
pub type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Automerge(err) => Some(err),
            Error::Autosurgeon(err) => err.source(),
            Error::InvalidKey { source, .. } => Some(source),
            Error::KeyMismatch { .. } => None,
            Error::ObjectAlreadyExists { .. } => None,
            Error::ObjectDoesNotExist { .. } => None,
            Error::Observer(err) => Some(err),
            Error::TransactionAborted(err) => Some(err),
            Error::UnsupportedType { .. } => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Automerge(err) => write!(f, "automerge: {err}"),
            Error::Autosurgeon(err) => write!(f, "autosurgeon: {err}"),
            Error::InvalidKey { source, .. } => write!(f, "{source}"),
            Error::KeyMismatch { msg, .. } => write!(f, "{msg}"),
            Error::ObjectAlreadyExists { table_name, id } => write!(
                f,
                "object with id \"{id}\" already exists in table \"{table_name}\""
            ),
            Error::ObjectDoesNotExist { table_name, id } => write!(
                f,
                "object with id \"{id}\" does not exist in table \"{table_name}\""
            ),
            Error::Observer(err) => write!(f, "observer: {err}"),
            Error::TransactionAborted(err) => write!(f, "transaction aborted: {err}"),
            Error::UnsupportedType { msg, .. } => write!(f, "{msg}"),
        }
    }
}

impl From<AutomergeError> for Error {
    fn from(err: AutomergeError) -> Self {
        Self::Automerge(err)
    }
}

impl From<AutosurgeonError> for Error {
    fn from(err: AutosurgeonError) -> Self {
        Self::Autosurgeon(err)
    }
}

impl From<HydrateError> for Error {
    fn from(err: HydrateError) -> Self {
        Self::Autosurgeon(err.into())
    }
}

impl From<ReconcileError> for Error {
    fn from(err: ReconcileError) -> Self {
        Self::Autosurgeon(err.into())
    }
}

impl std::error::Error for AutosurgeonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AutosurgeonError::Hydrate(err) => Some(err),
            AutosurgeonError::Reconcile(err) => Some(err),
        }
    }
}

impl Display for AutosurgeonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AutosurgeonError::Hydrate(err) => Display::fmt(err, f),
            AutosurgeonError::Reconcile(err) => Display::fmt(err, f),
        }
    }
}

impl From<HydrateError> for AutosurgeonError {
    fn from(err: HydrateError) -> Self {
        AutosurgeonError::Hydrate(err)
    }
}

impl From<ReconcileError> for AutosurgeonError {
    fn from(err: ReconcileError) -> Self {
        AutosurgeonError::Reconcile(err)
    }
}
