/// These are our API handlers, the ends of each filter chain.
/// Notice how thanks to using `Filter::and`, we can define a function
/// with the exact arguments we'd expect from each filter in the chain.
/// No tuples are needed, it's auto flattened for the functions.
use super::db::{ListOptions, Register, RegistersDB};
use std::convert::Infallible;
use warp::http::StatusCode;
use rbatis::rbatis::Rbatis;
use rbatis::crud::CRUD;
use std::sync::Arc;

// //登录查询操作
// pub async fn login(login: Register, db: Arc<Rbatis>) -> Result<impl warp::Reply, Infallible> {
//     log::debug!("login: {:?}", login);
//     // let login_user_db = RegistersDB::from(login.clone());
//
//     let login_id = login.id;
//     let ret_login_user_db = db.fetch_by_id::<Option<RegistersDB>>("", &login_id.to_string()).await;
//     match ret_login_user_db {
//         Err(_err) => {
//             log::debug!("login error, fetch by id error");
//             return Ok(StatusCode::BAD_REQUEST);
//         },
//         Ok(res) => {
//             match res {
//                 Some(some) => {
//                     let ret_register = Register::from(some);
//                     if ret_register == login {
//                         return Ok(StatusCode::OK);
//                     }
//                 },
//                 None => {
//                     log::debug!("login error, not found register");
//                     return Ok(StatusCode::BAD_REQUEST);
//                 }
//             }
//         }
//     }
//     // 登录成功返回 200 OK
//     Ok(StatusCode::OK)
// }

pub async fn list_register(_opts: ListOptions, db: Arc<Rbatis>) -> Result<impl warp::Reply, Infallible> {
    log::debug!("list_register");

    let registers = db.list("").await;
    if registers.is_err() {
        Ok(warp::reply::json(&Vec::<Register>::new()))
    }else {
        let registers : Vec<Register> = registers.unwrap().into_iter().map(|val| {
            Register::from(val)
        }).collect();
        Ok(warp::reply::json(&registers))
    }
}

// 创建用户
pub async fn create_register(create: Register, db: Arc<Rbatis>) -> Result<impl warp::Reply, Infallible> {
    // 用户请求的合法性判断
    log::debug!("create_register: {:?}", create);
    let create_register_db = RegistersDB::from(create.clone());

    let create_id = create.id;
    let ret_create_register_db  = db.fetch_by_id::<Option<RegistersDB>>("", &create_id.to_string()).await;
    match ret_create_register_db {
        Err(_err) => {
            log::debug!("search register by id error ");
        },
        Ok(res) => {
            match res {
                Some(some) => {
                    if some.id == create_register_db.id {
                        log::debug!("    -> id already exists: {}", create_id);
                        // create failed return StatusCode::BAD_REQUEST(400)
                        return Ok(StatusCode::BAD_REQUEST);
                    }
                },
                None => {
                    let r = db.save("", &create_register_db).await;
                    if r.is_err() {
                        log::debug!("create_resister: {}", r.err().unwrap().to_string());
                    }
                }
            }
        }
    }
    // create success return StatusCode::CREATED(200)
    Ok(StatusCode::CREATED)
}

// 更新用户
pub async fn update_register(id: u64, update: Register, db: Arc<Rbatis>) -> Result<impl warp::Reply, Infallible> {
    log::debug!("update_register: id={}, register={:?}", id, update);

    let mut update_register_db = RegistersDB::from(update.clone());
    let update_id = db.update_by_id("", &mut update_register_db).await;
    match update_id {
        Ok(update_id) => {
            log::debug!("update register, res: {}, id : {}", update_id, id);
            Ok(StatusCode::OK)
        },
        Err(_err) => {
            // If the for loop didn't return OK, then the ID doesn't exist...
            log::debug!("    -> register id not found!");
            Ok(StatusCode::NOT_FOUND)
        }
    }
}

// 删除用户
pub async fn delete_register(id: u64, db: Arc<Rbatis>) -> Result<impl warp::Reply, Infallible> {
    log::debug!("delete_register: id={}", id);

    let delete_id = db.remove_by_id::<RegistersDB>("", &id.to_string()).await;
    match delete_id {
        Ok(delete_id) => {
            log::debug!("delete_register: ret: {}, id: {}", delete_id, id);
            Ok(StatusCode::NO_CONTENT)
        },
        Err(_err) => {
            log::debug!("    -> register id not found!");
            Ok(StatusCode::NOT_FOUND)
        }
    }
}
