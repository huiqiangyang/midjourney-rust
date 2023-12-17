use std::error::Error;

use log::{error, info};
use polodb_core::Database;
use zmq::Context;

use crate::message::{get_message, Message, save_message};

pub(crate)

fn discord(db: &Database) {
    let context = Context::new();

    // First, connect our subscriber
    let subscriber = context.socket(zmq::SUB).expect("failed creating socket");
    subscriber
        .connect("tcp://107.172.190.71:5555")
        .expect("failed connecting subscriber");
    info!("Connected subscriber");
    // Set a subscription filter (empty string means subscribe to all messages)
    subscriber.set_subscribe(b"").expect("failed setting subscription");

    loop {
        match subscriber.recv_string(0) {
            Ok(Ok(message)) => {
                if let Err(err) = handle_message(message, &db) {
                    // 记录错误，但不中断程序
                    error!("Error handling message: {}", err);
                }
            }
            Ok(Err(e)) => {
                // 使用 error! 宏记录错误日志
                error!("Failed to decode message: {:?}", e);
            }
            Err(e) => {
                // 使用 error! 宏记录错误日志
                error!("Failed to receive message: {}", e);
            }
        }
    }
}

fn handle_message(message: String, db: &Database) -> Result<(), Box<dyn Error>> {
    // Attempt to parse the JSON
    let json: serde_json::Value = serde_json::from_str(&message).map_err(|err| {
        log::error!("Error parsing JSON: {}", err);
        Box::new(err) as Box<dyn Error>
    })?;

    // json get t
    let t = json.get("t").and_then(|t| t.as_str()).unwrap_or_default();

    // get message id
    if t == "MESSAGE_CREATE" {
        let message_id = json.get("d").and_then(|d| d.get("id")).and_then(|id| id.as_str()).unwrap_or_default();
        save_message(Message {
            message_id: message_id.to_string(),
            data: message,
        }, db);
        // find by message id
        let message = get_message(message_id, db);
        log::info!("Found document: {:?}", message);
    }

    Ok(())
}



