#[salsa::query_group(HirDatabaseStorage)]
pub trait HirDatabase {
    // #[salsa::invoke(Result::ok)]
    fn requires(&self) -> ();
}

fn requires(_: &dyn HirDatabase) -> () {
    todo!()
}
