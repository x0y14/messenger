use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod schema;
pub mod models;
pub mod profiles;
pub mod messages;
pub mod accounts;
pub mod operations;


pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;