mod idgen;
mod db;
mod util;

#[macro_use]
extern crate diesel;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[cfg(test)]
mod profiles_tests {
    use chrono::{DateTime, Utc};
    use diesel::{PgConnection, r2d2};
    use diesel::r2d2::{ConnectionManager};
    use dotenv::{dotenv, var};
    use crate::db;
    use crate::db::models::InputInsertProfile;
    use crate::util::datetime::mock_time::set_mock_time;

    #[test]
    fn insert_one() {
        // 接続
        dotenv().ok();
        let db_url = var("DATABASE_URL").expect("failed to load DATABASE_URL");

        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool: db::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("failed to create pool");

        // データ準備
        let id = &"test_id".to_string();
        let display_name = &"testUser".to_string();
        let status_message = &"hello".to_string();

        let new_profile = InputInsertProfile{
            id,
            display_name,
            status_message: Some(status_message),
            icon_path: None
        };

        let now_ = DateTime::parse_from_rfc3339("2020-02-01T00:00:00+00:00").unwrap().with_timezone(&Utc);

        // 時間設定
        set_mock_time(now_);
        let result_prof = db::profiles::insert_single_profile(pool.clone(), new_profile).expect("Failed to insert profile");

        assert_eq!(*id, result_prof.id);
        assert_eq!(*display_name, result_prof.display_name);
        assert_eq!(*status_message, result_prof.status_message.unwrap());
        assert_eq!(None, result_prof.icon_path);
        assert_eq!(now_, result_prof.created_at);
        assert_eq!(now_, result_prof.updated_at);

    }
    fn get_one() {}
    fn delete_one() {}
}