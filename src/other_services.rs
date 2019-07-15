use std::{
    sync::{
        Arc,
    },
};

use crossbeam_queue::{
    SegQueue,
};
use grpcio::{
    ChannelBuilder,
    EnvBuilder,
};

use crate::{
    email::{
        EmailRequest,
        EmailResponse,
    },
    email_grpc::{
        EmailServiceClient,
    },
    TheodorResponse,
    User,
    utils,
};

pub fn get_time_rem(
) -> reqwest::Result<TheodorResponse> {
    reqwest::get("http://localhost:8000/mock_time_rem")?
        .json()
}

fn send_email_to(
    addr: String,
    body: String,
    client: &EmailServiceClient,
) -> grpcio::Result<()> {
    let mut req = EmailRequest::new();
    req.set_to(addr);
    req.set_subject("Laundry Update".to_string());
    req.set_body(body);

    client.send_email(&req).map(|_| {
        // TODO: handle?
        ()
    })
}

// TODO: better body
fn body_from(
    queue: Arc<SegQueue<User>>,
) -> String {
    format!("{:?}", queue)
}

pub fn send_email(
    queue: Arc<SegQueue<User>>,
) -> grpcio::Result<()> {
    let vec = utils::vec_from(queue.clone());
    let client = EmailServiceClient::new(
        ChannelBuilder::new(
            Arc::new(EnvBuilder::new().build()),
        ).connect("localhost:50051")
    );
    
    vec.into_iter().map(|user| {
        send_email_to(
            user.email,
            body_from(queue.clone()),
            &client,
        )
    }).collect()
}

