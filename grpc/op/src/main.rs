use futures::Stream;
use std::{error::Error, net::ToSocketAddrs, pin::Pin, time::Duration};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{transport::Server, Request, Response, Status};


pub mod operation_service {
    tonic::include_proto!("operation_service");
}

use operation_service::{
    operation_service_server::{OperationService, OperationServiceServer},
    FetchOpsRequest, FetchOpsReply,
};

pub mod types {
    tonic::include_proto!("types");
}

use types::{
    PingRequest, PingReply,
    OpType, Operation,
    Message, MessageType
};

// no Default?
#[derive(Debug)]
pub struct OperationServiceProvider {}

type FetchOpsResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item=Result<FetchOpsReply, Status>> + Send>>;

#[tonic::async_trait]
impl OperationService for OperationServiceProvider {
    async fn ping(&self, _: Request<PingRequest>) -> Result<Response<PingReply>, Status> {
        let reply = types::PingReply {
            message: "hello, this is operation service.".to_string(),
        };
        Ok(Response::new(reply))
    }

    type FetchOpsStream = ResponseStream;
    async fn fetch_ops(&self, request: Request<FetchOpsRequest>) -> FetchOpsResult<Self::FetchOpsStream> {
        println!("fetch_ops");
        println!("\tconnected: {:?}", request.remote_addr());

        let msg = Message{
            id: "".to_string(),
            content_type: MessageType::Text as i32,
            from: "from".to_string(),
            to: "to".to_string(),
            metadata: "{}".to_string(),
            text: "placeholder".to_string(),
            created_at: "".to_string(),
            updated_at: "".to_string(),
        };

        let repeat = std::iter::repeat(FetchOpsReply {
            op: Some(Operation {
                id: 0,
                op_type: OpType::Noop as i32,
                source: "".to_string(),
                dest: vec!["".to_string()],
            }),
            message: Some(msg),
        });
        let mut stream = Box::pin(tokio_stream::iter(repeat).throttle(Duration::from_millis(200)));

        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            while let Some(item) = stream.next().await {
                match tx.send(Result::<_, Status>::Ok(item)).await {
                    Ok(_) => {
                        // queued
                        println!("\t- op queued!")
                    }
                    Err(_item) => {
                        break;
                    }
                }
            }
            println!("\tdisconnected");
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::FetchOpsStream
        ))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = OperationServiceProvider{};
    let addr = "[::1]:50051".to_socket_addrs().unwrap().next().unwrap();
    println!("OperationService listening on {}", addr);
    Server::builder()
        .add_service(OperationServiceServer::new(server))
        .serve(addr)
        .await
        .unwrap();
    Ok(())
}
