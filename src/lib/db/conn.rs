use rusqlite::Connection;

pub fn conn() -> Connection {
    Connection::open("/etc/sulfur/db.sql").expect("Error")
}
