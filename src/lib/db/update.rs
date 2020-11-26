use super::{conn::conn, query::query_package};

pub async fn update_install(id: i64, install: bool) {
    let mut conn = conn().await;
    let query = format!("UPDATE Packages SET installed={} WHERE id={};", install.to_string(), id);

    sqlx::query(query.as_str()).execute(&mut conn).await.expect("Error");
}

pub async fn update_install_from_name(name: String, install: bool) {
    let pkg = query_package(name).await.expect("Error").clone();
    update_install(pkg.first().unwrap().get_id().unwrap(), install).await;
}