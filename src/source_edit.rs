use std::path::PathBuf;

/// Handle to a file in [`Vfs`]
///
/// Most functions in rust-analyzer use this when they need to refer to a file.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FileId(u32);

impl FileId {
    /// Think twice about using this outside of tests. If this ends up in a wrong place it will cause panics!
    // FIXME: To be removed once we get rid of all `SpanData::DUMMY` usages.
    pub const BOGUS: FileId = FileId(0xe4e4e);
    pub const MAX_FILE_ID: u32 = 0x7fff_ffff;

    #[inline]
    pub const fn from_raw(raw: u32) -> FileId {
        assert!(raw <= Self::MAX_FILE_ID);
        FileId(raw)
    }

    #[inline]
    pub fn index(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub enum FileSystemEdit {
    CreateFile { dst: PathBuf, initial_contents: String },
    MoveFile { src: FileId, dst: PathBuf },
    MoveDir { src: PathBuf, src_id: FileId, dst: PathBuf },
}
