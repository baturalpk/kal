use core::fmt;

use rusqlite::{params, Connection, OpenFlags};

pub struct KalRecord {
    pub year: u32,
    pub ordinal_day: u32,
    pub category: String,
    pub details: Option<String>,
}

impl fmt::Display for KalRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}: {}: {}",
            self.year,
            self.ordinal_day,
            self.category,
            self.details.clone().unwrap_or("<no details>".to_string())
        )
    }
}

pub fn create_connection(url: &str) -> anyhow::Result<Connection> {
    let conn = Connection::open_with_flags(
        url,
        OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_CREATE
            | OpenFlags::SQLITE_OPEN_URI
            | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )?;
    Ok(conn)
}

pub fn init_database(conn: &Connection) -> anyhow::Result<()> {
    conn.execute(
        "CREATE TABLE kal_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            year INTEGER NOT NULL,
            ordinal_day INTEGER NOT NULL,
            category TEXT NOT NULL,
            details TEXT
        );",
        (),
    )?;
    Ok(())
}

pub fn upsert_record(conn: &Connection, r: &KalRecord) -> anyhow::Result<()> {
    conn.execute(
        "INSERT INTO kal_records (year, ordinal_day, category, details) VALUES (?1, ?2, ?3, ?4);",
        params![r.year, r.ordinal_day, r.category, r.details],
    )?;
    Ok(())
}

pub fn query_record_ordinal(
    conn: &Connection,
    year: u32,
    ordinal_day: u32,
) -> anyhow::Result<Vec<KalRecord>> {
    let mut stmt = conn.prepare(
        "SELECT year, ordinal_day, category, details 
        FROM kal_records 
        WHERE year=:y AND ordinal_day=:d;",
    )?;
    let rows = stmt.query_map(
        &[(":y", &year.to_string()), (":d", &ordinal_day.to_string())],
        |row| {
            Ok(KalRecord {
                year: row.get(0)?,
                ordinal_day: row.get(1)?,
                category: row.get(2)?,
                details: row.get(3)?,
            })
        },
    )?;
    Ok(rows.filter_map(|res| res.ok()).collect())
}

pub fn query_records_year(conn: &Connection, year: u32) -> anyhow::Result<Vec<KalRecord>> {
    let mut stmt = conn.prepare(
        "SELECT year, ordinal_day, category, details 
        FROM kal_records 
        WHERE year=:y",
    )?;
    let rows = stmt.query_map(&[(":y", &year.to_string())], |row| {
        Ok(KalRecord {
            year: row.get(0)?,
            ordinal_day: row.get(1)?,
            category: row.get(2)?,
            details: row.get(3)?,
        })
    })?;
    Ok(rows.filter_map(|res| res.ok()).collect())
}

pub fn delete_records_ordinal(
    conn: &Connection,
    year: u32,
    ordinal_day: u32,
) -> anyhow::Result<()> {
    let mut stmt = conn.prepare(
        "DELETE FROM kal_records 
        WHERE year=:y AND ordinal_day=:d;",
    )?;
    stmt.execute(&[(":y", &year), (":d", &ordinal_day)])?;
    Ok(())
}
