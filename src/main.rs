#![feature(proc_macro_hygiene, decl_macro)]

extern crate crossbeam_queue;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

use std::{
    thread,
};

use crossbeam_queue::{
    SegQueue,
};
use rocket::{
    State,
};
use rocket_contrib::{
    json::{
        Json,
        JsonValue,
    },
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Deserialize, Serialize)]
enum MachineState {
    On,
    Off,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct User {
    name: String,
}

fn popper(
    queue: SegQueue<User>,
) -> {
    thread
}

#[post("/push", format = "application/json", data = "<user>")]
fn push(
    user: Json<User>,
    queue: State<SegQueue<User>>,
) -> JsonValue {
    let user = user.into_inner();
    queue.push(user.clone());
    json!({
        "status_code": 200,
        "user": user,
    })
}

// #[get("/pop", format = "application/json")]
// fn pop(
//     queue: State<SegQueue<User>>,
// ) -> JsonValue {
//     queue.pop().map(|user| {
//         json!({
//             "status_code": 200,
//             "user": user,
//         })
//     }).unwrap_or(json!({
//         "status_code": 404,
//         "message": "empty queue",
//     }))
// }

#[get("/get", format = "application/json")]
fn get(
    queue: State<SegQueue<User>>,
) -> JsonValue {
    let mut vec = Vec::new();

    (0..queue.len()).map(|_| {
        let item = queue.pop().expect("problem in code");
        vec.push(item.clone());
        queue.push(item);
    }).last();

    json!({
        "status_code": 200,
        "queue": vec,
    })
}

fn rocket(
    queue: SegQueue<User>,
) -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![
            push,
            // pop,
            get,
        ])
        .manage(queue)
}

fn main(
) {
    let queue = SegQueue::new();
    rocket(queue).launch();
}
