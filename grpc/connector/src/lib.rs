pub mod idgen;
pub mod db;
mod util;

#[macro_use]
extern crate diesel;
extern crate core;

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
    use crate::db::models::{InputInsertProfile, InputUpdateProfile};
    use crate::db::profiles::{delete_single_profile, get_single_profile, update_single_profile};
    use crate::util::datetime::mock_time::set_mock_time;

    static TEST_USER_ID: &str = "test_id";
    static TEST_DISPLAY_NAME: &str = "testUser";
    static TEST_STATUS_MESSAGE: &str = "hello";
    static TEST_CREATED_AT: &str = "2020-02-01T00:00:00+00:00";

    static TEST_ICON_PATH: &str = "foo_bar";
    static TEST_UPDATED_AT: &str = "2020-02-02T00:00:00+00:00";


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
        let id = &TEST_USER_ID.to_string();
        let display_name = &TEST_DISPLAY_NAME.to_string();
        let status_message = &TEST_STATUS_MESSAGE.to_string();

        let new_profile = InputInsertProfile{
            id,
            display_name,
            status_message: Some(status_message),
            icon_path: None
        };

        // 時間設定
        let now_ = DateTime::parse_from_rfc3339(&TEST_CREATED_AT).unwrap().with_timezone(&Utc);
        set_mock_time(now_);

        // データ挿入
        let result_prof = db::profiles::insert_single_profile(pool.clone(), new_profile).expect("Failed to insert profile");

        // 比較
        assert_eq!(*id, result_prof.id);
        assert_eq!(*display_name, result_prof.display_name);
        assert_eq!(*status_message, result_prof.status_message.unwrap());
        assert_eq!(None, result_prof.icon_path);
        assert_eq!(now_, result_prof.created_at); // アップデートされてなければ、同じものが入っている
        assert_eq!(now_, result_prof.updated_at); //

    }

    #[test]
    fn get_one_before_update() {
        // 接続
        dotenv().ok();
        let db_url = var("DATABASE_URL").expect("failed to load DATABASE_URL");

        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool: db::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("failed to create pool");

        // get
        let prof = get_single_profile(pool.clone(), TEST_USER_ID.to_string()).expect("failed to get profile");

        // 比較
        assert_eq!(TEST_USER_ID.to_string(), prof.id);
        assert_eq!(TEST_DISPLAY_NAME.to_string(), prof.display_name);
        assert_eq!(TEST_STATUS_MESSAGE.to_string(), prof.status_message.unwrap());
        assert_eq!(None, prof.icon_path);
        assert_eq!(DateTime::parse_from_rfc3339(&TEST_CREATED_AT).unwrap().with_timezone(&Utc), prof.created_at);
        assert_eq!(DateTime::parse_from_rfc3339(&TEST_CREATED_AT).unwrap().with_timezone(&Utc), prof.updated_at);
    }

    #[test]
    fn update_one() {
        // 接続
        dotenv().ok();
        let db_url = var("DATABASE_URL").expect("failed to load DATABASE_URL");

        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool: db::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("failed to create pool");

        // 時間設定
        let now_ = DateTime::parse_from_rfc3339(&TEST_UPDATED_AT).unwrap().with_timezone(&Utc);
        set_mock_time(now_);

        let prof = update_single_profile(pool.clone(), TEST_USER_ID.to_string(), InputUpdateProfile{
            display_name: None,
            status_message: None,
            icon_path: Some(&TEST_ICON_PATH.to_string())
        }).expect("failed to update profile");

        // 比較
        assert_eq!(TEST_USER_ID.to_string(), prof.id);
        assert_eq!(TEST_DISPLAY_NAME.to_string(), prof.display_name);
        assert_eq!(TEST_STATUS_MESSAGE.to_string(), prof.status_message.unwrap());
        assert_eq!(TEST_ICON_PATH.to_string(), prof.icon_path.unwrap());
        assert_eq!(DateTime::parse_from_rfc3339(&TEST_CREATED_AT).unwrap().with_timezone(&Utc), prof.created_at);
        assert_eq!(DateTime::parse_from_rfc3339(&TEST_UPDATED_AT).unwrap().with_timezone(&Utc), prof.updated_at);
    }

    #[test]
    fn get_one_after_update() {
        // 接続
        dotenv().ok();
        let db_url = var("DATABASE_URL").expect("failed to load DATABASE_URL");

        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool: db::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("failed to create pool");

        // get
        let prof = get_single_profile(pool.clone(), TEST_USER_ID.to_string()).expect("failed to get profile");

        // 比較
        assert_eq!(TEST_USER_ID.to_string(), prof.id);
        assert_eq!(TEST_DISPLAY_NAME.to_string(), prof.display_name);
        assert_eq!(TEST_STATUS_MESSAGE.to_string(), prof.status_message.unwrap());
        assert_eq!(TEST_ICON_PATH.to_string(), prof.icon_path.unwrap());
        assert_eq!(DateTime::parse_from_rfc3339(&TEST_CREATED_AT).unwrap().with_timezone(&Utc), prof.created_at);
        assert_eq!(DateTime::parse_from_rfc3339(&TEST_UPDATED_AT).unwrap().with_timezone(&Utc), prof.updated_at);
    }

    #[test]
    fn delete_one() {
        // 接続
        dotenv().ok();
        let db_url = var("DATABASE_URL").expect("failed to load DATABASE_URL");

        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool: db::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("failed to create pool");

        // idはpkなので多くて1件
        let count = delete_single_profile(pool.clone(), TEST_USER_ID.to_string()).unwrap();
        assert_eq!(count, 1);
    }
}

