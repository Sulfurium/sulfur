use std::path::PathBuf;

use crate::lib::file::decompression::decompression;

pub async fn _extract(path: PathBuf) -> std::io::Result<bool> {

    decompression(path.to_str().unwrap_or_default().to_string(), "/var/cache/sulfur/").await;

    Ok(true)
}
