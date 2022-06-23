use diesel::{update, delete, insert_into, QueryDsl, RunQueryDsl, QueryResult};

use crate::db::models::{InputInsertProfile, InputUpdateProfile, NewProfile, Profile, UpdateProfile};
use crate::db::Pool;
use crate::db::schema::profiles::dsl::profiles;
use crate::util::datetime;


pub fn get_single_profile(pool_clone: Pool, user_id: String) -> Result<Profile, diesel::result::Error> {
    let conn = pool_clone.get().unwrap();
    profiles.find(user_id).get_result::<Profile>(&conn)
}

pub fn insert_single_profile(pool_clone: Pool, input_profile: InputInsertProfile) -> Result<Profile, diesel::result::Error> {
    let conn = pool_clone.get().unwrap();

    let new_profile = NewProfile {
        id: input_profile.id,
        display_name: input_profile.display_name,
        status_message: input_profile.status_message,
        icon_path: input_profile.icon_path,
        created_at: &datetime::now(),
        updated_at: &datetime::now()
    };

    let res = insert_into(profiles).values(&new_profile).get_result(&conn)?;
    Ok(res)
}

pub fn update_single_profile(pool_clone: Pool, user_id: String, input_profile: InputUpdateProfile) -> Result<Profile, diesel::result::Error> {
    let conn = pool_clone.get().unwrap();

    let res = update(profiles.find(user_id))
        .set(&UpdateProfile{
            display_name: input_profile.display_name,
            status_message: input_profile.status_message,
            icon_path: input_profile.icon_path,
            updated_at: &datetime::now()
        })
        .get_result(&conn).unwrap();

    Ok(res)
}

pub fn delete_single_profile(pool_clone: Pool, user_id: String) -> Result<usize, diesel::result::Error> {
    let conn = pool_clone.get().unwrap();
    let count = delete(profiles.find(user_id)).execute(&conn)?;
    Ok(count)
}