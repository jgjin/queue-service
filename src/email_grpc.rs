// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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

const METHOD_EMAIL_SERVICE_SEND_EMAIL: ::grpcio::Method<super::email::EmailRequest, super::email::EmailResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/email.EmailService/SendEmail",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct EmailServiceClient {
    client: ::grpcio::Client,
}

impl EmailServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        EmailServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn send_email_opt(&self, req: &super::email::EmailRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::email::EmailResponse> {
        self.client.unary_call(&METHOD_EMAIL_SERVICE_SEND_EMAIL, req, opt)
    }

    pub fn send_email(&self, req: &super::email::EmailRequest) -> ::grpcio::Result<super::email::EmailResponse> {
        self.send_email_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_email_async_opt(&self, req: &super::email::EmailRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::email::EmailResponse>> {
        self.client.unary_call_async(&METHOD_EMAIL_SERVICE_SEND_EMAIL, req, opt)
    }

    pub fn send_email_async(&self, req: &super::email::EmailRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::email::EmailResponse>> {
        self.send_email_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait EmailService {
    fn send_email(&mut self, ctx: ::grpcio::RpcContext, req: super::email::EmailRequest, sink: ::grpcio::UnarySink<super::email::EmailResponse>);
}

pub fn create_email_service<S: EmailService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_EMAIL_SERVICE_SEND_EMAIL, move |ctx, req, resp| {
        instance.send_email(ctx, req, resp)
    });
    builder.build()
}
