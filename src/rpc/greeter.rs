use crate::proto::helloworld::{HelloReply, HelloRequest};
use crate::proto::helloworld_grpc::{Greeter, GreeterServer};
use grpc;

pub struct GreeterService;

impl Greeter for GreeterService {
    fn say_hello(
        &self,
        _: grpc::RequestOptions,
        req: HelloRequest,
    ) -> grpc::SingleResponse<HelloReply> {
        let mut reply = HelloReply::default();
        let name = if req.get_name().is_empty() {
            "world"
        } else {
            req.get_name()
        };
        println!("greeting request from {}", name);
        reply.set_message(format!("Hello {}", name));
        grpc::SingleResponse::completed(reply)
    }
}

impl GreeterService {
    pub fn new() -> grpc::rt::ServerServiceDefinition {
        GreeterServer::new_service_def(GreeterService)
    }
}
