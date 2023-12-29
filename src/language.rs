use std::path::Path;

use tokio::fs::OpenOptions;
use tokio::io::AsyncReadExt;

pub type AnyResult<T> = anyhow::Result<T>;

async fn parse_file<T>(path: T) -> AnyResult<String>
where
    T: AsRef<Path>,
{
    let a = OpenOptions::new().open(path).await;
    let mut buffer = String::new();
    let b = a?.read_to_string(&mut buffer).await.unwrap();

    Ok(buffer)
}
