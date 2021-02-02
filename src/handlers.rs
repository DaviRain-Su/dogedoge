/// These are our API handlers, the ends of each filter chain.
/// Notice how thanks to using `Filter::and`, we can define a function
/// with the exact arguments we'd expect from each filter in the chain.
/// No tuples are needed, it's auto flattened for the functions.
use super::db::{ListOptions, Login, Login1, Login2, Register, RegistersDB};
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use rbatis::Error;
use std::convert::Infallible;
use std::sync::Arc;
use warp::http;

pub async fn list_user(
    _opts: ListOptions,
    db: Arc<Rbatis>,
) -> Result<impl warp::Reply, Infallible> {
    log::debug!("list_user");

    let registers = db.list("").await;
    if registers.is_err() {
        log::debug!("user is empty!");
        Ok(warp::reply::json(&Vec::<Register>::new()))
    } else {
        let registers: Vec<Register> = registers
            .unwrap()
            .into_iter()
            .map(|val| Register::from(val))
            .collect();
        log::debug!("users is {:?}", registers);
        Ok(warp::reply::json(&registers))
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
                    return Ok(warp::reply::with_status(
                        "login error: search none thing by uuid",
                        http::StatusCode::BAD_REQUEST,
                    ));
                }
                Ok(some) => match some {
                    None => {
                        log::debug!("login error: search result is None value by uuid");
                        return Ok(warp::reply::with_status(
                            "login error: search result is None value by uuid",
                            http::StatusCode::BAD_REQUEST,
                        ));
                    }
                    Some(res) => {
                        log::debug!("register db = {:?}", res);
                        if res.password.unwrap() == password {
                            log::debug!("login success");
                            return Ok(warp::reply::with_status(
                                "PASSWORD SUCCEESS",
                                http::StatusCode::OK,
                            ));
                        } else {
                            log::debug!("login failed");
                            return Ok(warp::reply::with_status(
                                "PASSWORD ERROR",
                                http::StatusCode::NOT_FOUND,
                            ));
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
                    return Ok(warp::reply::with_status(
                        "login error: search none thing by uuid",
                        http::StatusCode::BAD_REQUEST,
                    ));
                }
                Ok(some) => match some {
                    None => {
                        log::debug!("login error: search result is None value by uuid");
                        return Ok(warp::reply::with_status(
                            "login error: search result is None value by uuid",
                            http::StatusCode::BAD_REQUEST,
                        ));
                    }
                    Some(res) => {
                        log::debug!("register db = {:?}", res);
                        if res.password.unwrap() == password {
                            log::debug!("login success");
                            return Ok(warp::reply::with_status(
                                "PASSWORD SUCCEESS",
                                http::StatusCode::OK,
                            ));
                        } else {
                            log::debug!("login failed");
                            return Ok(warp::reply::with_status(
                                "PASSWORD ERROR",
                                http::StatusCode::NOT_FOUND,
                            ));
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

    let create_id = create.id;
    let w = db.new_wrapper().eq("id", &create_id.to_string());
    let ret_create_register_db: Result<Option<RegistersDB>, Error> =
        db.fetch_by_wrapper("", &w).await;

    match ret_create_register_db {
        Err(_err) => {
            log::debug!("search register by id error ");
            return Ok(warp::reply::with_status(
                "search register by id error",
                http::StatusCode::BAD_REQUEST,
            ));
        }
        Ok(res) => match res {
            Some(some)
                if some.id == create_register_db.id
                    || some.phone_number == create_register_db.phone_number
                    || some.web3_address == create_register_db.web3_address =>
            {
                log::debug!("    -> id already exists: {}", create_id);

                return Ok(warp::reply::with_status(
                    "user already exists",
                    http::StatusCode::BAD_REQUEST,
                ));
            }
            None => {
                let r = db.save("", &create_register_db).await;
                if r.is_err() {
                    log::debug!("create_resister: {}", r.err().unwrap().to_string());
                    return Ok(warp::reply::with_status(
                        "create user failed",
                        http::StatusCode::NOT_FOUND,
                    ));
                } else {
                    return Ok(warp::reply::with_status(
                        "create user success",
                        http::StatusCode::OK,
                    ));
                }
            }
            _ => {
                return Ok(warp::reply::with_status(
                    "cann assess",
                    http::StatusCode::NOT_FOUND,
                ));
            }
        },
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
            Ok(warp::reply::with_status(
                "update success",
                http::StatusCode::OK,
            ))
        }
        Err(_err) => {
            // If the for loop didn't return OK, then the ID doesn't exist...
            log::debug!("    -> register id not found!");
            Ok(warp::reply::with_status(
                "user id not found",
                http::StatusCode::NOT_FOUND,
            ))
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
            Ok(warp::reply::with_status(
                "delete user success",
                http::StatusCode::OK,
            ))
        }
        Err(_err) => {
            log::debug!("    -> user id not found!");
            Ok(warp::reply::with_status(
                "user id not found!",
                http::StatusCode::NOT_FOUND,
            ))
        }
    }
}
