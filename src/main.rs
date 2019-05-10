extern crate blursed;

use discord::Discord;
use discord::model::Event;
use std::env;

fn main() {
	// Log in to Discord using a bot token from the environment
	let discord = Discord::from_bot_token(
		&env::var("TOKEN_DISCORD").expect("Expected token"),
	).expect("login failed");

	// Establish and use a websocket connection
	let (mut connection, _) = discord.connect().expect("connect failed");
	println!("Ready.");
	loop {
		match connection.recv_event() {
			Ok(Event::MessageCreate(message)) => {
				println!("{} says: {}", message.author.name, message.content);
				if message.content == "b,test" {
					let _ = discord.send_message(message.channel_id, "This is a reply to the test.", "", false);
				} else if message.content == "b,quit" {
					println!("Quitting.");
					break
				}
			}
			Ok(_) => {}
			Err(discord::Error::Closed(code, body)) => {
				println!("Gateway closed on us with code {:?}: {}", code, body);
				break
			}
			Err(err) => println!("Receive error: {:?}", err)
		}
	}
}
