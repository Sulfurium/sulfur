use crate::lib::db::conn::conn;
use crate::lib::packages::pkg_struct::{Architecture, Licenses, PKG};
use futures::stream::TryStreamExt;
use sqlx::Row;
use std::str::FromStr;
use toml::value::Datetime;

pub async fn query_package(package: String) -> std::io::Result<Vec<PKG>> {
    let mut conn = conn().await;
    let mut rows = sqlx::query("SELECT * FROM Packages WHERE name = ?")
        .bind(package)
        .fetch(&mut conn);

    let mut vec_pkg: Vec<PKG> = Vec::new();

    while let Some(row) = rows.try_next().await.expect("Error") {
        let mut pkg = PKG::new();
        pkg.set_id(row.try_get::<i64, &str>("id").expect("Error"));
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
                .unwrap_or(Licenses::NOLICENSE),
        );

        pkg.set_architecture(
            Architecture::from_str(row.try_get::<&str, &str>("architecture").expect("Error"))
                .unwrap_or(Architecture::ANY),
        );

        pkg.set_installed_from_i64(row.try_get::<i64, &str>("installed").expect("Error"))
            .expect("Error");

        pkg.set_files(string_to_vec(
            row.try_get::<&str, &str>("file")
                .expect("Error")
                .to_string(),
        ));
        pkg.set_dependence(string_to_vec(
            row.try_get::<&str, &str>("dependence")
                .expect("Error")
                .to_string(),
        ));
        pkg.set_optional_dependence(string_to_vec(
            row.try_get::<&str, &str>("optional_dependence")
                .expect("Error")
                .to_string(),
        ));

        vec_pkg.push(pkg);
    }

    Ok(vec_pkg)
}

pub fn string_to_vec(string: String) -> Vec<String> {
    let string_replace = string.replace("[", "").replace("]", "");
    let mut result: Vec<String> = Vec::new();
    let split: Vec<&str> = string_replace.split(",").collect();

    for v in split {
        result.push(v.to_string());
    }

    result
}
