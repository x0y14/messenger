use diesel::{pg::PgConnection, RunQueryDsl};
use crate::models::Profile;
use crate::profiles;
use diesel;
use serde_derive::Deserialize;
use crate::schema::profiles as profiles_schema;


pub fn get_profile(conn: &PgConnection, user_id: String) -> String {
    let cl = conn.clone();
    let profs = profiles.load::<Profile>(cl).expect("failed to load profiles");
    println!("<profile");
    for p in profs {
        println!("profile: {}", p.display_name);
    }
    println!("profile>");
    return "".to_string();
}


#[derive(Debug, Insertable, Deserialize)]
#[table_name = "profiles_schema"]
pub struct NewProfile {
    pub id: String,
    pub display_name: String,
    pub status_message: String,
    pub icon_path: String,
}


pub fn insert_profile(conn: &PgConnection, prof: NewProfile) {
    let cl = conn.clone();

    diesel::insert_into(profiles)
        .values(&prof).execute(cl).expect("Failed to insert profile");
}