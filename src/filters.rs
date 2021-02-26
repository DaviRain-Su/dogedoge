use serde::Deserialize;
use super::db::{ListOptions, Register, UserReward};
use super::handlers;
use crate::db::{Login1, Login2};
use rbatis::rbatis::Rbatis;
use std::sync::Arc;
use warp::Filter;

/// The 4 registers filters combined.
pub fn main_logic(
    db: Arc<Rbatis>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    user_list(db.clone())
        .or(register(db.clone())) // 注册逻辑
        .or(login_by_uuid(db.clone())) // 登录by uuid
        .or(login_by_phone_number(db.clone())) // 登录by phone_number
        .or(update_password(db.clone())) // 更新密码
        .or(update_web3_address(db.clone())) // 更新web3地址
        .or(update_phone_number(db.clone())) // 更新手机号
        .or(user_list(db.clone()))
        .or(delete_user(db.clone()))
        .or(get_daily_reward(db.clone())) // 检查每日奖励
        .or(post_daily_reward(db.clone())) // 插入每日奖励
}

// 登录逻辑
/// POST /login1 with JSON body
pub fn login_by_uuid(
    db: Arc<Rbatis>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("login1")
        .and(warp::post())
        .and(json_body_for_login1())
        .and(with_db(db))
        .and_then(handlers::login_by_uuid)
}

/// POST /login2 with JSON body
pub fn login_by_phone_number(
    db: Arc<Rbatis>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("login2")
        .and(warp::post())
        .and(json_body_for_login2())
        .and(with_db(db))
        .and_then(handlers::login_by_phone_number)
}

/// GET /users?offset=3&limit=5
pub fn user_list(
    db: Arc<Rbatis>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("users")
        .and(warp::get())
        .and(warp::query::<ListOptions>())
        .and(with_db(db))
        .and_then(handlers::list_user)
}

// 注册
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

// 更新密码
/// PUT /user/password/:id with JSON body
pub fn update_password(
    db: Arc<Rbatis>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // let router = String::from("update-password");
    warp::path!("user" / "password" / u64)
        .and(warp::put())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::update_user)
}

// 更新地址
/// PUT /user/web3address/:id with JSON body
pub fn update_web3_address(
    db: Arc<Rbatis>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // let router = String::from("update-web3-address");
    warp::path!("user" / "web3address" / u64)
        .and(warp::put())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::update_user)
}

// 更新手机号
/// PUT /user/phonenumber/:id with JSON body
pub fn update_phone_number(
    db: Arc<Rbatis>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // let router = String::from("update-phone-number");
    warp::path!("user" / "phonenumber" / u64)
        .and(warp::put())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::update_user)
}

// 查询是否已经获得日常奖励
/// GET /daily-reward
pub fn get_daily_reward(
    db: Arc<Rbatis>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("daily-reward")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::get_daily_reward)
}

// 插入当日奖励
/// POST /daily-reward JSON body
pub fn post_daily_reward(
    db: Arc<Rbatis>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let admin_only = warp::header::exact("authorization", "Bearer admin");
    warp::path!("daily-reward")
        .and(admin_only)
        .and(generics_json_body::<UserReward>())
        .and(warp::post())
        .and(with_db(db))
        .and_then(handlers::post_daily_reward)
}

// 删除用户
/// DELETE /user/:id
pub fn delete_user(
    db: Arc<Rbatis>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // We'll make one of our endpoints admin-only to show how authentication filters are used
    let admin_only = warp::header::exact("authorization", "Bearer admin");

    warp::path!("user" / u64)
        // It is important to put the auth check _after_ the path filters.
        // If we put the auth check before, the request `PUT /register/invalid-string`
        // would try this filter and reject because the authorization header doesn't match,
        // rather because the param is wrong for that other path.
        .and(admin_only)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(handlers::delete_user)
}

fn with_db(
    db: Arc<Rbatis>,
) -> impl Filter<Extract = (Arc<Rbatis>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (Register,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn generics_json_body<T>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone
where
    T: Send +  for<'de> Deserialize<'de>,
{
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn json_body_for_login1() -> impl Filter<Extract = (Login1,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
fn json_body_for_login2() -> impl Filter<Extract = (Login2,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
