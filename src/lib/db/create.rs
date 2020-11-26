use crate::lib::db::conn::conn;

pub async fn create() {
    let mut conn = conn().await;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS Packages (
        id                      INTEGER PRIMARY KEY,
        name                    TEXT NOT NULL,
        version                 TEXT NOT NULL,
        subversion              TEXT,
        description             TEXT NOT NULL,
        url                     TEXT,
        packager                TEXT NOT NULL,
        date                    TEXT,
        source                  TEXT,
        license                 TEXT,
        dependence              TEXT NOT NULL,
        architecture            TEXT NOT NULL,
        optional_dependence     TEXT,
        installed               INTEGER NOT NULL,
        file                    TEXT
    )",
    )
    .execute(&mut conn)
    .await
    .expect("Error");
}
