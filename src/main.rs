// #![deny(warnings)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;

use std::env;
use warp::Filter;

mod filters;
mod handlers;
mod db;

use db::init_rbatis;

#[cfg(test)]
mod tests;

/// Provides a RESTful web server managing some Registers.
///
/// API will be:
///
/// - `GET /Registers`: return a JSON list of Registerso.
/// - `POST /Registers`: create a new Register.
/// - `PUT /Registers/:id`: update a specific Register.
/// - `DELETE /Registers/:id`: delete a specific Register.
#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=registers=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "registers=info");
    }
    pretty_env_logger::init();


    let db = init_rbatis().await;


    let api = filters::registers(db.clone());

    // View access logs by setting `RUST_LOG=Registers`.
    let routes = api.with(warp::log("registers"));
    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
