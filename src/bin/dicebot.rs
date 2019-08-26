#![allow(unused_imports)]
//#![feature(const_fn)]

extern crate randomize;
use randomize::*;

#[macro_use]
extern crate serenity;
use serenity::{
  client::*,
  client::bridge::gateway::ShardManager,
  framework::standard::*,
  framework::standard::macros::*,
  model::{
    channel::*,
    gateway::*,
    event::*,
    id::*,
  },
  prelude::*,
};

use std::collections::HashSet;
use std::sync::Arc;
use std::collections::HashMap;
use std::fmt::Write;

extern crate dice_bot;
use dice_bot::{earthdawn::*, eote::*, shadowrun::*, *};

use std::process::{Command, Stdio};

// A container type is created for inserting into the Client's `data`, which
// allows for data to be accessible across all events and framework commands, or
// anywhere else that has a copy of the `data` Arc.
struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

pub struct Handler;

impl EventHandler for Handler {
  fn ready(&self, _: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
  }
}

group!({
    name: "general",
    options: {},
    commands: [commands, ddate, after_sundown, dice, thaco, sigil_command, stat2e, champions]
});

fn main() {
  let mut client = Client::new(
    &::std::env::var("DISCORD_TOKEN").expect("Could not obtain DISCORD_TOKEN"),
    Handler,
  )
  .expect("Could not create the client");

  {
    let mut data = client.data.write();
    data.insert::<CommandCounter>(HashMap::default());
    data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
  }


  // We will fetch your bot's id.
  let bot_id = match client.cache_and_http.http.get_current_application_info() {
      Ok(info) => {
          info.id
      },
      Err(why) => panic!("Could not access application info: {:?}", why),
  };

  let userid_str = &::std::env::var("USER_ID").expect("Could not obtain USER_ID");
  let userid : UserId = UserId(userid_str.parse::<u64>().unwrap());

  client.with_framework(
    StandardFramework::new()
      .configure(|c| {
        c.allow_dm(true)
          .with_whitespace(WithWhiteSpace { prefixes: true, groups: true, commands: true })
          .ignore_bots(true)
          .ignore_webhooks(true)
          .on_mention(Some(bot_id))
          .owners(vec![userid].into_iter().collect())
          .prefixes(vec!["zztop"])
          .no_dm_prefix(true)
          .delimiter(" ")
          .case_insensitivity(true)
      })
      .bucket("ddate", |b| b.delay(60))
      .bucket("help", |b| b.delay(30))
      .bucket("complicated", |b| b.delay(5).time_span(30).limit(2))
      .group(&GENERAL_GROUP)
      .group(&SHADOWRUN_GROUP)
      .group(&EOTE_GROUP)
      .group(&EARTHDAWN_GROUP)
      .help(&MY_HELP)
  );

  if let Err(why) = client.start() {
    println!("Client::start error: {:?}", why);
  }
}

#[help]
fn my_help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners)
}

