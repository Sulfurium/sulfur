use crate::lib::db::query::query_package;
use crate::lib::packages::pkg_struct::PKG;
use walkdir::WalkDir;

pub async fn query(packages: Vec<String>) {
    if packages.is_empty() {
        println!("No packages names given")
    } else {
        for package in packages {
            println!("{:?}", query_folder(package.clone()).await);
            format(query_package(package).await.expect("Error"));
        }
    }
}

pub fn format(vec_pkg: Vec<PKG>) {
    if vec_pkg.is_empty() {
        println!("Nothing Found");
    } else {
        for pkg in vec_pkg {
            let mut install = String::new();
            if pkg.installed.unwrap() {
                install.push_str("(Installed)")
            }
            println!(
                "{} {}-{} {} \n{}",
                pkg.name, pkg.version, pkg.subversion, install, pkg.description
            )
        }
    }
}

pub async fn query_folder(name: String) -> Vec<Vec<String>> {
    let mut vec: Vec<Vec<String>> = Vec::new();
    for entry in WalkDir::new(format!("./temp/{}", name))
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.metadata().expect("Err").is_file() {
            let contain = ["usr", "home", "bin"];
            let mut to_push = Vec::new();
            let mut push = false;
            let split = entry.path().to_str().expect("Error").split('/');
            for e in split.into_iter() {
                if push {
                    to_push.push(e.to_string());
                }
                if contain.contains(&e) {
                    push = true;
                    to_push.push(e.to_string());
                }
            }
            if !to_push.is_empty() {
                vec.push(to_push);
            }
        }
    }
    vec
}
