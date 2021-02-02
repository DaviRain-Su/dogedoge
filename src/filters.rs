use super::handlers;
use super::db::{ ListOptions, Register};
use warp::Filter;
use rbatis::rbatis::Rbatis;
use std::sync::Arc;


/// The 4 registers filters combined.
pub fn main_logic(db: Arc<Rbatis>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    user_list(db.clone())
        .or(register(db.clone()))
        .or(update_user(db.clone()))
        .or(delete_user(db))
}

/// GET /login
pub fn login(db: Arc<Rbatis>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("login")
        .and(warp::get())
        .and(warp::query::<ListOptions>())
        .and(with_db(db))
        .and_then(handlers::list_register)
}

/// GET /registers?offset=3&limit=5
pub fn user_list(
    db: Arc<Rbatis>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("register")
        .and(warp::get())
        .and(warp::query::<ListOptions>())
        .and(with_db(db))
        .and_then(handlers::list_user)
}

/// POST /register with JSON body
pub fn register(
    db: Arc<Rbatis>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("register")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::create_user)
}

/// PUT /registers/:id with JSON body
pub fn update_user(
    db: Arc<Rbatis>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("register" / u64)
        .and(warp::put())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::update_user)
}

/// DELETE /registers/:id
pub fn delete_user(
    db: Arc<Rbatis>,
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
        .and_then(handlers::delete_user)
}

fn with_db(db: Arc<Rbatis>) -> impl Filter<Extract = (Arc<Rbatis>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (Register,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
