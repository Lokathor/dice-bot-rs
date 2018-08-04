#![allow(unused_imports)]

extern crate randomize;
use randomize::*;

#[macro_use]
extern crate serenity;
use serenity::framework::standard::*;
use serenity::model::channel::{Message, ReactionType};
use serenity::model::gateway::Ready;
use serenity::model::id::UserId;
use serenity::prelude::*;

pub const EMOJI_CHECK: &str = "☑";
pub const EMOJI_QUESTION: &str = "❓";
pub const LOKATHOR_ID: UserId = UserId(244106113321140224);

pub struct Handler;

impl EventHandler for Handler {
  fn ready(&self, _: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
  }
}

fn main() {
  let mut client =
    Client::new(&::std::env::var("DISCORD_TOKEN").expect("Could not obtain DISCORD_TOKEN"), Handler).expect("Could not create the client");
  client.with_framework(
    StandardFramework::new()
      .configure(|c| {
        c.allow_dm(true)
          .allow_whitespace(true)
          .ignore_bots(true)
          .ignore_webhooks(true)
          .on_mention(true)
          .owners(vec![LOKATHOR_ID].into_iter().collect())
          .prefixes(vec!["?", ",", "!"])
          .no_dm_prefix(true)
          .delimiter(" ")
          .case_insensitivity(true)
      })
      .command("as", |c| c.cmd(after_sundown))
      .command("sr", |c| c.cmd(shadowrun)),
  );

  if let Err(why) = client.start() {
    println!("Client error: {:?}", why);
  }
}

command!(after_sundown(_ctx, msg, args) {
  match args.single::<u32>() {
    Ok(dice_count) => {
      //msg.react(ReactionType::Unicode(EMOJI_CHECK.to_string())).ok();
      let gen: &mut PCG32 = &mut get_global_generator();
      let mut hits = 0;
      for _ in 0 .. dice_count {
        if d6.sample_with(gen) > 4 {
          hits += 1;
        }
      }
      if let Err(why) = msg.channel_id.say(format!("Rolled {} dice, got {} hit{}", dice_count, hits, if hits != 1 {"s"} else {""})) {
        println!("Error sending message: {:?}", why);
      }
    },
    Err(_) => {
      msg.react(ReactionType::Unicode(EMOJI_QUESTION.to_string())).ok();
    }
  }
});

command!(shadowrun(_ctx, msg, args) {
  match args.single::<u32>() {
    Ok(dice_count) => {
      //msg.react(ReactionType::Unicode(EMOJI_CHECK.to_string())).ok();
      let gen: &mut PCG32 = &mut get_global_generator();
      let mut hits = 0;
      let mut ones = 0;
      for _ in 0 .. dice_count {
        let roll = d6.sample_with(gen);
        if roll == 1 {
          ones += 1;
        } else if roll > 4 {
          hits += 1;
        }
      }
      let is_glitch = ones >= (dice_count+1) / 2;
      let output = format!("Rolled {} dice: {}, {} hit{}", dice_count, match (hits, is_glitch) {
        (0, true) => "CRITICAL GLITCH",
        (_, true) => "GLITCH",
        (0, false) => "failure",
        (_, false) => "success",
      }, hits, if hits != 1 {"s"} else {""});
      if let Err(why) = msg.channel_id.say(output) {
        println!("Error sending message: {:?}", why);
      }
    },
    Err(_) => {
      msg.react(ReactionType::Unicode(EMOJI_QUESTION.to_string())).ok();
    }
  }
});
