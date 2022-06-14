use tonic::{transport::Server, Request, Response, Status};

use talk_service::{
    talk_service_server::{TalkService, TalkServiceServer},
    SendMessageRequest, SendMessageReply,
    SendReadReceiptRequest, SendReadReceiptReply,
};

use types::{
    PingRequest, PingReply,
    OpType, Operation,
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
        let reply = talk_service::SendMessageReply{
            revision: 0,
            message: request.into_inner().message
        };
        Ok(Response::new(reply))
    }

    async fn send_read_receipt(&self, _: Request<SendReadReceiptRequest>) -> Result<Response<SendReadReceiptReply>, Status> {
        let reply = talk_service::SendReadReceiptReply{
            revision: 0,
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let provider = TalkServiceProvider::default();

    println!("TalkServiceServer listening on {}", addr);

    Server::builder()
        .add_service(TalkServiceServer::new(provider))
        .serve(addr)
        .await?;

    Ok(())
}