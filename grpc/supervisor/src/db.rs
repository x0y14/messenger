use diesel::{r2d2::{Pool, ConnectionManager, PooledConnection}, pg::PgConnection, Connection, RunQueryDsl};
use crate::models::Profile;
use crate::profiles;
use diesel;
use diesel::prelude::*;
use serde_derive::Deserialize;
use crate::schema::profiles as profiles_schema;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
static DB_URL: &str = "postgres://docker:password@0.0.0.0:5432/messenger_db";

pub fn connect() -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(DB_URL);
    Pool::new(manager).expect("failed to create pool")
}

pub fn get_profile(pool: Pool<ConnectionManager<PgConnection>>, user_id: String) -> String {
    // let pool_clone = pool.clone();
    let cl = pool.get().unwrap();
    let profs = profiles.load::<Profile>(&cl).expect("failed to load profiles");
    println!("<profile");
    for p in profs {
        println!("profile: {}",p.display_name);
    }
    println!("profile>");
    return "".to_string()
}


#[derive(Debug, Insertable, Deserialize)]
#[table_name="profiles_schema"]
pub struct NewProfile {
    pub id: String,
    pub display_name: String,
    pub status_message: String,
    pub icon_path: String
}
// id -> Varchar,
// display_name -> Varchar,
// status_message -> Nullable<Varchar>,
// icon_path -> Nullable<Varchar>,
// created_at -> Timestamptz,
// updated_at -> Timestamptz,


pub fn insert_profile(pool: Pool<ConnectionManager<PgConnection>>, prof: NewProfile) {
    // let pool_clone = pool.clone();
    let cl = pool.get().unwrap();

    diesel::insert_into(profiles)
        .values(&prof).execute(&cl).expect("Failed to insert profile");
}