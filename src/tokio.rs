//! Module for use with I/O supported by [`tokio`]

pub use self::{
    archive::{Archive, ArchiveBuilder, Entries},
    builder::Builder,
    entry::{Entry, Unpacked},
};

mod archive;
mod builder;
mod entry;
