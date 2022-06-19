#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate core;

use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;

use chrono::NaiveDate;

mod db;

use schema::accounts::dsl::*;
use schema::operations::dsl::*;
use schema::profiles::dsl::*;

mod models;
mod schema;

use tonic::{transport::Server, Request, Response, Status};
use supervisor::supervisor_service::{};
use supervisor::types::{};
use crate::db::{connect, get_profile, insert_profile, NewProfile};


#[derive(Default)]
pub struct SupervisorServiceServer {}

#[tokio::main]
async fn main() {
    // println!("Hello, world!");
    let conn = connect();
    get_profile(conn.clone(), "".to_string());
    println!("done!");
    insert_profile(
        conn.clone(),
        NewProfile { id: "this_is_id".to_string(), display_name: "john".to_string(), status_message: "hey".to_string(), icon_path: "pathhh".to_string() })
    // return Ok(())
}
