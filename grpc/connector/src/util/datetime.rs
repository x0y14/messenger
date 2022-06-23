use chrono::{DateTime, Utc};

#[cfg(not(test))]
pub fn now() -> DateTime<Utc> {
    Utc::now()
}

#[cfg(test)]
pub mod mock_time {
    use super::*;
    use std::cell::RefCell;

    thread_local! {
        static MOCK_TIME: RefCell<Option<DateTime<Utc>>> = RefCell::new(None);
    }

    pub fn now() -> DateTime<Utc> {
        MOCK_TIME.with(|cell| {
            cell.borrow()
                .as_ref()
                .cloned()
                .unwrap_or_else(|| Utc::now())
        })
    }

    pub fn set_mock_time(time: DateTime<Utc>) {
        MOCK_TIME.with(|cell| *cell.borrow_mut() = Some(time));
    }

    pub fn clear_mock_time() {
        MOCK_TIME.with(|cell| *cell.borrow_mut() = None);
    }
}

#[cfg(test)]
pub use mock_time::now;

#[cfg(test)]
mod tests {
    use crate::util::datetime::mock_time::set_mock_time;
    use super::*;

    #[test]
    fn now_ok() {
        let dt = String::from("2020-02-01T00:00:00+00:00");
        
        let datetime = DateTime::parse_from_rfc3339(&*dt).unwrap();
        let datetime_utc = datetime.with_timezone(&Utc);
        set_mock_time(datetime_utc);

        assert_eq!(now().to_rfc3339(), dt);
    }
}