#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate core;
extern crate serde;

use std::default::Default;

use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;

use chrono::NaiveDate;
use diesel::insertable::ColumnInsertValue::Default as OtherDefault;
use pulsar::{
    message::proto, producer, Error as PulsarError, Pulsar, SerializeMessage, TokioExecutor,
};

mod profiles;

use schema::accounts::dsl::*;
use schema::operations::dsl::*;
use schema::profiles::dsl::*;

mod models;
mod schema;
mod mq;
mod operations;
mod accounts;
mod db;

use tonic::{transport::Server, Request, Response, Status};
use supervisor::{pick_new_id, pick_new_revision};
use supervisor::supervisor_service::{};
use supervisor::types::{};
use crate::mq::Notice;
use crate::operations::{establish_connection, insert_op, NewOperation};
use crate::profiles::{connect, get_profile, insert_profile, NewProfile};


#[derive(Default)]
pub struct SupervisorServiceServer {}

#[tokio::main]
async fn main() {
    // println!("Hello, world!");

    // let conn = connect();
    // get_profile(conn.clone(), "".to_string());
    // insert_profile(
    //     conn.clone(),
    //     NewProfile { id: "this_is_id".to_string(), display_name: "john".to_string(), status_message: "hey".to_string(), icon_path: "pathhh".to_string() })

    // let op_conn = establish_connection();
    // let rev = pick_new_revision().await.expect("");
    // let new_op = NewOperation{
    //     revision: rev,
    //     op_type: 0,
    //     source: "x0y14".to_string(),
    //     destination: vec![]
    // };
    //
    // insert_op(&op_conn, new_op);

    let addr = "pulsar://127.0.0.1:6650";
    let pulsar: Pulsar<_> = Pulsar::builder(addr, TokioExecutor)
        .build().await.expect("failed to connect with pulsar");
    let mut producer = pulsar
        .producer()
        .with_topic("my-topic")
        .with_name("pd1")
        .with_options(producer::ProducerOptions{
            schema: Some(proto::Schema {
                r#type: proto::schema::Type::String as i32,
                ..Default::default()
            }),
            ..Default::default()
        })
        .build()
        .await.expect("");

    producer.send(&Notice{data: "a".to_string()}).await.expect("");

}
