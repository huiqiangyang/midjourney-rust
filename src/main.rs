use std::thread;
use std::time::Duration;

use log::{error, info};
use zmq::Context;

fn main() {
    // 初始化 log4rs
    log4rs::init_file("./log4rs.yaml", Default::default()).expect("failed to initialize log4rs");

    let context = Context::new();

    // First, connect our subscriber
    let subscriber = context.socket(zmq::SUB).expect("failed creating socket");
    subscriber
        .connect("tcp://localhost:5555")
        .expect("failed connecting subscriber");
    info!("Connected subscriber");
    // Set a subscription filter (empty string means subscribe to all messages)
    subscriber.set_subscribe(b"").expect("failed setting subscription");

    thread::sleep(Duration::from_millis(1000));

    // Third, get our updates and report how many we got
    loop {
        match subscriber.recv_string(0) {
            Ok(Ok(message)) => {
                // 使用 info! 宏记录日志
                info!("Received {} updates", message);
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
