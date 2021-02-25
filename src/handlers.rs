/// These are our API handlers, the ends of each filter chain.
/// Notice how thanks to using `Filter::and`, we can define a function
/// with the exact arguments we'd expect from each filter in the chain.
/// No tuples are needed, it's auto flattened for the functions.
use super::db::{
    DailyReward, ListOptions, Login, Login1, Login2, Register, RegistersDB, UserReward,
};
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use rbatis::Error;
use std::convert::Infallible;
use std::sync::Arc;
use warp::http::{self, StatusCode};
use warp::reply::Response;
use warp::Reply;

pub async fn list_user(
    _opts: ListOptions,
    db: Arc<Rbatis>,
) -> Result<impl warp::Reply, Infallible> {
    log::debug!("list_user");

    // let registers = db.list("").await;
    let registers = db.fetch_list("").await;
    if registers.is_err() {
        log::debug!("user is empty!");
        Ok(get_response(
            warp::reply::json(&Vec::<Register>::new()),
            StatusCode::OK,
        ))
    } else {
        let registers: Vec<Register> = registers
            .unwrap()
            .into_iter()
            .map(|val| Register::from(val))
            .collect();
        log::debug!("users is {:?}", registers);
        Ok(get_response(warp::reply::json(&registers), StatusCode::OK))
    }
}

// login by Login1
pub async fn login_by_uuid(par: Login1, db: Arc<Rbatis>) -> Result<impl warp::Reply, Infallible> {
    login(Login::LOGIN1(par), db.clone()).await
}

// login by Login2
pub async fn login_by_phone_number(
    par: Login2,
    db: Arc<Rbatis>,
) -> Result<impl warp::Reply, Infallible> {
    login(Login::LOGIN2(par), db.clone()).await
}

async fn login(login: Login, db: Arc<Rbatis>) -> Result<impl warp::Reply, Infallible> {
    match login {
        Login::LOGIN1(Login1 { uuid, password }) => {
            log::debug!("uuid = {}, password = {}", uuid, password);
            let w = db.new_wrapper().eq("uuid", &uuid);
            let r: Result<Option<RegistersDB>, Error> = db.fetch_by_wrapper("", &w).await;
            match r {
                Err(_err) => {
                    log::debug!("login error[{:?}]: search none thing by uuid", _err);
                    return Ok(get_response(
                        "login error: search none thing by uuid",
                        StatusCode::BAD_REQUEST,
                    ));
                }
                Ok(some) => match some {
                    None => {
                        log::debug!("login error: search result is None value by uuid");
                        return Ok(get_response(
                            "login error: search result is None value by uuid",
                            http::StatusCode::BAD_REQUEST,
                        ));
                    }
                    Some(res) => {
                        log::debug!("register db = {:?}", res);
                        if res.password.unwrap() == password {
                            log::debug!("login success");
                            return Ok(get_response("PASSWORD SUCCEESS", http::StatusCode::OK));
                        } else {
                            log::debug!("login failed");
                            return Ok(get_response("PASSWORD ERROR", http::StatusCode::NOT_FOUND));
                        }
                    }
                },
            }
        }
        Login::LOGIN2(Login2 {
            phone_number,
            password,
        }) => {
            let w = db.new_wrapper().eq("phone_number", &phone_number);
            let r: Result<Option<RegistersDB>, Error> = db.fetch_by_wrapper("", &w).await;
            match r {
                Err(_err) => {
                    log::debug!("login error[{:?}]: search none thing by uuid", _err);
                    return Ok(get_response(
                        "login error: search none thing by uuid",
                        http::StatusCode::BAD_REQUEST,
                    ));
                }
                Ok(some) => match some {
                    None => {
                        log::debug!("login error: search result is None value by uuid");
                        return Ok(get_response(
                            "login error: search result is None value by uuid",
                            http::StatusCode::BAD_REQUEST,
                        ));
                    }
                    Some(res) => {
                        log::debug!("register db = {:?}", res);
                        if res.password.unwrap() == password {
                            log::debug!("login success");
                            return Ok(get_response("PASSWORD SUCCEESS", http::StatusCode::OK));
                        } else {
                            log::debug!("login failed");
                            return Ok(get_response("PASSWORD ERROR", http::StatusCode::NOT_FOUND));
                        }
                    }
                },
            }
        }
    }
}

