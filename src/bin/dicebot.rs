use std::env;

use serenity::{
  model::{channel::Message, gateway::Ready},
  prelude::*,
};

use dice_bot::bot_handle_this;

struct Handler;
impl EventHandler for Handler {
  // Set a handler for the `message` event - so that whenever a new message
  // is received - the closure (or function) passed will be called.
  //
  // Event handlers are dispatched through a threadpool, and so multiple
  // events can be dispatched simultaneously.
  fn message(&self, ctx: Context, msg: Message) {
    if msg.author.bot {
      // We never examine bot messages.
      return;
    }
    println!("Message Content: {}", msg.content);
    if let Some(response) = bot_handle_this(&msg.content) {
      println!("Response: {}", response);
      if let Err(why) = msg.channel_id.say(&ctx.http, response) {
        println!("Error Sending Response: {:?}", why);
      }
    }
  }

  // Set a handler to be called on the `ready` event. This is called when a
  // shard is booted, and a READY payload is sent by Discord. This payload
  // contains data like the current user's guild Ids, current user data,
  // private channels, and more.
  //
  // In this case, just print what the current user's username is.
  fn ready(&self, _: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
  }
}

fn main() {
  // Configure the client with your Discord bot token in the environment.
  let token = env::var("DISCORD_TOKEN").expect("Couldn't read DISCORD_TOKEN.");

  // Create a new instance of the Client, logging in as a bot. This will
  // automatically prepend your bot token with "Bot ", which is a requirement
  // by Discord for bot users.
  let mut client = Client::new(&token, Handler).expect("Err creating client");

  // Finally, start a single shard, and start listening to events.
  //
  // Shards will automatically attempt to reconnect, and will perform
  // exponential back-off until it reconnects.
  if let Err(why) = client.start() {
    println!("Client error: {:?}", why);
  }
}
