use crate::{Blob, Commit, Object, Tag, Tree};

mod convert;

mod write {
    use std::io;

    use crate::Object;

    /// Serialization
    impl Object {
        /// Write the contained object to `out` in the git serialization format.
        pub fn write_to(&self, out: impl io::Write) -> io::Result<()> {
            use crate::Object::*;
            match self {
                Tree(v) => v.write_to(out),
                Blob(v) => v.write_to(out),
                Commit(v) => v.write_to(out),
                Tag(v) => v.write_to(out),
            }
        }
    }
}

/// Convenient extraction of typed object.
impl Object {
    /// Returns a [`Blob`][Blob] if it is one.
    pub fn as_blob(&self) -> Option<&Blob> {
        match self {
            Object::Blob(v) => Some(v),
            _ => None,
        }
    }
    /// Returns a [`Commit`][Commit] if it is one.
    pub fn as_commit(&self) -> Option<&Commit> {
        match self {
            Object::Commit(v) => Some(v),
            _ => None,
        }
    }
    /// Returns a [`Tree`][Tree] if it is one.
    pub fn as_tree(&self) -> Option<&Tree> {
        match self {
            Object::Tree(v) => Some(v),
            _ => None,
        }
    }
    /// Returns a [`Tag`][Tag] if it is one.
    pub fn as_tag(&self) -> Option<&Tag> {
        match self {
            Object::Tag(v) => Some(v),
            _ => None,
        }
    }
    /// Returns the kind of object stored in this instance.
    pub fn kind(&self) -> crate::Kind {
        match self {
            Object::Tree(_) => crate::Kind::Tree,
            Object::Blob(_) => crate::Kind::Blob,
            Object::Commit(_) => crate::Kind::Commit,
            Object::Tag(_) => crate::Kind::Tag,
        }
    }
}