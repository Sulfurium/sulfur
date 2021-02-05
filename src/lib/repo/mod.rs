pub mod sync;
pub mod file;

use crate::lib::repo::file::get_repo_from_repod;

pub async fn sync() {
    let repos = get_repo_from_repod().await;
    for repo in repos {
        println!("Syncing : {}", repo.0);
        sync::sync(repo.1).await;
    }
}