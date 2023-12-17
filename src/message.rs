use std::error::Error;

use polodb_core::bson::doc;
use polodb_core::Database;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub(crate) message_id: String,
    pub(crate) data: String,
}


// save message
pub(crate) fn save_message(message: Message, db: &Database) {
    let collection = db.collection("message");
    collection.insert_one(message).map_err(|err| {
        log::error!("Error inserting document: {}", err);
        Box::new(err) as Box<dyn Error>
    }).unwrap();
}


pub(crate) fn get_message(message_id: &str, db: &Database) -> Option<Message> {
    let collection = db.collection("message");

    // 使用 Option 来处理可能的错误
    match collection.find_one(doc! {"message_id": message_id}) {
        Ok(Some(message)) => Some(message),
        Ok(None) => {
            log::warn!("No message found with id: {}", message_id);
            None
        }
        Err(err) => {
            log::error!("Error finding document: {}", err);
            None
        }
    }
}