use std::{
    sync::{
        Arc,
    },
};

use crossbeam_queue::{
    SegQueue,
};

use crate::{
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
    email: String,
) -> reqwest::Result<()> {
    Ok(())
}

pub fn send_email(
    queue: Arc<SegQueue<User>>,
) -> reqwest::Result<()> {
    let vec = utils::vec_from(queue);

    vec.into_iter().map(|user| {
        send_email_to(user.email)
    }).collect()
}

