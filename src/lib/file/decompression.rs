use runas::Command;

pub async fn _decompression(file: String, folder: String) -> bool {
    if async_std::fs::read_dir(folder.clone()).await.is_err() {
        async_std::fs::create_dir(folder.clone())
            .await
            .expect("Cannot create folder");
    }
    match Command::new("tar")
        .arg("-xf")
        .arg(file)
        .arg("-C")
        .arg(folder.clone())
        .status()
    {
        Ok(_) => true,
        Err(_) => false,
    }
}
