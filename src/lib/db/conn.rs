use sqlx::Connection;
use sqlx::SqliteConnection;

pub async fn conn() -> SqliteConnection {
    SqliteConnection::connect("/etc/sulfur/db.sql").await.expect("Error")
}
