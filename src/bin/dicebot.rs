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
      .command("as", |c| {
        c.cmd(after_sundown).desc("Rolls After Sundown style").usage("[DICE_COUNT] [...]")
      })
      .command("sr", |c| c.cmd(shadowrun).desc("Rolls Shadowrun 4e style").usage("[DICE_COUNT] [...]"))
      .command("sre", |c| {
        c.cmd(shadowrun)
          .desc("Rolls Shadowrun 4e+Edge style (6-again)")
          .usage("[DICE_COUNT] [...]")
      })
      .command("dice", |c| {
        c.cmd(dice).desc("Rolls a standard dice expression").usage("[DICE_EXPRESSION] [...]")
      })
      .simple_bucket("help", 30)
      .help(help_commands::with_embeds),
  );

  if let Err(why) = client.start() {
    println!("Client::start error: {:?}", why);
  }
}

#[allow(dead_code)]
fn owner_check(_: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> bool {
  msg.author.id == LOKATHOR_ID
}

command!(echo(_ctx, msg, args) {
  msg.channel_id.say(format!("{}",args.full())).ok();
});

command!(after_sundown(_ctx, msg, args) {
  let gen: &mut PCG32 = &mut get_global_generator();
  for arg in args.iter::<u32>().take(5) {
    match arg {
      Ok(dice_count) => {
        let dice_count = dice_count.min(5_000);
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
  }
});

command!(shadowrun(_ctx, msg, args) {
  let gen: &mut PCG32 = &mut get_global_generator();
  for arg in args.iter::<u32>().take(5) {
    match arg {
      Ok(dice_count) => {
        let dice_count = dice_count.min(5_000);
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
        let output = format!("Rolled {} dice: {}{} hit{}", dice_count, match (hits, is_glitch) {
          (0, true) => "CRITICAL GLITCH, ",
          (_, true) => "GLITCH, ",
          _ => "",
        }, hits, if hits != 1 {"s"} else {""});
        if let Err(why) = msg.channel_id.say(output) {
          println!("Error sending message: {:?}", why);
        }
      },
      Err(_) => {
        msg.react(ReactionType::Unicode(EMOJI_QUESTION.to_string())).ok();
      }
    }
  }
});

command!(shadowrun_edge(_ctx, msg, args) {
  let gen: &mut PCG32 = &mut get_global_generator();
  for arg in args.iter::<u32>().take(5) {
    match arg {
      Ok(dice_count) => {
        let dice_count = dice_count.min(5_000);
        let mut hits = 0;
        let mut ones = 0;
        let mut dice_rolled = 0;
        while dice_rolled < dice_count {
          let roll = d6.sample_with(gen);
          if roll == 1 {
            ones += 1;
          } else if roll > 4 {
            hits += 1;
            if roll == 6 {
              continue;
            }
          }
          dice_rolled += 1;
        }
        let is_glitch = ones >= (dice_count+1) / 2;
        let output = format!("Rolled {} dice with 6-again: {}{} hit{}", dice_count, match (hits, is_glitch) {
          (0, true) => "CRITICAL GLITCH, ",
          (_, true) => "GLITCH, ",
          _ => "",
        }, hits, if hits != 1 {"s"} else {""});
        if let Err(why) = msg.channel_id.say(output) {
          println!("Error sending message: {:?}", why);
        }
      },
      Err(_) => {
        msg.react(ReactionType::Unicode(EMOJI_QUESTION.to_string())).ok();
      }
    }
  }
});

command!(dice(_ctx, msg, args) {
  let gen: &mut PCG32 = &mut get_global_generator();
  'exprloop: for dice_expression_str in args.full().split_whitespace().take(5) {
    let mut plus_only_form = dice_expression_str.replace("-","+-");
    let mut total: i32 = 0;
    let mut sub_expressions = vec![];
    for sub_expression in plus_only_form.split('+').take(7) {
      if sub_expression.len() == 0 {
        continue;
      }
      let mut d_iter = sub_expression.split('d');
      let num_dice: i32 = match d_iter.next() {
        Some(num_dice_str) => {
          if num_dice_str.len() > 0 {
            match num_dice_str.parse::<i32>() {
              Ok(num) => num.max(-5_000).min(5_000),
              Err(_) => {
                msg.react(ReactionType::Unicode(EMOJI_QUESTION.to_string())).ok();
                continue 'exprloop;
              }
            }
          } else {
            1
          }
        }
        None => {
          msg.react(ReactionType::Unicode(EMOJI_QUESTION.to_string())).ok();
          continue 'exprloop;
        }
      };
      let num_sides: u32 = match d_iter.next() {
        Some(num_sides_str) => {
          match num_sides_str.parse::<u32>() {
            Ok(num) => num.min(4_000_000),
            Err(_) => {
              msg.react(ReactionType::Unicode(EMOJI_QUESTION.to_string())).ok();
              continue 'exprloop;
            }
          }
        }
        None => {
          1
        }
      };
      if d_iter.next().is_some() {
        msg.react(ReactionType::Unicode(EMOJI_QUESTION.to_string())).ok();
        continue 'exprloop;
      }
      if num_sides == 0 {
        // do nothing with 0-sided dice
      } else if num_sides == 1 {
        total += num_dice;
        sub_expressions.push(format!("{}", num_dice));
      } else {
        let range = match num_sides {
          4 => d4,
          6 => d6,
          8 => d8,
          10 => d10,
          12 => d12,
          20 => d20,
          _ => RandRangeInclusive32::new(1,num_sides)
        };
        if num_dice > 0 {
          for _ in 0 .. num_dice {
            total += range.sample_with(gen) as i32;
          }
          sub_expressions.push(format!("{}d{}", num_dice, num_sides));
        } else if num_dice < 0 {
          for _ in 0 .. num_dice.abs() {
            total -= range.sample_with(gen) as i32;
          }
          sub_expressions.push(format!("{}d{}", num_dice, num_sides));
        }
        // do nothing if num_dice == 0
      }
    }
    if sub_expressions.len() > 0 {
      let mut parsed_string = sub_expressions[0].clone();
      for sub_expression in sub_expressions.into_iter().skip(1) {
        if sub_expression.chars().nth(0) == Some('-') {
          parsed_string.push_str(&sub_expression);
        } else {
          parsed_string.push('+');
          parsed_string.push_str(&sub_expression);
        }
      }
      let output = format!("Rolled {}: {}",parsed_string, total);
      if let Err(why) = msg.channel_id.say(output) {
        println!("Error sending message: {:?}", why);
      }
    } else {
      msg.react(ReactionType::Unicode(EMOJI_QUESTION.to_string())).ok();
    }
  }
});
