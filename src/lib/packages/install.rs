use crate::lib::db::{insert::insert, update::update_install_from_name};
use crate::lib::packages::pkg_struct::PKG;
use async_std::io::ReadExt;
use indicatif::ProgressBar;
use std::{io::{Read, Write}, thread, time::Duration};
use walkdir::WalkDir;

pub async fn install(packages: Vec<String>) {
    for package in packages {
        install_package(package.clone()).await;
    }
}

pub async fn install_package(package: String) {
    insert(package.clone()).await;
    let struct_pkg: PKG = match get_config_of_package(package.clone()) {
        Ok(e) => {
            e
        },
        Err(e) => {
            println!("{}", e.to_string());
            return;
        }
    };
    if struct_pkg.get_name() == "" {
        print!("I can't install Package");
    } else {
        if let Some(deps) = struct_pkg.dependence {
            for d in deps {
                if install_file_of_package(d.clone()).await.expect("Error") {
                    println!("{} was installed", d.clone());
                }
            }
        }

        if install_file_of_package(package.clone())
            .await
            .expect("Error")
        {
            println!("{} was installed", package.clone());
        }
        update_install_from_name(package.clone(), true).await;
    }
}

pub fn get_config_of_package(package: String) -> std::result::Result<PKG, toml::de::Error> {
    let path = format!("./temp/{}/.PKG", package);
    let mut pkg_info = String::new();
    std::fs::File::open(path).expect("Error").read_to_string(&mut pkg_info).expect("Error");
    toml::from_str::<PKG>(pkg_info.as_str())
}

pub async fn install_file_of_package(package: String) -> std::io::Result<bool> {
    let result = true;
    let path = format!("./temp/{}/", package);
    let af = analyze_folder(path).await;
    check_path(package, af).await;
    Ok(result)
}

pub async fn move_file(package_name: String, path: String) -> std::io::Result<bool> {
    let dest_path = path.clone();
    let source_path = format!("./temp/{}{}", package_name.clone(), path.clone());
    let mut buf: Vec<u8> = Vec::new();
    let mut source_file = async_std::fs::File::open(source_path.clone()).await?;
    source_file.read_to_end(&mut buf).await?;
    std::fs::File::create(dest_path.to_string())
        .unwrap()
        .write_all(buf.as_slice())
        .expect("Error");
    Ok(true)
}

async fn check_path(package_name: String, paths: Vec<Vec<String>>) {
    for path in paths {
        let mut n = 1;
        let mut string_path = String::new();
        let lenght = path.len();
        let wait = format!("{:.0}", (1.00 / lenght.to_string().parse::<f64>().unwrap_or_default() * 100.0 + 100.0)).parse::<u64>().unwrap();
        let pb = ProgressBar::new(lenght.clone().to_string().parse().unwrap());
        for p in path {
            pb.inc(1);
            string_path.push_str(format!("/{}", p).as_str());
            if n == lenght {
                move_file(package_name.clone(), string_path.clone())
                    .await
                    .expect("Error");
            } else {
                match async_std::fs::read_dir(string_path.clone()).await {
                    Ok(_) => {}
                    Err(e) => println!("{}) {} => {}", n, p, e),
                }
            }
            n += 1;
            thread::sleep(Duration::from_millis(wait));
        }
        pb.finish_and_clear();
    }
}
pub async fn analyze_folder(path: String) -> Vec<Vec<String>> {
    let mut vec: Vec<Vec<String>> = Vec::new();
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
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
