use std::env;
use diesel::{pg::PgConnection, Connection};
use dotenv::dotenv;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("not found: DATABASE_URL");
    PgConnection::establish(&db_url)
        .expect(&format!("failed to connect: {}", db_url))
}