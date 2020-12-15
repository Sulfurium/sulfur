use runas::Command;

pub async fn _compression(folder: String, file: String) -> bool {
    match Command::new("tar")
        .arg("--zstd")
        .arg("-cvf")
        .arg(file)
        .arg(folder.clone())
        .status()
    {
        Ok(_) => true,
        Err(_) => false,
    }
}