#[cfg(test)]
mod messages_test {
    use chrono::{DateTime, Utc};
    use diesel::{PgConnection, r2d2};
    use diesel::r2d2::{ConnectionManager};
    use dotenv::{dotenv, var};

    use crate::db;
    use crate::db::models::InputInsertMessage;
    use crate::db::messages::{get_single_message, insert_single_message};
    use crate::util::datetime::mock_time::set_mock_time;

    static TEST_MSG_ID: &str = "test_msg_id";
    static TEST_MSG_FROM: &str = "from";
    static TEST_MSG_TO: &str = "to";
    static TEST_MSG_CONTENT_TYPE: i32 = 0;
    static TEST_MSG_TEXT: &str = "hello,world";
    static TEST_MSG_CREATED_UPDATED_AT: &str = "2020-03-02T00:00:00+00:00";


    #[test]
    fn insert_one() {
        // 接続
        dotenv().ok();
        let db_url = var("DATABASE_URL").expect("failed to load DATABASE_URL");

        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool: db::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("failed to create pool");

        // データ作成
        let test_msg_metadata: serde_json::Value = serde_json::from_str("{\"kind\": 1}").unwrap();
        let msg = InputInsertMessage {
            id: &TEST_MSG_ID.to_string(),
            from: &TEST_MSG_FROM.to_string(),
            to: &TEST_MSG_TO.to_string(),
            content_type: &TEST_MSG_CONTENT_TYPE,
            metadata: &test_msg_metadata,
            text: &TEST_MSG_TEXT.to_string(),
        };

        // 時間設定
        let now_ = DateTime::parse_from_rfc3339(&TEST_MSG_CREATED_UPDATED_AT).unwrap().with_timezone(&Utc);
        set_mock_time(now_);

        // 挿入
        let res = insert_single_message(pool.clone(), msg).expect("failed to insert message");

        // 検証
        assert_eq!(TEST_MSG_ID.to_string(), res.id);
        assert_eq!(TEST_MSG_FROM.to_string(), res.from);
        assert_eq!(TEST_MSG_TO.to_string(), res.to);
        assert_eq!(TEST_MSG_CONTENT_TYPE, res.content_type);
        assert_eq!(test_msg_metadata, res.metadata);
        assert_eq!(TEST_MSG_TEXT.to_string(), res.text);
        assert_eq!(now_, res.created_at);
        assert_eq!(now_, res.updated_at);
    }

    #[test]
    fn get_one() {
        // 接続
        dotenv().ok();
        let db_url = var("DATABASE_URL").expect("failed to load DATABASE_URL");

        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool: db::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("failed to create pool");

        let test_msg_metadata: serde_json::Value = serde_json::from_str("{\"kind\": 1}").unwrap();
        let msg = get_single_message(pool.clone(), TEST_MSG_ID.to_string()).expect("failed to get message");

        let now_ = DateTime::parse_from_rfc3339(&TEST_MSG_CREATED_UPDATED_AT).unwrap().with_timezone(&Utc);

        // 検証
        assert_eq!(TEST_MSG_ID.to_string(), msg.id);
        assert_eq!(TEST_MSG_FROM.to_string(), msg.from);
        assert_eq!(TEST_MSG_TO.to_string(), msg.to);
        assert_eq!(TEST_MSG_CONTENT_TYPE, msg.content_type);
        assert_eq!(test_msg_metadata, msg.metadata);
        assert_eq!(TEST_MSG_TEXT.to_string(), msg.text);
        assert_eq!(now_, msg.created_at);
        assert_eq!(now_, msg.updated_at);
    }
}

