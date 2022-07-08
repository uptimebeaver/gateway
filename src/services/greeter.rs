use template::greeter_server::{Greeter, GreeterServer};
use template::{HelloReply, HelloRequest};
use tonic::{Request, Response, Status};

pub mod template {
    tonic::include_proto!("template");
}

#[derive(Debug, Default)]
pub struct GreeterService {}

#[tonic::async_trait]
impl Greeter for GreeterService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloReply>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = template::HelloReply {
            message: format!("Hello {}!", request.into_inner().name), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

impl GreeterService {
    pub fn create_server() -> GreeterServer<GreeterService> {
        let server = GreeterService::default();
        GreeterServer::new(server)
    }
}
