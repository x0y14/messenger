extern crate core;

use tonic::{transport::Server, Request, Response, Status};
use talk::{pick_new_id, pick_new_revision};

use talk_service::{
    talk_service_server::{TalkService, TalkServiceServer},
    SendMessageRequest, SendMessageReply,
    SendReadReceiptRequest, SendReadReceiptReply,
};

use types::{
    PingRequest, PingReply,
    // OpType, Operation,
    MessageType, Message,
};


pub mod talk_service {
    tonic::include_proto!("talk_service");
}

pub mod types {
    tonic::include_proto!("types");
}

#[derive(Default)]
pub struct TalkServiceProvider {}

#[tonic::async_trait]
impl TalkService for TalkServiceProvider {
    async fn ping(&self, _: Request<PingRequest>) -> Result<Response<PingReply>, Status> {
        let reply = types::PingReply {
            message: "hello, this is talk service.".to_string(),
        };
        Ok(Response::new(reply))
    }

    async fn send_message(&self, request: Request<SendMessageRequest>) -> Result<Response<SendMessageReply>, Status> {
        let rev = match pick_new_revision().await {
            Ok(_rev) => {_rev}
            Err(err) => {
                return Err(Status::unknown(err.to_string()))
            }
        };

        let msg_id = match pick_new_id().await {
            Ok(_msg_id) => {_msg_id}
            Err(err) => {
                return Err(Status::unknown(err.to_string()))
            }
        };

        let msg = Message{
            id: msg_id,
            content_type: MessageType::Text as i32,
            from: "from".to_string(),
            to: "to".to_string(),
            metadata: "{}".to_string(),
            text: "placeholder".to_string(),
            created_at: "".to_string(),
            updated_at: "".to_string(),
        };

        let reply = talk_service::SendMessageReply {
            revision: rev,
            message: Some(msg),
        };
        Ok(Response::new(reply))
    }

    async fn send_read_receipt(&self, _: Request<SendReadReceiptRequest>) -> Result<Response<SendReadReceiptReply>, Status> {
        let rev1 = match pick_new_revision().await {
            Ok(_rev) => {_rev}
            Err(err) => {
                return Err(Status::unknown(err.to_string()))
            }
        };

        let reply = talk_service::SendReadReceiptReply {
            revision: rev1,
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50050".parse().unwrap();
    let provider = TalkServiceProvider::default();

    println!("TalkServiceServer listening on {} !", addr);

    Server::builder()
        .add_service(TalkServiceServer::new(provider))
        .serve(addr)
        .await?;

    Ok(())
}