/// These are our API handlers, the ends of each filter chain.
/// Notice how thanks to using `Filter::and`, we can define a function
/// with the exact arguments we'd expect from each filter in the chain.
/// No tuples are needed, it's auto flattened for the functions.
use super::models::{Db, ListOptions, Register};
use std::convert::Infallible;
use warp::http::StatusCode;

pub async fn list_register(opts: ListOptions, db: Db) -> Result<impl warp::Reply, Infallible> {
    // Just return a JSON array of register, applying the limit and offset.
    let registers = db.lock().await;
    let registers: Vec<Register> = registers
        .clone()
        .into_iter()
        .skip(opts.offset.unwrap_or(0))
        .take(opts.limit.unwrap_or(std::usize::MAX))
        .collect();
    Ok(warp::reply::json(&registers))
}

pub async fn create_register(create: Register, db: Db) -> Result<impl warp::Reply, Infallible> {
    log::debug!("create_register: {:?}", create);

    let mut vec = db.lock().await;

    for register in vec.iter() {
        if register.id == create.id {
            log::debug!("    -> id already exists: {}", create.id);
            // register with id already exists, return `400 BadRequest`.
            return Ok(StatusCode::BAD_REQUEST);
        }
    }

    // No existing register with id, so insert and return `201 Created`.
    vec.push(create);

    Ok(StatusCode::CREATED)
}

pub async fn update_register(id: u64, update: Register, db: Db) -> Result<impl warp::Reply, Infallible> {
    log::debug!("update_register: id={}, register={:?}", id, update);
    let mut vec = db.lock().await;

    // Look for the specified register...
    for register in vec.iter_mut() {
        if register.id == id {
            *register = update;
            return Ok(StatusCode::OK);
        }
    }

    log::debug!("    -> register id not found!");

    // If the for loop didn't return OK, then the ID doesn't exist...
    Ok(StatusCode::NOT_FOUND)
}

pub async fn delete_register(id: u64, db: Db) -> Result<impl warp::Reply, Infallible> {
    log::debug!("delete_register: id={}", id);

    let mut vec = db.lock().await;

    let len = vec.len();
    vec.retain(|register| {
        // Retain all registers that aren't this id...
        // In other words, remove all that *are* this id...
        register.id != id
    });

    // If the vec is smaller, we found and deleted a register!
    let deleted = vec.len() != len;

    if deleted {
        // respond with a `204 No Content`, which means successful,
        // yet no body expected...
        Ok(StatusCode::NO_CONTENT)
    } else {
        log::debug!("    -> register id not found!");
        Ok(StatusCode::NOT_FOUND)
    }
}
