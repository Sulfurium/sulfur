use crate::lib::packages::pkg_struct::PKG;
use std::io::Read;

pub fn install(packages: Vec<String>) {
    for package in packages {
        install_package(package)
    }
}

pub fn install_package(package: String) {
    let mut pkg_info = String::new();
    let mut file = std::fs::File::open(format!("./temp/{}/.PKG", package)).expect("Error");
    std::fs::File::read_to_string(&mut file, &mut pkg_info);

    let struck_pkg: PKG = toml::from_str(pkg_info.as_str()).unwrap();
    println!("{:?}", struck_pkg);
}