#[cfg(test)]
mod accounts_tests {
    use chrono::{DateTime, Utc};
    use diesel::{PgConnection, r2d2};
    use diesel::r2d2::{ConnectionManager};
    use dotenv::{dotenv, var};

    use crate::db;
    use crate::db::models::{InputInsertAccount, InputUpdateAccount};
    use crate::db::accounts::{delete_single_account, get_multiple_account_with_email, get_single_account, insert_single_account, update_single_account};
    use crate::util::datetime::mock_time::set_mock_time;

    static TEST_USER_ID: &str = "test_id";
    static TEST_EMAIL: &str = "example@example.example";
    static TEST_CREATED_AT: &str = "2020-02-01T00:00:00+00:00";

    static TEST_USER_NAME: &str = "test_handle_name";
    static TEST_UPDATED_AT: &str = "2020-04-02T00:00:00+00:00";

    #[test]
    fn insert_one() {
        // 接続
        dotenv().ok();
        let db_url = var("DATABASE_URL").expect("failed to load DATABASE_URL");

        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool: db::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("failed to create pool");

        // データ生成
        let new_account = InputInsertAccount{
            id: &TEST_USER_ID.to_string(),
            email: &TEST_EMAIL.to_string(),
            username: None
        };

        // 時間設定
        let now_ = DateTime::parse_from_rfc3339(&TEST_CREATED_AT).unwrap().with_timezone(&Utc);
        set_mock_time(now_);

        let res = insert_single_account(pool.clone(), new_account).expect("failed to insert account");

        assert_eq!(TEST_USER_ID.to_string(), res.id);
        assert_eq!(TEST_EMAIL.to_string(), res.email);
        assert_eq!(None, res.username);
        assert_eq!(now_, res.created_at);
        assert_eq!(now_, res.updated_at);
    }

    #[test]
    fn get_one_before_update() {
        // 接続
        dotenv().ok();
        let db_url = var("DATABASE_URL").expect("failed to load DATABASE_URL");

        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool: db::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("failed to create pool");

        let now_ = DateTime::parse_from_rfc3339(&TEST_CREATED_AT).unwrap().with_timezone(&Utc);
        let acc = get_single_account(pool.clone(), TEST_USER_ID.to_string()).expect("failed to get account");

        assert_eq!(TEST_USER_ID.to_string(), acc.id);
        assert_eq!(TEST_EMAIL.to_string(), acc.email);
        assert_eq!(None, acc.username);
        assert_eq!(now_, acc.created_at);
        assert_eq!(now_, acc.updated_at);
    }

    #[test]
    fn get_some_with_email_before_update() {
        // 接続
        dotenv().ok();
        let db_url = var("DATABASE_URL").expect("failed to load DATABASE_URL");

        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool: db::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("failed to create pool");

        let now_ = DateTime::parse_from_rfc3339(&TEST_CREATED_AT).unwrap().with_timezone(&Utc);
        let accs = get_multiple_account_with_email(pool.clone(), TEST_EMAIL.to_string()).expect("failed to get account");

        assert_eq!(accs.len(), 1);

        let acc = accs.first().unwrap();

        assert_eq!(TEST_USER_ID.to_string(), acc.id);
        assert_eq!(TEST_EMAIL.to_string(), acc.email);
        assert_eq!(None, acc.username);
        assert_eq!(now_, acc.created_at);
        assert_eq!(now_, acc.updated_at);
    }

    #[test]
    fn update_one() {
        // 接続
        dotenv().ok();
        let db_url = var("DATABASE_URL").expect("failed to load DATABASE_URL");

        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool: db::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("failed to create pool");

        let now_ = DateTime::parse_from_rfc3339(&TEST_UPDATED_AT).unwrap().with_timezone(&Utc);
        set_mock_time(now_);

        let res = update_single_account(pool.clone(), TEST_USER_ID.to_string(),
                                        InputUpdateAccount{ email: None, username: Some(&TEST_USER_NAME.to_string()) }).expect("failed to update account");

        assert_eq!(TEST_USER_ID.to_string(), res.id);
        assert_eq!(TEST_EMAIL.to_string(), res.email);
        assert_eq!(TEST_USER_NAME.to_string(), res.username.unwrap());
        assert_eq!(DateTime::parse_from_rfc3339(&TEST_CREATED_AT).unwrap().with_timezone(&Utc), res.created_at);
        assert_eq!(DateTime::parse_from_rfc3339(&TEST_UPDATED_AT).unwrap().with_timezone(&Utc), res.updated_at);

    }

