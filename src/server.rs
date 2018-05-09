extern crate futures;
extern crate grpcio;
extern crate protobuf;
extern crate rand;

use std::io::Read;
use std::sync::Arc;
use std::{io, thread, time};

use rand::Rng;
use futures::Future;
use futures::sync::oneshot;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

pub mod protos;
pub use protos::chaos::*;
pub use protos::chaos_grpc::*;

#[derive(Clone)]
struct ChaosService;

impl Chaos for ChaosService {
    fn spread_chaos(&self, ctx: RpcContext, req: ChaosRequest, sink: UnarySink<ChaosResponse>) {
        println!("received a thing");
        // TODO: There's no guarantee this won't collide
        thread::sleep_ms(1000);
        let id = rand::thread_rng().gen::<u64>();
        let mut resp = ChaosResponse::new();
        resp.set_id(id);
        let f = sink.success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f);
    }
}

fn main() {
    let env = Arc::new(Environment::new(8));
    let service = create_chaos(ChaosService);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50051)
        .build()
        .expect("Could not create server");
    server.start();

    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }

    let (tx, rx) = oneshot::channel();

    thread::spawn(move || {
        println!("press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });

    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
