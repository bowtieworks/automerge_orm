#[cfg(feature = "automerge_repo")]
pub mod automerge_repo;
#[cfg(feature = "env")]
pub mod env;
pub mod function;

#[cfg(feature = "anyhow")]
pub type Result<T> = anyhow::Result<T>;
#[cfg(not(feature = "anyhow"))]
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;
