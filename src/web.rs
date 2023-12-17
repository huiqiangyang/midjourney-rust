use polodb_core::Database;
use rouille::{Request, Response, router};

use crate::message::get_message;

pub(crate) fn handle_request(db: &Database, request: &Request) -> Response {
    router!(request,
        (GET) (/) => {
            Response::text("Hello world!")
        },
        (GET) (/{message_id: String}) => {
            let message = get_message(message_id.as_str(), db);
            match message {
                Some(message) => Response::json(&message),
                None => Response::empty_404()
            }
        },
        (GET) (/ hello /{name: String}) => {
        Response::text(format ! ("Hello {}", name))
        },
        _ => Response::empty_404()
        )
}
