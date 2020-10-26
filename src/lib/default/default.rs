use async_std::fs;
use tokio::stream::StreamExt;
use async_std::fs::read_dir;
use tokio::io::Error;
use async_std::fs::read;

pub async fn default() -> std::io::Result<()> {
    default_folder().await.expect("Error");
    default_file().await.expect("Error");
    Ok(())
}

async fn default_folder() -> std::io::Result<()> {
    if read_dir("/etc/sulfur").await.is_err() {
        match fs::create_dir("/etc/sulfur").await {
            Ok(_) => {println!("Ok")}
            Err(e) => {println!("{}", e)}
        }
    }
    if read_dir("/tmp/sulfur/").await.is_err() {
        match fs::create_dir("/tmp/sulfur").await {
            Ok(_) => {println!("Ok")}
            Err(e) => {println!("{}", e)}
        }
    }

    Ok(())
}

async fn default_file() -> std::io::Result<()> {
    if read("/etc/sulfur/sulfur.conf").await.is_err() {
        match fs::write("/etc/sulfur/sulfur.conf", "").await {
            Ok(_) => {println!("Ok")}
            Err(e) => {println!("{}", e)}
        }
    }

    Ok(())
}