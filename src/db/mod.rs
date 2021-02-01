use serde_derive::{Deserialize, Serialize};
// use chrono::NaiveDateTime;
// use rbatis::core::value::DateTimeNow;
// use rbatis::core::Error;
// use rbatis::crud::CRUD;
// use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
// use rbatis::plugin::page::{Page, PageRequest};
use rbatis::rbatis::Rbatis;
// use rbatis::plugin::version_lock::RbatisVersionLockPlugin;
use rbatis::core::db::DBPoolOptions;
use std::sync::Arc;

#[crud_enable]
#[derive(Clone, Debug)]
pub struct RegisterDB {
    pub id: Option<String>,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub web3_address: Option<String>,
}

impl RegisterDB {
    pub fn from(register: Register) -> Self {
        Self {
            id: Some(register.id.to_string()),
            phone_number: Some(register.phone_number),
            password: Some(register.password),
            web3_address: Some(register.web3_address),
        }
    }
}

pub async fn init_rbatis() -> Arc<Rbatis>{
    let rb = Rbatis::new();

    // 自定义连接池
    let mut opt = DBPoolOptions::new();
    opt.max_connections = 20;
    rb.link_opt(MYSQL_URL, &opt).await.unwrap();
    Arc::new(rb)
}


/// So we don't have to tackle how different database work, we'll just use
/// a simple in-memory DB, a vector synchronized by a mutex.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Register {
    pub id: u64,
    pub phone_number: String,
    pub password: String,
    pub web3_address: String,
}

impl Register {
    pub fn from(register_db: RegisterDB) -> Self {
        Self {
            id: register_db.id.unwrap().parse::<u64>().unwrap(),
            phone_number: register_db.phone_number.unwrap(),
            password: register_db.password.unwrap(),
            web3_address: register_db.web3_address.unwrap(),
        }
    }
}

// The query parameters for list_todos.
#[derive(Debug, Deserialize)]
pub struct ListOptions {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}




pub const MYSQL_URL: &str = "mysql://root:123456@47.98.193.249:3306/user";

lazy_static!{
    // Rbatis是线程、协程安全的，运行时的方法是Send+Sync，无需担心线程竞争
    pub static ref RB: Rbatis = Rbatis::new();
}


