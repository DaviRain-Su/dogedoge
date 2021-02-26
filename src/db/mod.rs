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
pub struct RegistersDB {
    pub id: Option<u64>,
    pub uuid: Option<String>,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub web3_address: Option<String>,
    pub sign_time: Option<String>,
    pub login_time: Option<String>,
}

impl RegistersDB {
    pub fn from(register: Register) -> Self {
        Self {
            id: None,
            uuid: Some(register.uuid),
            phone_number: Some(register.phone_number),
            password: Some(register.password),
            web3_address: Some(register.web3_address),
            login_time: Some(register.login_time),
            sign_time: Some(register.sign_time),
        }
    }
}

// #[crud_enable]
// #[derive(Clone, Debug)]
// pub struct DailyReward {
//     pub id: Option<String>,
//     pub address: Option<String>,
//     pub created_at: Option<String>,
//     pub updated_at: Option<String>,
// }

pub async fn init_rbatis() -> Arc<Rbatis> {
    let rb = Rbatis::new();

    // 自定义连接池
    let mut opt = DBPoolOptions::new();
    opt.max_connections = 20;
    rb.link_opt(MYSQL_URL, &opt).await.unwrap();
    Arc::new(rb)
}

/// So we don't have to tackle how different database work, we'll just use
/// a simple in-memory DB, a vector synchronized by a mutex.

#[derive(Debug, Deserialize, Serialize, Clone, std::cmp::PartialEq)]
pub struct Register {
    pub id: u64,
    pub uuid: String,
    pub phone_number: String,
    pub password: String,
    pub web3_address: String,
    pub sign_time: String,
    pub login_time: String,
}

impl Register {
    pub fn from(register_db: RegistersDB) -> Self {
        Self {
            id: register_db.id.unwrap(),
            uuid: register_db.uuid.unwrap(),
            phone_number: register_db.phone_number.unwrap(),
            password: register_db.password.unwrap(),
            web3_address: register_db.web3_address.unwrap(),
            sign_time: register_db.sign_time.unwrap(),
            login_time: register_db.login_time.unwrap(),
        }
    }
}

// #[crud_enable]
#[derive(Debug, Deserialize, Serialize, Clone, std::cmp::PartialEq)]
pub struct UserReward {
    pub address: String,
}


impl UserReward {
    pub fn from(daily_reward: DailyReward) -> Self {
        Self {
            address: daily_reward.address.unwrap(),
        }
    }
}


#[crud_enable]
#[derive(Debug, Clone)]
pub struct DailyReward {
    pub id: Option<u64>,
    pub address: Option<String>,
}

impl DailyReward {
    pub fn from(user_reward: UserReward) -> Self {
        Self {
            id: None,
            address: Some(user_reward.address),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Login {
    LOGIN1(Login1),
    LOGIN2(Login2),
}

#[derive(Debug, Deserialize, Serialize, Clone, std::cmp::PartialEq)]
pub struct Login1 {
    pub uuid: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, std::cmp::PartialEq)]
pub struct Login2 {
    pub phone_number: String,
    pub password: String,
}

// The query parameters for list_todos.
#[derive(Debug, Deserialize)]
pub struct ListOptions {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

pub const MYSQL_URL: &str = "mysql://root:123456@47.98.193.249:3306/user";

lazy_static! {
    // Rbatis是线程、协程安全的，运行时的方法是Send+Sync，无需担心线程竞争
    pub static ref RB: Rbatis = Rbatis::new();
}
