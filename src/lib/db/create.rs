use crate::lib::db::conn::conn;
use rusqlite::params;

pub fn create() {
    let conn = conn();

    conn.execute("
    CREATE TABLE IF NOT EXISTS Packages (
        id                      INTEGER PRIMARY KEY,
        name                    TEXT NOT NULL,
        version                 TEXT NOT NULL,
        subversion              TEXT,
        description             TEXT NOT NULL,
        url                     TEXT,
        packager                TEXT NOT NULL,
        date                    DATE,
        license                 TEXT,
        dependence              TEXT NOT NULL,
        architecture            TEXT NOT NULL,
        optional_dependence     TEXT
    )", params![]).expect("Error");
}
