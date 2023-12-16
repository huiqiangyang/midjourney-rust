use std::thread;
use std::time::Duration;

use zmq::Context;

fn main() {
    let context = Context::new();

    // First, connect our subscriber
    let subscriber = context.socket(zmq::SUB).expect("failed creating socket");
    subscriber
        .connect("tcp://localhost:5555")
        .expect("failed connecting subscriber");

    // Set a subscription filter (empty string means subscribe to all messages)
    subscriber.set_subscribe(b"").expect("failed setting subscription");

    thread::sleep(Duration::from_millis(1000));

    // Third, get our updates and report how many we got
    loop {
        let message = subscriber
            .recv_string(0)
            .expect("failed receiving update")
            .unwrap();
        println!("Received {} updates", message);
    }
}
