/// An entity which is mapped to an Automerge document.
pub trait Mapped {
    fn table_name() -> String;
}
