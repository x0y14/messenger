use diesel::{pg::PgConnection, RunQueryDsl};
use crate::accounts;
use crate::schema::accounts as accounts_schema;


#[derive(Insertable, Debug)]
#[table_name = "accounts_schema"]
pub struct NewAccount {
    pub id: String,
    pub email: String,
    pub username: Option<String>,
}


pub fn insert_account(conn: &PgConnection, account: NewAccount) {
    let cl = conn.clone();
    diesel::insert_into(accounts)
        .values(&account).execute(cl).expect("failed to insert account");
}