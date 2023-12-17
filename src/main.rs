use std::sync::Arc;

use polodb_core::Database;

use crate::discord::discord;

mod discord;
mod web;
mod message;

#[tokio::main]
async fn main() {
    // 读取数据库
    let db = Arc::new(Database::open_file("db/midjourney").unwrap());

    // 初始化 log4rs
    log4rs::init_file("./log4rs.yaml", Default::default()).expect("failed to initialize log4rs");

    let db_clone = Arc::clone(&db);
    tokio::spawn(async move {
        discord(&db_clone);
    });

    rouille::start_server("127.0.0.1:8000", move |request| {
        web::handle_request(&db, request)
    });
}