extern crate futures;
extern crate grpcio;
extern crate protobuf;

use std::sync::Arc;
use grpcio::{ChannelBuilder, EnvBuilder};
use futures::Future;

pub mod protos;
pub use protos::chaos::*;
pub use protos::chaos_grpc::*;

fn main() {
    let env = Arc::new(EnvBuilder::new()
        .cq_count(8)
        .build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = ChaosClient::new(ch);

    // initialize the request
    let mut request = ChaosRequest::new();
    request.set_whatever("this is a string".to_string());

    let replies: Vec<grpcio::ClientUnaryReceiver<_>> = (0..3).map(|i| {
        client.spread_chaos_async(&request)
            .expect("rpc error")
    }).collect();

    let results: Vec<ChaosResponse> = replies.into_iter().map(|reply| {
        reply.wait()
            .expect("error with future")
    }).collect();

    for result in results {
        println!("Received chaos with id: {}", result.get_id());
    }
}
