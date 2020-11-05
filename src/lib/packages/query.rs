use crate::lib::db::conn::conn;

pub async fn query(packages: Vec<String>) {
    for package in packages {
        query_package(package).await;
    }
}

async fn query_package(package: String) {
    let mut conn = conn().await;

    sqlx::query("SELECT * FROM Packages WHERE name = ?").bind(package).execute(&mut conn).await.expect("Error");
}
