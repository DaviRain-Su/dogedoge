use super::handlers;
use super::models::{Db, ListOptions, Register};
use warp::Filter;

/// The 4 registers filters combined.
pub fn registers(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    register_list(db.clone())
        .or(register_create(db.clone()))
        .or(register_update(db.clone()))
        .or(register_delete(db))
}

/// GET /registers?offset=3&limit=5
pub fn register_list(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("register")
        .and(warp::get())
        .and(warp::query::<ListOptions>())
        .and(with_db(db))
        .and_then(handlers::list_register)
}

/// POST /register with JSON body
pub fn register_create(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("register")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::create_register)
}

/// PUT /registers/:id with JSON body
pub fn register_update(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("register" / u64)
        .and(warp::put())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::update_register)
}

/// DELETE /registers/:id
pub fn register_delete(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // We'll make one of our endpoints admin-only to show how authentication filters are used
    let admin_only = warp::header::exact("authorization", "Bearer admin");

    warp::path!("register" / u64)
        // It is important to put the auth check _after_ the path filters.
        // If we put the auth check before, the request `PUT /register/invalid-string`
        // would try this filter and reject because the authorization header doesn't match,
        // rather because the param is wrong for that other path.
        .and(admin_only)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(handlers::delete_register)
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (Register,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
