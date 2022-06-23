use diesel::{delete, insert_into, QueryDsl, QueryResult, RunQueryDsl, update};
use diesel::dsl::date;

use crate::db::models::{InputInsertOperation, NewOperation, Operation};
use crate::db::Pool;
use crate::db::schema::operations::dsl::operations;
use crate::util::datetime;


pub fn get_single_operation(pool_clone: Pool, revision: i64) -> Result<Operation, diesel::result::Error> {
    let conn = pool_clone.get().unwrap();
    operations.find(revision).get_result(&conn)
}

pub fn insert_single_operation(pool_clone: Pool, input_operation: InputInsertOperation) -> Result<Operation, diesel::result::Error> {
    let conn = pool_clone.get().unwrap();

    let new_operation = NewOperation {
        revision: input_operation.revision,
        op_type: input_operation.op_type,
        source: input_operation.source,
        destination: input_operation.destination,
        created_at: &datetime::now(),
        updated_at: &datetime::now()
    };

    let res = insert_into(operations).values(&new_operation).get_result(&conn)?;
    Ok(res)
}

pub fn delete_single_operations(pool_clone: Pool, revision: i64) -> Result<usize, diesel::result::Error> {
    let conn = pool_clone.get().unwrap();
    let count = delete(operations.find(revision)).execute(&conn)?;
    Ok(count)
}