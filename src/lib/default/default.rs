use crate::lib::db::create::create;
use async_std::fs;
use async_std::fs::read;
use async_std::fs::read_dir;

pub async fn default() -> std::io::Result<()> {
    default_folder().await.expect("Error");
    default_file().await.expect("Error");
    create().await;
    Ok(())
}

async fn default_folder() -> std::io::Result<()> {
    if read_dir("/etc/sulfur").await.is_err() {
        match fs::create_dir("/etc/sulfur").await {
            Ok(_) => {
                if read_dir("/etc/sulfur/hook.d").await.is_err() {
                    match fs::create_dir("/etc/sulfur/hook.d").await {
                        Ok(_) => {}
                        Err(e) => println!("{}", e),
                    }
                };
                if read_dir("/etc/sulfur/repo.d").await.is_err() {
                    match fs::create_dir("/etc/sulfur/repo.d").await {
                        Ok(_) => {}
                        Err(e) => println!("Error: {}", e),
                    }
                };
            }
            Err(e) => println!("Error: {}", e),
        }
    }
    if read_dir("/tmp/sulfur/").await.is_err() {
        match fs::create_dir("/tmp/sulfur").await {
            Ok(_) => {}
            Err(e) => println!("{}", e),
        }
    }

    Ok(())
}

async fn default_file() -> std::io::Result<()> {
    if read("/etc/sulfur/sulfur.conf").await.is_err() {
        match fs::write("/etc/sulfur/sulfur.conf", "").await {
            Ok(_) => println!("Ok"),
            Err(e) => println!("{}", e),
        }
    }
    if read("/etc/sulfur/db.sql").await.is_err() {
        match fs::write("/etc/sulfur/db.sql", "").await {
            Ok(_) => println!("Ok"),
            Err(e) => println!("{}", e),
        }
    }
    Ok(())
}
