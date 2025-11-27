use rusqlite::{params, Connection};
use crate::metrics::Metrics;

pub fn init_db() -> Connection {
    let conn = Connection::open("metrics.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS metrics (
            id INTEGER PRIMARY KEY,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            cpu REAL,
            total_mem INTEGER,
            used_mem INTEGER,
            disk_usage INTEGER
        )",
        [],
    ).unwrap();
    conn
}

pub fn save_metrics(conn: &Connection, m: &Metrics) {
    conn.execute(
        "INSERT INTO metrics (cpu, total_mem, used_mem, disk_usage) VALUES (?1, ?2, ?3, ?4)",
        params![m.cpu_usage, m.total_memory, m.used_memory, m.disk_usage],
    ).unwrap();
}
