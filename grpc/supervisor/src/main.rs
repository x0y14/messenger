extern crate core;

use chrono::{DateTime, FixedOffset, Utc};
use connector::db::{accounts};
use connector::idgen::{get_new_id, get_new_revision};
mod lib;

use tonic::{transport::Server, Request, Response, Status};
use connector::db;

use diesel::{PgConnection, r2d2};
use diesel::r2d2::{ConnectionManager};
use dotenv::{dotenv, var};

use crate::db::models::{InputInsertAccount, };
use crate::db::accounts::{get_multiple_account_with_email, get_single_account, insert_single_account, update_single_account};

pub mod supervisor_service { tonic::include_proto!("supervisor_service"); }
use supervisor_service::{
    supervisor_service_server::{SupervisorService, SupervisorServiceServer},
    CreateAccountRequest, CreateAccountReply,
    CreateProfileRequest, CreateProfileReply,
    RecordOperationRequest, RecordOperationReply
};

pub mod types { tonic::include_proto!("types"); }
use types::{
    PingRequest, PingReply,
    Account,
    Profile,
    Operation, OpType
};

pub struct SupervisorServiceProvider {
    pool: db::Pool
}


#[tonic::async_trait]
impl SupervisorService for SupervisorServiceProvider {
    async fn ping(&self, _: Request<PingRequest>) -> Result<Response<PingReply>, Status> {
        let now_jst: DateTime<FixedOffset> = Utc::now().with_timezone(&FixedOffset::east(9 *3600));
        let reply = types::PingReply {
            message: now_jst.to_rfc3339(),
        };
        Ok(Response::new(reply))
    }

    async fn create_account(&self, request: Request<CreateAccountRequest>) -> Result<Response<CreateAccountReply>, Status> {
        let acc: Account = request.into_inner().account.unwrap();

        // id生成
        let user_id = get_new_id().await.unwrap();
        let revision = get_new_revision().await.unwrap();
        
        let res = match insert_single_account(self.pool.clone(), InputInsertAccount {
            id: &user_id,
            email: &acc.email,
            username: if acc.username == "" {None} else {Some(&acc.username)},
        }) {
            Ok(r) => r,
            Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, ..)) => {
                return Err(Status::already_exists("the user using same email or username already exists"))
            }
            Err(e) => {
                return Err(Status::unknown(format!("{:?}", e)))
            }
        };


        Ok(Response::new(CreateAccountReply{ revision: revision.parse().unwrap(), account: Some(types::Account{
            id: res.id,
            email: res.email,
            username: if res.username == None { "".to_string() } else { res.username.unwrap() },
            created_at: res.created_at.to_rfc3339(),
            updated_at: res.updated_at.to_rfc3339()
        }) }))
    }

    async fn create_profile(&self, request: Request<CreateProfileRequest>) -> Result<Response<CreateProfileReply>, Status> {
        todo!()
    }

    async fn record_operation(&self, request: Request<RecordOperationRequest>) -> Result<Response<RecordOperationReply>, Status> {
        todo!()
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50060".parse().unwrap();

    // 接続
    dotenv().ok();
    let db_url = var("DATABASE_URL").expect("failed to load DATABASE_URL");

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool: db::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("failed to create pool");

    let provider = SupervisorServiceProvider{pool: pool};

    println!("supervisor service listening on {} !", addr);

    Server::builder()
        .add_service(SupervisorServiceServer::new(provider))
        .serve(addr)
        .await?;

    Ok(())
}
