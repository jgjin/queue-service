extern crate rand;

use rand::{
    Rng,
};
use rocket_contrib::{
    json::{
        JsonValue,
    },
};

use crate::{
    TheodorResponse,
};

#[get("/", format = "application/json")]
pub fn mock_time_rem(
) -> JsonValue {
    let mut rng = rand::thread_rng();

    json!(TheodorResponse {
        time_remaining: rng.gen::<i32>(),
    })
}
