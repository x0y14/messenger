use std::env;
use std::fmt::format;
// extern crate dotenv;
use diesel::{r2d2::{Pool, ConnectionManager, PooledConnection}, pg::PgConnection, Connection, RunQueryDsl};
use dotenv::dotenv;
use crate::operations;
use crate::schema::operations as operations_schema;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("not found: DATABASE_URL");
    PgConnection::establish(&db_url)
        .expect(&format!("failed to connect: {}", db_url))
}


#[derive(Insertable, Debug)]
#[table_name="operations_schema"]
pub struct NewOperation {
    pub revision: i64,
    pub op_type: i32,
    pub source: String,
    pub destination: Vec<String>,
}


pub fn insert_op(conn: &PgConnection, op: NewOperation) {
    let cl = conn.clone();
    diesel::insert_into(operations)
        .values(&op).execute(cl).expect("failed to insert op");
}