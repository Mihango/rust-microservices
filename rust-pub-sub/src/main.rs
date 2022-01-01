mod messages;
mod redis_publisher;
mod redis_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("......... Service started .........");

    if let Err(error) = redis_subscriber::subscribe(String::from("order")) {
        eprintln!("{:?}", error);
        panic!("{:?}", error);
    } else {
        println!("Connected to queue");
    }

    redis_publisher::publish_message(messages::Message::new(
        messages::Order::new("T-shirt".to_string(), 3, 24.0)
    ))?;
    redis_publisher::publish_message(messages::Message::new(
        messages::Order::new("Sneakers".to_string(), 1, 230.0)
    ))?;
    redis_publisher::publish_message(messages::Message::new(
        messages::Order::new("Milka Bar".to_string(), 10, 50.0)
    ))?;

    println!("published");

    Ok(())
}

pub mod message_handler {
    use crate::messages::Message;
    
    pub fn handle(message: Message) {
        println!("{:?}", message);
    }
}
