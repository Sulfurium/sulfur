use crate::lib::packages::query::query_folder;

pub async fn remove(packages: Vec<String>) {
    for i in packages {
        remove_package(i).await;
    }
}

pub async fn remove_package(package: String) {
    let query_file = query_folder(package).await;
    let formated = format_path(query_file);
    for f in formated {
        match async_std::fs::remove_file(f).await {
            Ok(_) => {
                println!("Delete file")
            }
            Err(e) => {
                println!("{} / {}", e.raw_os_error().unwrap_or(-1), e.to_string())
            }
        }
    }
}

pub fn format_path(paths: Vec<Vec<String>>) -> Vec<String> {
    let mut vpath: Vec<String> = Vec::new();
    for path in paths {
        let mut format_path = String::new();
        let lenght = path.len();
        let mut n = 0;
        for p in path {
            if n == lenght - 1 {
                format_path.push_str(format!("{}", p).as_str());
            } else if n == 0 {
                format_path.push_str(format!("/{}/", p).as_str());
            } else {
                format_path.push_str(format!("{}/", p).as_str());
            }
            n += 1;
        }
        vpath.push(format_path)
    }
    vpath
}
