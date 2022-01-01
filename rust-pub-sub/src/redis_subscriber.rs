use std::error::Error;
use crate::messages::Message;

use redis::PubSubCommands;

pub fn subscribe(channel: String) -> Result<(), Box<dyn Error>> {
    let _ = tokio::spawn(async move {
        let client = redis::Client::open("redis://localhost").unwrap();
        let mut con = client.get_connection().unwrap();
        let _: () = con.subscribe(&[channel], |msg| {
            let received: String = msg.get_payload().unwrap();
            let msg_obj = serde_json::from_str::<Message>(&received).unwrap();
            crate::message_handler::handle(msg_obj);
            return redis::ControlFlow::Continue;
        }).unwrap();
    });
    Ok(())
}