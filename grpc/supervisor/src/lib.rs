#[cfg(test)]
mod db_tests {
    use chrono::{DateTime, Utc};
    use diesel::{PgConnection, r2d2};
    use diesel::r2d2::{ConnectionManager};
    use dotenv::{dotenv, var};

    use connector::db;

    #[test]
    fn connection() {
        // 接続
        dotenv().ok();
        let db_url = var("DATABASE_URL").expect("failed to load DATABASE_URL");

        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool: db::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("failed to create pool");


    }
}