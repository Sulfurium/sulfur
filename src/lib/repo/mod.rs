pub mod sync;
pub mod file;
pub mod update_repod;

use crate::lib::repo::file::get_repo_from_repod_formatted;

pub async fn sync() {
    let repos = get_repo_from_repod_formatted().await;
    for repo in repos {
        println!("Syncing : {}", repo.0);
        sync::sync(repo.1).await;
    }
}