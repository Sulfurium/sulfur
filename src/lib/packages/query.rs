use crate::lib::db::conn::conn;
use crate::lib::packages::pkg_struct::{Architecture, Licenses, PKG};
use sqlx::Row;
use std::str::FromStr;
use tokio::stream::StreamExt;
use toml::value::Datetime;

pub async fn query(packages: Vec<String>) {
    if packages.is_empty() {
        println!("No packages names given")
    } else {
        for package in packages {
            format(query_package(package).await.expect("Error"));
        }
    }
}

async fn query_package(package: String) -> std::io::Result<Vec<PKG>> {
    let mut conn = conn().await;
    let mut rows = sqlx::query("SELECT * FROM Packages WHERE name = ?")
        .bind(package)
        .fetch(&mut conn);

    let mut vec_pkg: Vec<PKG> = Vec::new();

    while let Some(row) = rows.try_next().await.expect("Error") {
        let mut pkg = PKG::new();

        pkg.set_name(
            row.try_get::<&str, &str>("name")
                .expect("Error")
                .to_string(),
        );
        pkg.set_version(
            (row.try_get::<&str, &str>("version").expect("Error"))
                .parse()
                .unwrap(),
        );
        pkg.set_subversion(
            (row.try_get::<&str, &str>("subversion").expect("Error"))
                .parse()
                .unwrap_or(0),
        );
        pkg.set_description(
            row.try_get::<&str, &str>("description")
                .expect("Error")
                .to_string(),
        );
        pkg.set_url(row.try_get::<&str, &str>("url").expect("Error").to_string());
        pkg.set_packager(
            row.try_get::<&str, &str>("packager")
                .expect("Error")
                .to_string(),
        );
        pkg.set_date(
            Datetime::from_str(row.try_get::<&str, &str>("date").expect("Error"))
                .unwrap_or(Datetime::from_str("1979-05-27T07:32:00-08:00").unwrap()),
        );
        pkg.set_license(
            Licenses::from_str(row.try_get::<&str, &str>("license").expect("Error"))
                .unwrap_or(Licenses::NO_LICENSE),
        );
        pkg.set_architecture(
            Architecture::from_str(row.try_get::<&str, &str>("architecture").expect("Error"))
                .unwrap_or(Architecture::ANY),
        );
        pkg.set_installed_from_i64(row.try_get::<i64, &str>("installed").expect("Error"))
            .expect("Error");

        vec_pkg.push(pkg);
    }

    Ok(vec_pkg)
}

pub fn format(vec_pkg: Vec<PKG>) {
    if vec_pkg.is_empty() {
        println!("Nothing Found");
    } else {
        for pkg in vec_pkg {
            let mut install = String::new();
            if pkg.installed {
                install.push_str("(Installed)")
            }
            println!(
                "{} {}-{} {} \n{}",
                pkg.name, pkg.version, pkg.subversion, install, pkg.description
            )
        }
    }
}
