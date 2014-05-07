#![crate_id = "trilogy#0.1.0-pre"]

#![comment = "The best PostgreSQL client in the world"]
#![crate_type = "rlib"]

extern crate postgres;
use postgres::{PostgresConnection, PostgresRows, NoSsl, PostgresResult, PostgresStatement, PostgresRow};

pub struct Column {
    pub name: ~str
}

pub struct Table {
    pub name: ~str,
}

pub struct Connection {
    pub conn: PostgresConnection
}

impl Connection {
    pub fn new(host: ~str, username: ~str, database: ~str) -> Connection {
        let url = format!("postgres://{:s}@{:s}/{:s}", username, host, database);
        let conn = PostgresConnection::connect(url.as_slice(), &NoSsl).unwrap();
        Connection{ conn: conn }
    }

    pub fn databases(&self) -> Vec<~str> {
        let stmt = self.conn.prepare("SELECT datname FROM pg_database WHERE datistemplate='f'").unwrap();
        stmt.query([]).unwrap().map(|row| -> ~str { row[1] }).collect()
    }

    pub fn databases_refactor(&self) -> Vec<~str> {
        match self.query("SELECT datname FROM pg_database WHERE datistemplate='f'") {
            Ok(rows) => rows.move_iter().map(|row| row[1]).collect(),
            Err(error) => fail!("Error when querying: {}", error),
        }
    }

    // pub fn query<'a>(&'a self, sql: &str) -> PostgresRows<'a> {
        // let c      : PostgresConnection                = self.conn;
        // let stmt   : PostgresResult<PostgresStatement> = c.prepare(sql);
        // let ustmt  : PostgresStatement                 = stmt.unwrap();
        // let result : PostgresResult<PostgresRows>      = ustmt.query([]);
        // result.unwrap()
    // }

    pub fn query(&self, sql: &str) -> PostgresResult<Vec<PostgresRow>> {
        self.conn.prepare(sql).and_then(|stmt| {
            stmt.query([])
        }).map(|rows| {
            rows.collect()
        })
    }

    pub fn tables(&self) -> Vec<Table> {
        let stmt = self.conn.prepare("SELECT table_name FROM information_schema.tables WHERE table_schema='public'").unwrap();
        stmt.query([])
            .unwrap()
            .map(|row| -> Table { Table{ name: row[1] } })
            .collect()
    }

    pub fn columns(&self, table: &Table) -> Vec<Column> {
        let sql = format!("SELECT column_name FROM information_schema.columns WHERE table_name='{}'", table.name);
        let stmt = self.conn.prepare(sql).unwrap();
        stmt.query([])
            .unwrap()
            .map(|row| -> Column { Column{ name: row[1] } })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::Connection;

    #[test]
    fn test_databases() {
        let conn = Connection::new("localhost".to_owned(), "trilogy".to_owned(), "postgres".to_owned());
        assert!(conn.databases().len() > 0);
    }

    #[test]
    fn test_tables() {
        let conn = Connection::new("localhost".to_owned(), "trilogy".to_owned(), "postgres".to_owned());
        assert!(conn.tables().len() > 0);
    }

    #[test]
    fn test_columns() {
        let conn = Connection::new("localhost".to_owned(), "trilogy".to_owned(), "postgres".to_owned());
        assert!(conn.columns(conn.tables().get(0)).len() > 0);
    }
}
