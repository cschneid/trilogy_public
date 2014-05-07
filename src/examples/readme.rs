extern crate trilogy;

use trilogy::Connection;

fn main() {
    let host     = "localhost".to_owned();
    let username = "trilogy".to_owned();
    let database = "postgres".to_owned();

    println!("Connecting to '{}' as user '{}' to database '{}'", host, username, database);
    println!("");

    let conn = Connection::new(host, username, database);

    println!("Databases:");
    println!("=========");
    for database in conn.databases().iter() {
        println!("{}", database);
    }

    println!("");

    println!("Tables:");
    println!("=======");
    let tables = conn.tables();
    for table in tables.iter() {
        println!("{}", table.name);
    }

    println!("");

    let first_table = tables.get(0);
    println!("Columns in Table ({}):", first_table.name);
    println!("=======");

    for column in conn.columns(first_table).iter() {
        println!("{}", column.name)
    }
}
