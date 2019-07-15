use std::{
    sync::{
        Arc,
    },
};

use crossbeam_queue::{
    SegQueue,
};

use crate::{
    User,
};

pub fn vec_from(
    queue: Arc<SegQueue<User>>
) -> Vec<User> {
    let mut vec = Vec::new();

    (0..queue.len()).map(|_| {
        let item = queue.pop().expect("problem in code");
        vec.push(item.clone());
        queue.push(item);
    }).last();

    vec
}
