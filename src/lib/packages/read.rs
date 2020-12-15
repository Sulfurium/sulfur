use crate::lib::packages::pkg_struct::PKG;
use async_std::stream::StreamExt;
use std::path::PathBuf;

pub async fn _read_package(path: PathBuf) -> std::io::Result<PKG> {
    let pkg = PKG::new();
    let mut entries = async_std::fs::read_dir(path).await?;
    while let Some(res) = entries.next().await {
        let entry = res?;
        println!("{}", entry.file_name().to_string_lossy());
    }

    Ok(pkg)
}
