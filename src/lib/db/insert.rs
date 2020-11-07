use crate::lib::db::conn::conn;
use crate::lib::packages::install::get_config_of_package;

pub async fn insert(package: String) {
    let mut conn = conn().await;

    let package_info = get_config_of_package(package);
    let query = format!("INSERT INTO Packages (name, version, subversion, description, url, packager, date, license, dependence, architecture, optional_dependence, installed) VALUES(\"{}\", {}, {}, \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", {})",
                            package_info.get_name(),
                            package_info.get_version(),
                            package_info.get_subversion(),
                            package_info.get_description(),
                            package_info.get_url(),
                            package_info.get_packager(),
                            package_info.get_date().to_string(),
                            package_info.get_license().format(),
                            package_info.deps_format(),
                            package_info.get_architecture().format(),
                            package_info.opt_dep_format(),0);

    sqlx::query(query.as_str())
        .execute(&mut conn)
        .await
        .expect("Error");
}