    #[test]
    fn get_one_after_update() {
        // 接続
        dotenv().ok();
        let db_url = var("DATABASE_URL").expect("failed to load DATABASE_URL");

        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool: db::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("failed to create pool");

        let acc = get_single_account(pool.clone(), TEST_USER_ID.to_string()).expect("failed to get account");

        assert_eq!(TEST_USER_ID.to_string(), acc.id);
        assert_eq!(TEST_EMAIL.to_string(), acc.email);
        assert_eq!(TEST_USER_ID, acc.username.unwrap());
        assert_eq!(DateTime::parse_from_rfc3339(&TEST_CREATED_AT).unwrap().with_timezone(&Utc) , acc.created_at);
        assert_eq!(DateTime::parse_from_rfc3339(&TEST_UPDATED_AT).unwrap().with_timezone(&Utc) , acc.updated_at);
    }

    #[test]
    fn delete_one() {
        // 接続
        dotenv().ok();
        let db_url = var("DATABASE_URL").expect("failed to load DATABASE_URL");

        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool: db::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("failed to create pool");

        let count = delete_single_account(pool.clone(), TEST_USER_ID.to_string()).expect("failed to delete account");
        assert_eq!(count, 1);
    }
}

#[cfg(test)]
mod operations_tests {
    use chrono::{DateTime, Utc};
    use diesel::{PgConnection, r2d2};
    use diesel::r2d2::{ConnectionManager};
    use dotenv::{dotenv, var};

    use crate::db;
    use crate::db::models::{InputInsertAccount, InputInsertOperation, InputUpdateAccount};
    use crate::db::operations::{delete_single_operations, get_single_operation, insert_single_operation};
    use crate::util::datetime::mock_time::set_mock_time;
    use crate::util::datetime::now;

    // pub revision: &'a i64,
    // pub op_type: &'a i32,
    // pub source: &'a String,
    // pub destination: &'a Vec<String>,
    // pub created_at: &'a DateTime<Utc>,
    // pub updated_at: &'a DateTime<Utc>,

    static TEST_REVISION: i64 = 12;
    static TEST_OP_TYPE: i32 = 1;
    static TEST_SOURCE: &str = "userA";
    static TEST_DEST_01: &str = "userB";
    static TEST_DEST_02: &str = "userC";

    static TEST_CREATED_UPDATED_AT: &str = "2020-02-01T00:00:00+00:00";

    #[test]
    fn insert_one() {
        // 接続
        dotenv().ok();
        let db_url = var("DATABASE_URL").expect("failed to load DATABASE_URL");

        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool: db::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("failed to create pool");

        let new_operation = InputInsertOperation {
            revision: &TEST_REVISION,
            op_type: &TEST_OP_TYPE,
            source: &TEST_SOURCE.to_string(),
            destination: &vec![TEST_DEST_01.to_string(), TEST_DEST_02.to_string()]
        };

        // 時間設定
        let now_ = DateTime::parse_from_rfc3339(&TEST_CREATED_UPDATED_AT).unwrap().with_timezone(&Utc);
        set_mock_time(now_);

        let res = insert_single_operation(pool.clone(), new_operation).expect("failed to insert operation");

        assert_eq!(TEST_REVISION, res.revision);
        assert_eq!(TEST_OP_TYPE, res.op_type);
        assert_eq!(TEST_SOURCE, res.source);
        assert_eq!(vec![TEST_DEST_01.to_string(), TEST_DEST_02.to_string()], res.destination);
        assert_eq!(now_, res.created_at);
        assert_eq!(now_, res.updated_at);
    }

    #[test]
    fn get_one() {
        // 接続
        dotenv().ok();
        let db_url = var("DATABASE_URL").expect("failed to load DATABASE_URL");

        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool: db::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("failed to create pool");

        let op = get_single_operation(pool.clone(), TEST_REVISION).expect("failed to get operation");

        let now_ = DateTime::parse_from_rfc3339(&TEST_CREATED_UPDATED_AT).unwrap().with_timezone(&Utc);

        assert_eq!(TEST_REVISION, op.revision);
        assert_eq!(TEST_OP_TYPE, op.op_type);
        assert_eq!(TEST_SOURCE, op.source);
        assert_eq!(vec![TEST_DEST_01.to_string(), TEST_DEST_02.to_string()], op.destination);
        assert_eq!(now_, op.created_at);
        assert_eq!(now_, op.updated_at);
    }

    #[test]
    fn delete_one() {
        let db_url = var("DATABASE_URL").expect("failed to load DATABASE_URL");

        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool: db::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("failed to create pool");

        let count = delete_single_operations(pool.clone(), TEST_REVISION).expect("failed to delete operation");

        assert_eq!(count, 1);
    }
}