// Commands can be created via the attribute `#[command]` macro.
#[command]
// Options are passed via subsequent attributes.
// Make this command use the "complicated" bucket.
#[bucket = "complicated"]
fn commands(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut contents = "Commands used:\n".to_string();

    let data = ctx.data.read();
    let counter = data.get::<CommandCounter>().expect("Expected CommandCounter in ShareMap.");

    for (k, v) in counter {
        let _ = write!(contents, "- {name}: {amount}\n", name=k, amount=v);
    }

    if let Err(why) = msg.channel_id.say(&ctx.http, &contents) {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

/// Opens a child process to check the `ddate` value.
fn ddate_process() -> Option<String> {
  String::from_utf8(
    Command::new("ddate")
      .stdout(Stdio::piped())
      .spawn()
      .ok()?
      .wait_with_output()
      .ok()?
      .stdout,
  )
  .ok()
}

#[command]
#[description = "https://en.wikipedia.org/wiki/Discordian_calendar"]
#[bucket = "ddate"]
fn ddate(_ctx: &mut Context, msg: &Message, _: Args) -> CommandResult {
  ddate_process().map(|date| {
    if let Err(why) = msg.channel_id.say(&_ctx.http, date) {
      println!("Error sending message: {:?}", why);
    }
  });
  Ok(())
}

#[command]
#[aliases("as")]
#[description = "Rolls After Sundown style"]
#[usage = "DICE [...]"]
fn after_sundown(_ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
  let gen: &mut PCG32 = &mut global_gen();
  let mut output = String::new();
  for dice_count in args.rest().split_whitespace().flat_map(basic_sum_str).take(10) {
    let dice_count = dice_count.max(0).min(5_000) as u32;
    if dice_count > 0 {
      let mut hits = 0;
      const DICE_REPORT_MAXIMUM: u32 = 30;
      let mut dice_record = String::with_capacity(DICE_REPORT_MAXIMUM as usize * 2 + 20);
      dice_record.push_str(" `(");
      for _ in 0 .. dice_count {
        let roll = D6.sample(gen);
        if roll >= 5 {
          hits += 1;
        }
        if dice_count < DICE_REPORT_MAXIMUM {
          dice_record.push(('0' as u8 + roll as u8) as char);
          dice_record.push(',');
        }
      }
      dice_record.pop();
      // I have ABSOLUTELY no idea why we need to put this extra space in here,
      // but we do and that makes the output correct.
      dice_record.push_str(")`");
      let s_for_hits = if hits != 1 {"s"} else {""};
      let dice_report_output = if dice_count < DICE_REPORT_MAXIMUM { &dice_record } else { "" };
      output.push_str(&format!("Rolled {} dice: {} hit{}{}", dice_count, hits, s_for_hits, dice_report_output));
    } else {
      let output = format!("No dice to roll!");
      if let Err(why) = msg.channel_id.say(&_ctx.http, output) {
        println!("Error sending message: {:?}", why);
      }
    }
    output.push('\n');
  }
  output.pop();
  if output.len() > 0 {
    if let Err(why) = msg.channel_id.say(&_ctx.http, output) {
      println!("Error sending message: {:?}", why);
    }
  }
  Ok(())
}

#[command]
#[aliases("roll", "dice")]
#[description = "Rolls a standard dice expression"]
#[usage = "EXPRESSION [...]"]
fn dice(_ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
  let gen: &mut PCG32 = &mut global_gen();
  let mut output = String::new();
  'exprloop: for dice_expression_str in args.rest().split_whitespace().take(20) {
    let plus_only_form = dice_expression_str.replace("-","+-");
    let mut total: i32 = 0;
    let mut sub_expressions = vec![];
    for sub_expression in plus_only_form.split('+').take(70) {
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
                //msg.react(ReactionType::Unicode(EMOJI_QUESTION.to_string())).ok();
                continue 'exprloop;
              }
            }
          } else {
            1
          }
        }
        None => {
          //msg.react(ReactionType::Unicode(EMOJI_QUESTION.to_string())).ok();
          continue 'exprloop;
        }
      };
      let num_sides: u32 = match d_iter.next() {
        Some(num_sides_str) => {
          match num_sides_str.parse::<u32>() {
            Ok(num) => num.min(4_000_000),
            Err(_) => {
              //msg.react(ReactionType::Unicode(EMOJI_QUESTION.to_string())).ok();
              continue 'exprloop;
            }
          }
        }
        None => {
          1
        }
      };
      if d_iter.next().is_some() {
        //msg.react(ReactionType::Unicode(EMOJI_QUESTION.to_string())).ok();
        continue 'exprloop;
      }
      if num_sides == 0 {
        // do nothing with 0-sided dice
      } else if num_sides == 1 {
        total += num_dice;
        sub_expressions.push(format!("{}", num_dice));
      } else {
        let range = match num_sides {
          4 => D4,
          6 => D6,
          8 => D8,
          10 => D10,
          12 => D12,
          20 => D20,
          _ => RandRangeU32::new(1, num_sides)
        };
        if num_dice > 0 {
          for _ in 0 .. num_dice {
            total += range.sample(gen) as i32;
          }
          sub_expressions.push(format!("{}d{}", num_dice, num_sides));
        } else if num_dice < 0 {
          for _ in 0 .. num_dice.abs() {
            total -= range.sample(gen) as i32;
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
      output.push_str(&format!("Rolled {}: {}\n",parsed_string, total));
    } else {
      //msg.react(ReactionType::Unicode(EMOJI_QUESTION.to_string())).ok();
    }
  }
  output.pop();
  if output.len() > 0 {
    if let Err(why) = msg.channel_id.say(&_ctx.http, output) {
      println!("Error sending message: {:?}", why);
    }
  }
  Ok(())
}

