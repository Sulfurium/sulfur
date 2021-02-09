use crate::lib::cli::Action;
use crate::lib::repo::file::get_repo_from_repod_formatted;

pub async fn repo(action: Action, repo_name: String, url: String) {
    let repo = get_repo_from_repod_formatted().await;
    match action {
        Action::Add => {
            for i in repo {
                if i.0 == repo_name {
                    println!("The repo is already installed");
                    return;
                }
            }
             write_repo_to_repod(repo_name, url).await;
        }
        Action::Del => {
            for i in repo {
                if i.0 == repo_name {
                    match async_std::fs::remove_file(format!("/etc/sulfur/repo.d/{}", repo_name)).await {
                        Ok(_) => {println!("Repo has been delete !");},
                        Err(e) => {
                            println!("Error : {}", e.to_string());
                        }
                    }
                }
            }

        }
        Action::Update => {
            for i in repo {
                if i.0 == repo_name.clone() {
                    write_repo_to_repod(repo_name.clone(), url.clone()).await;
                }
            }

        }
        Action::Error => {
            println!("Error");
        }
        Action::List => {
            for i in get_repo_from_repod_formatted().await {
                println!("{} => {}", i.0, i.1);
            }
        }
    }
}

pub async fn write_repo_to_repod(name: String, repo: String) {
    let towrite = format!("[{}] \nrepo = \"{}\"", name, repo);
    if async_std::fs::write(format!("/etc/sulfur/repo.d/{}", name), towrite).await.is_ok() {
        println!("Repo ({}) has been added", name);
    } else {
        println!("Oh no no")
    }
}