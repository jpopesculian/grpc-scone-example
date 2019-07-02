extern crate futures;
extern crate signal_hook;

extern crate grpc;

mod proto;
mod rpc;
mod utils;

use crate::utils::sync::block_until_user_exit;

fn main() {
    let server = rpc::server().unwrap();
    println!("server started on {}", server.local_addr());
    block_until_user_exit().unwrap();
    println!("server shut down");
}
