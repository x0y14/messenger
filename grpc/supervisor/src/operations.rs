use diesel::{pg::PgConnection, RunQueryDsl};
use crate::operations;
use crate::schema::operations as operations_schema;


#[derive(Insertable, Debug)]
#[table_name = "operations_schema"]
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