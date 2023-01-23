use serenity::{
    client::Client,
    framework::standard::StandardFramework,
};

#[tokio::main]
async fn main() {
    // Initialize the client
    let mut client = Client::new(&token)
        .event_handler(Handler)
        .framework(StandardFramework::new().prefix("!").command

