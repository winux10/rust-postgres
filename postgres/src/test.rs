use tokio_postgres::types::Type;
use tokio_postgres::NoTls;

use super::*;

#[test]
fn prepare() {
    let mut client = Client::connect("host=localhost port=5433 user=postgres", NoTls).unwrap();

    let stmt = client.prepare("SELECT 1::INT, $1::TEXT").unwrap();
    assert_eq!(stmt.params(), &[Type::TEXT]);
    assert_eq!(stmt.columns().len(), 2);
    assert_eq!(stmt.columns()[0].type_(), &Type::INT4);
    assert_eq!(stmt.columns()[1].type_(), &Type::TEXT);
}

#[test]
fn query_prepared() {
    let mut client = Client::connect("host=localhost port=5433 user=postgres", NoTls).unwrap();

    let stmt = client.prepare("SELECT $1::TEXT").unwrap();
    let rows = client.query(&stmt, &[&"hello"]).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].get::<_, &str>(0), "hello");
}

#[test]
fn query_unprepared() {
    let mut client = Client::connect("host=localhost port=5433 user=postgres", NoTls).unwrap();

    let rows = client.query("SELECT $1::TEXT", &[&"hello"]).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].get::<_, &str>(0), "hello");
}

#[test]
fn transaction_commit() {
    let mut client = Client::connect("host=localhost port=5433 user=postgres", NoTls).unwrap();

    client
        .batch_execute("CREATE TEMPORARY TABLE foo (id SERIAL PRIMARY KEY)")
        .unwrap();

    let mut transaction = client.transaction().unwrap();

    transaction
        .execute("INSERT INTO foo DEFAULT VALUES", &[])
        .unwrap();

    transaction.commit().unwrap();

    let rows = client.query("SELECT * FROM foo", &[]).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].get::<_, i32>(0), 1);
}

#[test]
fn transaction_rollback() {
    let mut client = Client::connect("host=localhost port=5433 user=postgres", NoTls).unwrap();

    client
        .batch_execute("CREATE TEMPORARY TABLE foo (id SERIAL PRIMARY KEY)")
        .unwrap();

    let mut transaction = client.transaction().unwrap();

    transaction
        .execute("INSERT INTO foo DEFAULT VALUES", &[])
        .unwrap();

    transaction.rollback().unwrap();

    let rows = client.query("SELECT * FROM foo", &[]).unwrap();
    assert_eq!(rows.len(), 0);
}

#[test]
fn transaction_drop() {
    let mut client = Client::connect("host=localhost port=5433 user=postgres", NoTls).unwrap();

    client
        .batch_execute("CREATE TEMPORARY TABLE foo (id SERIAL PRIMARY KEY)")
        .unwrap();

    let mut transaction = client.transaction().unwrap();

    transaction
        .execute("INSERT INTO foo DEFAULT VALUES", &[])
        .unwrap();

    drop(transaction);

    let rows = client.query("SELECT * FROM foo", &[]).unwrap();
    assert_eq!(rows.len(), 0);
}