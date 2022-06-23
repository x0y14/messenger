use connector::db::{accounts};
use connector::idgen::{get_new_id};
mod lib;

use tonic::{transport::Server, Request, Response, Status};
use connector::db;
use connector::db::accounts::get_multiple_account_with_email;
use connector::db::models::{InputInsertAccount};

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
        let reply = types::PingReply {
            message: "".to_string(),
        };
        Ok(Response::new(reply))
    }

    async fn create_account(&self, request: Request<CreateAccountRequest>) -> Result<Response<CreateAccountReply>, Status> {
        // リクエストに含まれるemailをもつアカウントが存在しないことを確認
        let acc: types::Account = request.into_inner().account.unwrap();
        
        let accounts_ = get_multiple_account_with_email(
            self.pool.clone(), acc.email).unwrap();
        // 存在している
        if accounts_.len() >= 1 {
            Err(Status::already_exists("this email is already used."))?
        }

        // id生成
        let user_id = get_new_id().await.unwrap();

        // let new_account = InputInsertAccount {
        //     id: &user_id,
        //     email: &acc.email,
        //     username: acc.username
        // }

        // Ok(Response::new(()))

        Err(Status::unimplemented(""))
    }
}


// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // let provider = SupervisorServiceProvider{}
// }
fn main() {}