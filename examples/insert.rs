//
// Rust Firebird Client
//
// Example of insert
//
// You need create a database with this table:
// create table test (col_a int generated by default as identity, col_b float, col_c varchar(10));
//

use rsfbclient::Connection;

const SQL_INSERT: &'static str = "insert into test (col_b, col_c) values (rand(), 'test')";

fn main() {
    let conn = Connection::open(
        "localhost".to_string(),
        3050,
        "examples.fdb".to_string(),
        "SYSDBA".to_string(),
        "masterkey".to_string(),
    )
    .expect("Error on connect");

    let tr = conn
        .start_transaction()
        .expect("Error on start the transaction");

    // First alternative
    {
        tr.execute_immediate(SQL_INSERT.to_string())
            .expect("Error on insert");
    }

    // Second alternative
    {
        let stmt = tr
            .prepare(SQL_INSERT.to_string())
            .expect("Error on prepare the insert");

        stmt.execute_simple()
            .expect("Error on execute the prepared insert");
        stmt.execute_simple()
            .expect("Error on execute the prepared insert");
    }

    tr.commit().expect("Error on commit the transaction");

    conn.close().expect("Error on close the connection");
}