#![feature(proc_macro_hygiene, decl_macro)]

extern crate crossbeam_queue;
extern crate periodic;
extern crate reqwest;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

mod mock;
mod other_services;
mod utils;

use std::{
    sync::{
        Arc,
        RwLock,
    },
    time::{
        Duration,
    },
};

use crossbeam_queue::{
    SegQueue,
};
use periodic::{
    Planner,
    Every,
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
pub struct TheodorResponse {
    time_remaining: i32,
}

#[derive(Debug, Deserialize, Serialize)]
enum MachineState {
    On,
    Off,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    name: String,
    email: String,
}

fn popper(
    planner: &mut Planner,
    queue: Arc<SegQueue<User>>,
) {
    let machine_state: Arc<RwLock<MachineState>> = Arc::new(RwLock::new(
        MachineState::Off,
    ));

    planner.add(
        move || {
            other_services::get_time_rem().map(|theodor_response| {
                let tr = theodor_response.time_remaining;
                let temp = match *(
                    machine_state.read().expect("poisoned RwLock")
                ) {
                    MachineState::Off => Some(tr).filter(|tr| {
                        *tr > 0
                    }).map(|_| {
                        queue.clone().pop().map(|_| {
                            ()
                        }).unwrap_or(println!("empty queue"));
                        MachineState::On
                    }).unwrap_or(MachineState::Off),
                    MachineState::On => Some(tr).filter(|tr| {
                        *tr < 0
                    }).map(|_| {
                        other_services::send_email(queue.clone()).map(|_| {
                            ()
                        }).unwrap_or(println!("error emailing"));
                        MachineState::Off
                    }).unwrap_or(MachineState::On),
                };
                *(machine_state.write().expect("poisoned RwLock")) = temp;
            }).expect("error in theodor api");
        },
        Every::new(Duration::from_secs(2)),
    );
}

#[post("/push", format = "application/json", data = "<user>")]
fn push(
    user: Json<User>,
    queue: State<Arc<SegQueue<User>>>,
) -> JsonValue {
    let user = user.into_inner();
    queue.push(user.clone());
    json!({
        "status_code": 200,
        "user": user,
    })
}

#[get("/get", format = "application/json")]
fn get(
    queue: State<Arc<SegQueue<User>>>,
) -> JsonValue {
    let vec = utils::vec_from(queue.inner().clone());

    json!({
        "status_code": 200,
        "queue": vec,
    })
}

fn rocket(
    queue: Arc<SegQueue<User>>,
) -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![
            push,
            get,
        ])
        .mount("/mock_time_rem", routes![
            mock::mock_time_rem,
        ])
        .manage(queue)
}

fn main(
) {
    let mut planner = Planner::new();
    let queue = Arc::new(SegQueue::new());

    popper(&mut planner, queue.clone());

    planner.start();
    rocket(queue).launch();
}
