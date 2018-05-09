// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_CHAOS_SPREAD_CHAOS: ::grpcio::Method<super::chaos::ChaosRequest, super::chaos::ChaosResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/Chaos/SpreadChaos",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct ChaosClient {
    client: ::grpcio::Client,
}

impl ChaosClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ChaosClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn spread_chaos_opt(&self, req: &super::chaos::ChaosRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::chaos::ChaosResponse> {
        self.client.unary_call(&METHOD_CHAOS_SPREAD_CHAOS, req, opt)
    }

    pub fn spread_chaos(&self, req: &super::chaos::ChaosRequest) -> ::grpcio::Result<super::chaos::ChaosResponse> {
        self.spread_chaos_opt(req, ::grpcio::CallOption::default())
    }

    pub fn spread_chaos_async_opt(&self, req: &super::chaos::ChaosRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::chaos::ChaosResponse>> {
        self.client.unary_call_async(&METHOD_CHAOS_SPREAD_CHAOS, req, opt)
    }

    pub fn spread_chaos_async(&self, req: &super::chaos::ChaosRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::chaos::ChaosResponse>> {
        self.spread_chaos_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Chaos {
    fn spread_chaos(&self, ctx: ::grpcio::RpcContext, req: super::chaos::ChaosRequest, sink: ::grpcio::UnarySink<super::chaos::ChaosResponse>);
}

pub fn create_chaos<S: Chaos + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CHAOS_SPREAD_CHAOS, move |ctx, req, resp| {
        instance.spread_chaos(ctx, req, resp)
    });
    builder.build()
}