// 创建用户
pub async fn create_user(
    create: Register,
    db: Arc<Rbatis>,
) -> Result<impl warp::Reply, Infallible> {
    // 用户请求的合法性判断
    log::debug!("create_register: {:?}", create);
    let create_register_db = RegistersDB::from(create.clone());

    // 通过uuid唯一的标示
    let create_uuid = create.uuid;
    let create_phone_number = create.phone_number;
    let create_web3_address = create.web3_address;
    let w = db
        .new_wrapper()
        .eq("uuid", &create_uuid)
        .or()
        .eq("phone_number", &create_phone_number)
        .or()
        .eq("web3_address", &create_web3_address);

    // let ret_create_register_db: Result<Vec<RegistersDB>, Error> = db.list_by_wrapper("", &w).await;
    let ret_create_register_db: Result<Vec<RegistersDB>, Error> =
        db.fetch_list_by_wrapper("", &w).await;

    match ret_create_register_db {
        Err(_err) => {
            log::debug!("search register by id error ");
            return Ok(get_response(
                "search register by id error",
                http::StatusCode::BAD_REQUEST,
            ));
        }
        Ok(res) => {
            if res.is_empty() {
                let r = db.save("", &create_register_db).await;
                if r.is_err() {
                    log::debug!("create_resister: {}", r.err().unwrap().to_string());
                    return Ok(get_response(
                        "create user failed",
                        http::StatusCode::NOT_FOUND,
                    ));
                } else {
                    return Ok(get_response(
                        "create user success",
                        http::StatusCode::CREATED,
                    ));
                }
            } else {
                log::debug!(
                    "    -> id already exists (uuid :{}, phone number: {}, web3 address: {})",
                    create_uuid,
                    create_phone_number,
                    create_web3_address
                );
                return Ok(get_response(
                    "user already exists",
                    http::StatusCode::BAD_REQUEST,
                ));
            }
        }
    }
}

// 更新用户
pub async fn update_user(
    id: u64,
    update: Register,
    db: Arc<Rbatis>,
) -> Result<impl warp::Reply, Infallible> {
    log::debug!("update_register: id={}, register={:?}", id, update);

    let mut update_register_db = RegistersDB::from(update.clone());
    let update_id = db.update_by_id("", &mut update_register_db).await;
    match update_id {
        Ok(update_id) => {
            log::debug!("update register, res: {}, id : {}", update_id, id);
            return Ok(get_response("update success", http::StatusCode::OK));
        }
        Err(_err) => {
            // If the for loop didn't return OK, then the ID doesn't exist...
            log::debug!("    -> register id not found!");
            return Ok(get_response(
                "user id not found",
                http::StatusCode::NOT_FOUND,
            ));
        }
    }
}

pub async fn get_daily_reward(
    db: Arc<Rbatis>,
) -> Result<impl warp::Reply, Infallible> {
    log::debug!("get daily reward");

    let daily_reward = db.fetch_list("").await;
    if daily_reward.is_err() {
        log::debug!("daily is empty!");
        Ok(get_response(
            warp::reply::json(&Vec::<Register>::new()),
                    // "There is no daily address",
            StatusCode::OK,
        ))
    } else {
        let registers: Vec<UserReward> = daily_reward
            .unwrap()
            .into_iter()
            .map(|val| UserReward::from(val))
            .collect();
        log::debug!("daily reward {:?}", registers);
        Ok(get_response(warp::reply::json(&registers), StatusCode::OK))
    }
}

pub async fn post_daily_reward(
    user_ward: UserReward,
    db: Arc<Rbatis>,
) -> Result<impl warp::Reply, Infallible> {
    log::debug!("user ward : {:?}", user_ward);

    let daily_reward_db = DailyReward::from(user_ward.clone());

    let address = user_ward.address;
    let w = db.new_wrapper().eq("address", &address);

    let ret_daily_reward_db: Result<Vec<DailyReward>, Error> =
        db.fetch_list_by_wrapper("", &w).await;

    match ret_daily_reward_db {
        Err(_err) => {
            log::debug!("search daily reward by address error: {:?}", _err);
            return Ok(get_response(
                "search daily reward by address error",
                http::StatusCode::BAD_REQUEST,
            ));
        }
        Ok(res) => {
            if res.is_empty() {
                let r = db.save("", &daily_reward_db).await;
                if r.is_err() {
                    log::debug!("daily_reward : {}", r.err().unwrap().to_string());
                    return Ok(get_response(
                        "create daily reward failed",
                        http::StatusCode::NOT_FOUND,
                    ));
                } else {
                    return Ok(get_response(
                        "create daily reward success",
                        http::StatusCode::CREATED,
                    ));
                }
            } else {
                log::debug!("    -> id already exists (address :{})", address);
                return Ok(get_response(
                    "daily reward already exists",
                    http::StatusCode::BAD_REQUEST,
                ));
            }
        }
    }
}

// 删除用户
pub async fn delete_user(id: u64, db: Arc<Rbatis>) -> Result<impl warp::Reply, Infallible> {
    log::debug!("delete_register: id={}", id);

    let delete_id = db.remove_by_id::<RegistersDB>("", &id.to_string()).await;
    match delete_id {
        Ok(delete_id) => {
            log::debug!("delete_user: ret: {}, id: {}", delete_id, id);
            return Ok(get_response("delete user success", http::StatusCode::OK));
        }
        Err(_err) => {
            log::debug!("    -> user id not found!");
            return Ok(get_response(
                "user id not found!",
                http::StatusCode::NOT_FOUND,
            ));
        }
    }
}

fn get_response<T: Reply>(reply: T, statues: StatusCode) -> Response {
    // log::debug!("user is empty!");
    let mut resp =
        warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*").into_response();
    resp.headers_mut()
        .append("Connection", "Keep-Alive".parse().unwrap());
    resp.headers_mut()
        .append("Keep-Alive", "timeout=2, max=100".parse().unwrap());
    *resp.status_mut() = statues;
    resp
}
