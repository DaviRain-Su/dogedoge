use warp::http::StatusCode;
use warp::test::request;

use super::{
    filters,
    models::{self, Register},
};

#[tokio::test]
async fn test_post() {
    let db = models::blank_db();
    let api = filters::registers(db);

    let resp = request()
        .method("POST")
        .path("/register")
        .json(&Register {
            id: 1,
            phone_number: "17366503261".into(),
            password: "123456".into(),
            web3_address: "12bzRJfh7arnnfPPUZHeJUaE62QLEwhK48QnH9LXeK2m1iZU".into(),
        })
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_post_conflict() {
    let db = models::blank_db();
    db.lock().await.push(register1());
    let api = filters::registers(db);

    let resp = request()
        .method("POST")
        .path("/register")
        .json(&register1())
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_put_unknown() {
    let _ = pretty_env_logger::try_init();
    let db = models::blank_db();
    let api = filters::registers(db);

    let resp = request()
        .method("PUT")
        .path("/register/1")
        .header("authorization", "Bearer admin")
        .json(&register1())
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

fn register1() -> Register {
    Register {
        id: 1,
        phone_number: "17366503261".into(),
        password: "123456".into(),
        web3_address: "12bzRJfh7arnnfPPUZHeJUaE62QLEwhK48QnH9LXeK2m1iZU".into(),
    }
}
