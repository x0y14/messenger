use std::f32::consts::E;
use diesel::{delete, insert_into, QueryDsl, RunQueryDsl, update, ExpressionMethods};
use diesel::result::Error;

use crate::db::models::{Account, InputInsertAccount, InputUpdateAccount, NewAccount, UpdateAccount};
use crate::db::Pool;
use crate::db::schema::accounts::dsl::accounts;
use crate::db::schema::accounts::email;
use crate::util::datetime;

pub fn get_single_account(pool_clone: Pool, user_id: String) -> Result<Account, diesel::result::Error> {
    let conn = pool_clone.get().unwrap();
    accounts.find(user_id).get_result(&conn)
}


pub fn get_multiple_account_with_email(pool_clone: Pool, input_email: String) -> Result<Vec<Account>, diesel::result::Error> {
    // todo : test
    let conn = pool_clone.get().unwrap();
    let accos = accounts.filter(email.eq(input_email))
        .limit(1)
        .load::<Account>(&conn)?;

    Ok(accos)
}

// pub fn get


pub fn insert_single_account(pool_clone: Pool, input_account: InputInsertAccount) -> Result<Account, diesel::result::Error> {
    let conn = pool_clone.get().unwrap();

    let new_account = NewAccount {
        id: input_account.id,
        email: input_account.email,
        username: input_account.username,
        created_at: &datetime::now(),
        updated_at: &datetime::now()
    };

    let res = insert_into(accounts).values(&new_account).get_result(&conn)?;
    Ok(res)
}

pub fn update_single_account(pool_clone: Pool, user_id: String, input_account: InputUpdateAccount) -> Result<Account, diesel::result::Error> {
    let conn = pool_clone.get().unwrap();

    let res = update(accounts.find(user_id))
        .set(&UpdateAccount{
            email: input_account.email,
            username: input_account.username,
            updated_at: &datetime::now()
        })
        .get_result(&conn).unwrap();

    Ok(res)
}

pub fn delete_single_account(pool_clone: Pool, user_id: String) -> Result<usize, diesel::result::Error> {
    let conn = pool_clone.get().unwrap();
    let count = delete(accounts.find(user_id)).execute(&conn)?;
    Ok(count)
}