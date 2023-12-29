use salsa::Snapshot;

use crate::source_edit::FileId;
use crate::{ir, PARSER};

#[salsa::query_group(SourceDatabaseStorage)]
pub trait SourceDatabase {
    // Parses the file into the syntax tree.
    // fn parse(&self, file_id: FileId) -> ();
}

#[salsa::query_group(FileDatabaseStorage)]
pub trait FileDatabase {
    // Parses the file into the syntax tree.
    // fn parse(&self, file_id: FileId) -> ();
}

fn parse(db: &dyn SourceDatabase, file_id: FileId) -> () {
    // PARSER.lock().parse()
}

/// The "master" database where every query from salsa is joined into.
#[salsa::database(SourceDatabaseStorage, FileDatabaseStorage, ir::HirDatabaseStorage)]
pub struct RootDatabase {
    storage: salsa::Storage<RootDatabase>,
}

impl salsa::Database for RootDatabase {}

impl salsa::ParallelDatabase for RootDatabase {
    fn snapshot(&self) -> Snapshot<Self> {
        Snapshot::new(RootDatabase {
            storage: self.storage.snapshot(),
        })
    }
}
