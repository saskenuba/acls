use std::path::PathBuf;

use nohash_hasher::IntMap;
use rustc_hash::FxHashMap;
use text_size::{TextRange, TextSize};

use crate::source_edit::FileId;

/// `InsertDelete` -- a single "atomic" change to text
///
/// Must not overlap with other `InDel`s
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Indel {
    pub insert: String,
    /// Refers to offsets in the original text
    pub delete: TextRange,
}

#[derive(Default, Debug, Clone)]
pub struct TextEdit {
    /// Invariant: disjoint and sorted by `delete`.
    indels: Vec<Indel>,
}

/// Contains paths and files for all source files available to ACLSP.
pub struct FileSet {
    files: FxHashMap<PathBuf, FileId>,
    paths: IntMap<FileId, PathBuf>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FilePosition {
    pub file_id: FileId,
    pub offset: TextSize,
}