#[command]
#[description = "Does a THACO attack roll"]
#[usage = "THACO [...]"]
#[aliases("thaco", "taco")]
fn thaco(_ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
  let gen: &mut PCG32 = &mut global_gen();
  let mut output = String::new();
  for thaco_value in args.rest().split_whitespace().flat_map(basic_sum_str).take(20) {
    let roll = D20.sample(gen) as i32;
    output.push_str(&format!("THACO {}: Rolled {}, Hits AC {} or greater.\n", thaco_value, roll, thaco_value - roll));
  }
  output.pop();
  if output.len() > 0 {
    if let Err(why) = msg.channel_id.say(&_ctx.http, output) {
      println!("Error sending message: {:?}", why);
    }
  }
  Ok(())
}

#[command]
#[description = "It does a mystery thing that Sigil decided upon"]
#[aliases("sigil")]
#[usage = "BASIC_SUM_STRING [...]"]
fn sigil_command(_ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
  let gen: &mut PCG32 = &mut global_gen();
  let mut output = String::new();
  let terms: Vec<i32> = args.rest().split_whitespace().filter_map(basic_sum_str).collect();
  for term in terms {
    let x = term.abs();
    if x > 0 {
      let mut total : i32 = 0;
      for _ in 0 .. x {
        total += D6.sample(gen) as i32;
        total -= D6.sample(gen) as i32;
      }
      output.push_str(&format!("Rolling Sigil {}: {}\n", x, total.abs()));
    } else {
      output.push_str(&format!("Rolling Sigil {}: 0\n", x));
    }
  }
  output.pop();
  if output.len() > 0 {
    if let Err(why) = msg.channel_id.say(&_ctx.http, output) {
      println!("Error sending message: {:?}", why);
    }
  } else {
    if let Err(why) = msg.channel_id.say(&_ctx.http, "usage: sigil NUMBER") {
      println!("Error sending message: {:?}", why);
    }
  }
  Ok(())
}

#[command]
#[description = "Rolls a 2e stat array"]
#[aliases("stat2e")]
fn stat2e(_ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
  let gen: &mut PCG32 = &mut global_gen();
  let mut output = String::new();
  let roll = |gen: &mut PCG32| {
    4 + D4.sample(gen) + D4.sample(gen) + D4.sample(gen) + D4.sample(gen)
  };
  output.push_str(&format!("Str: {}\n", roll(gen)));
  output.push_str(&format!("Dex: {}\n", roll(gen)));
  output.push_str(&format!("Con: {}\n", roll(gen)));
  output.push_str(&format!("Int: {}\n", roll(gen)));
  output.push_str(&format!("Wis: {}\n", roll(gen)));
  output.push_str(&format!("Cha: {}\n", roll(gen)));
  output.pop();
  if let Err(why) = msg.channel_id.say(&_ctx.http, output) {
    println!("Error sending message: {:?}", why);
  }
  Ok(())
}

#[command]
#[aliases("champ")]
#[description = "Rolls a Champions roll"]
#[usage = "EXPRESSION [...]"]
fn champions(_ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
  let gen: &mut PCG32 = &mut global_gen();
  let mut output = String::new();
  let terms: Vec<i32> = args.rest().split_whitespace().filter_map(basic_sum_str).collect();
  for term in terms {
    let mut rolls = [0; 3];
    for roll_mut in rolls.iter_mut() {
      *roll_mut = D6.sample(gen) as i32;
    }
    output.push_str(&format!("Rolling Champions {}: {}, [{},{},{}]\n",
      term,
      if rolls.iter().cloned().sum::<i32>() < term { "Success" } else { "Failure" },
      rolls[0],
      rolls[1],
      rolls[2]
      )
    );
  }
  output.pop();
  if output.len() > 0 {
    if let Err(why) = msg.channel_id.say(&_ctx.http, output) {
      println!("Error sending message: {:?}", why);
    }
  } else {
    if let Err(why) = msg.channel_id.say(&_ctx.http, "usage: sigil NUMBER") {
      println!("Error sending message: {:?}", why);
    }
  }
  Ok(())
}
