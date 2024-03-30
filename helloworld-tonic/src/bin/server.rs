use helloworld_tonic::generated_protos::helloworld::greeter_server::{Greeter, GreeterServer};
use helloworld_tonic::generated_protos::helloworld::{HelloReply, HelloRequest};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let addr = "[::1]:50051".parse()?;
//     let greeter = MyGreeter::default();
//
//     println!("Starting server at address: {:?}", addr);
//
//     Server::builder()
//         .add_service(GreeterServer::new(greeter))
//         .serve(addr)
//         .await?;
//
//     Ok(())
// }

#[derive(Debug)]
struct MyDumbError {
    msg: String,
}

fn main() -> Result<(), MyDumbError> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .or_else(|e| {
            Err(MyDumbError {
                msg: format!("{:?}", e),
            })
        })?;
    rt.block_on(async {
        let addr = "[::1]:50051".parse().map_err(|e| MyDumbError {
            msg: format!("{:?}", e),
        })?;
        let greeter = MyGreeter::default();

        println!("Starting server at address: {:?}", addr);

        Server::builder()
            .add_service(GreeterServer::new(greeter))
            .serve(addr)
            .await
            .map_err(|e| MyDumbError {
                msg: format!("{:?}", e),
            })
    })
}
