use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod schema;
pub(crate) mod models;
pub(crate) mod profiles;
pub(crate) mod messages;


pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;