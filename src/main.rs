#![feature(proc_macro_hygiene, decl_macro)]

extern crate crossbeam_queue;
extern crate periodic;
extern crate reqwest;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

mod mock;

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
struct TheodorResponse {
    time_remaining: i32,
}

#[derive(Debug, Deserialize, Serialize)]
enum MachineState {
    On,
    Off,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct User {
    name: String,
}

fn get_time_rem(
) -> reqwest::Result<TheodorResponse> {
    reqwest::get("http://localhost:8000/mock_time_rem")?
        .json()
}

fn popper(
    planner: &mut Planner,
    queue: Arc<SegQueue<User>>,
) {
    let machine_state: Arc<RwLock<MachineState>> = Arc::new(RwLock::new(
        MachineState::Off,
    ));
    let queue_clone = queue.clone();

    planner.add(
        move || {
            get_time_rem().map(|theodor_response| {
                let tr = theodor_response.time_remaining;
                println!("{} {:?}", tr, machine_state);
                // *(
                //     machine_state.write().expect("poisoned RwLock")
                // ) = match *(
                //     machine_state.read().expect("poisoned RwLock")
                // ) {
                //     MachineState::Off => Some(tr).filter(|tr| {
                //         *tr > 0
                //     }).and_then(|_| {
                //         queue_clone.pop().map(|_| {
                //             MachineState::On
                //         }).ok()
                //     }).or_else(|| {
                //         Some(MachineState::Off)
                //     }).unwrap(),
                //     MachineState::On => Some(tr).filter(|tr| {
                //         *tr < 0
                //     }).and_then(|_| {
                //         Some(MachineState::Off)
                //     }).or_else(|| {
                //         Some(MachineState::On)
                //     }).unwrap(),
                // };
                println!("{}_{:?}", tr, machine_state);
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

// #[get("/pop", format = "application/json")]
// fn pop(
//     queue: State<Arc<SegQueue<User>>>,
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
    queue: State<Arc<SegQueue<User>>>,
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
    queue: Arc<SegQueue<User>>,
) -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![
            push,
            // pop,
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
