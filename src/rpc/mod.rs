use grpc;

mod greeter;

pub fn server() -> grpc::Result<grpc::Server> {
    let mut builder = grpc::ServerBuilder::new_plain();
    builder.http.set_port(3000);
    builder.add_service(greeter::GreeterService::new());
    builder.build()
}
