use std::ops::Deref;

use salsa::Snapshot;
use text_size::TextSize;

use crate::capabilities;
use crate::db::{RootDatabase, SourceDatabase};

/// Analysis is a snapshot of a world state at a moment in time. It is the main
/// entry point for asking semantic information about the world. When the world
/// state is advanced using `AnalysisHost::apply_change` method, all existing
/// `Analysis` are canceled (most method return `Err(Canceled)`).
pub struct Analysis {
    db: Snapshot<RootDatabase>,
}

impl Analysis {
    fn move_function(&self) {
        todo!()
    }
    fn rename(&self) {
        todo!()
    }

    fn will_rename_file(&self) {
        todo!()
    }

    /// Returns choices of all aliases to standardize it.
    fn prepare_standardize_namespace_aliases(&self) {
        todo!()
    }

    /// Standardize all aliases to the one passed. Generally this will be called after
    /// `prepare_standardize_namespace_aliases`.
    fn standardize_namespace_aliases(&self, offset: TextSize) {
        // ensure nothing clashes with the new chosen alias

        capabilities::aliases::normalize(&self.db, offset);
        // self.db.parse()
        todo!()
    }
}
