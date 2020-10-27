use crate::lib::packages::pkg_struct::PKG;
use std::path::PathBuf;
use tokio::stream::StreamExt;

pub async fn read_package(path: PathBuf) -> std::io::Result<PKG> {
    let pkg = PKG::new();
    let mut entries = async_std::fs::read_dir(path).await?;
    while let Some(res) = entries.next().await {
        let entry = res?;
        println!("{}", entry.file_name().to_string_lossy());
    }

    Ok(pkg)
}
