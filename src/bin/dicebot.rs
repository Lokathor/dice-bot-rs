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

extern crate dice_bot;
use dice_bot::earthdawn::*;
use dice_bot::eote::*;
use dice_bot::shadowrun::*;
use dice_bot::*;

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
      // Shadowrun
      .command("sr", |c| c.cmd(shadowrun).desc("Rolls Shadowrun 4e style (up to 10)").usage("DICE [...]"))
      .command("sre", |c| {
        c.cmd(shadowrun_edge)
          .desc("Rolls Shadowrun 4e with 6-again (up to 10)")
          .usage("DICE [...]")
      })
      .command("sra", |c| {
        c.cmd(shadowrun_attack).desc("Rolls a Shadowrun 4e attack cycle").usage("ATTACK EVADE DAMAGE SOAK")
      })
      .command("friend", |c| {
        c.cmd(shadowrun_friend)
          .desc("Rolls up a conjured buddy (Spirit / Sprite)")
          .usage("CONJURE FORCE SOAK")
      })
      .command("foe", |c| {
        c.cmd(shadowrun_foe)
          .desc("Binds a conjured buddy (Spirit / Sprite)")
          .usage("BINDING FORCE SOAK")
      })
      // Earthdawn
      .command("ed", |c| {
        c.cmd(earthdawn).desc("Rolls an Earthdawn 4e step (up to 10)").usage("STEP [...]")
      })
      .command("edk", |c| {
        c.cmd(earthdawn_karma)
          .desc("Rolls an Earthdawn 4e step with karma (up to 10)")
          .usage("STEP [...]")
      })
      .command("edt", |c| c.cmd(earthdawn_target).desc("Rolls an Earthdawn 4e step").usage("STEP TARGET"))
      // Other
      .command("as", |c| c.cmd(after_sundown).desc("Rolls After Sundown style").usage("DICE [...]"))
      .command("dice", |c| c.cmd(dice).desc("Rolls a standard dice expression").usage("EXPRESSION [...]"))
      .command("eote", |c| c.cmd(eote).desc("Rolls EotE dice (b=black, u=blue)").usage("EXPRESSION [...]"))
      // User Commands
      .command("sigil", |c| c.cmd(sigil_command).desc("It does a mystery thing that Sigil decided upon").usage("BASIC_SUM_STRING [...]"))
      .simple_bucket("help", 30)
      .help(help_commands::with_embeds),
  );

  if let Err(why) = client.start() {
    println!("Client::start error: {:?}", why);
  }
}

command!(after_sundown(_ctx, msg, args) {
  let gen: &mut PCG32 = &mut global_gen();
  let mut output = String::new();
  for dice_count in args.full().split_whitespace().flat_map(basic_sum_str).take(10) {
    let dice_count = dice_count.max(0).min(5_000) as u32;
    if dice_count > 0 {
      let mut hits = 0;
      const DICE_REPORT_MAXIMUM: u32 = 30;
      let mut dice_record = String::with_capacity(DICE_REPORT_MAXIMUM as usize * 2 + 20);
      dice_record.push_str(" `(");
      for _ in 0 .. dice_count {
        let roll = d6.sample(gen);
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
      dice_record.push_str(")` ");
      println!("{}", dice_record);
      let s_for_hits = if hits != 1 {"s"} else {""};
      let dice_report_output = if dice_count < DICE_REPORT_MAXIMUM { &dice_record } else { "" };
      output.push_str(&format!("Rolled {} dice: {} hit{}{}", dice_count, hits, s_for_hits, dice_report_output));
    } else {
      let output = format!("No dice to roll!");
      if let Err(why) = msg.channel_id.say(output) {
        println!("Error sending message: {:?}", why);
      }
    }
  }
  output.pop();
  if output.len() > 0 {
    if let Err(why) = msg.channel_id.say(output) {
      println!("Error sending message: {:?}", why);
    }
  }
});

command!(dice(_ctx, msg, args) {
  let gen: &mut PCG32 = &mut global_gen();
  let mut output = String::new();
  'exprloop: for dice_expression_str in args.full().split_whitespace().take(10) {
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
          4 => d4,
          6 => d6,
          8 => d8,
          10 => d10,
          12 => d12,
          20 => d20,
          _ => RandRangeU32::new(1..=num_sides)
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
    if let Err(why) = msg.channel_id.say(output) {
      println!("Error sending message: {:?}", why);
    }
  }
});

command!(sigil_command(_ctx, msg, args) {
  let gen: &mut PCG32 = &mut global_gen();
  let mut output = String::new();
  let terms: Vec<i32> = args.full().split_whitespace().filter_map(basic_sum_str).collect();
  for term in terms {
    let x = term.abs();
    if x > 0 {
      let mut total = 0;
      for _ in 0 .. x {
        total += d6.sample(gen) as i32;
        total -= d6.sample(gen) as i32;
      }
      output.push_str(&format!("Rolling Sigil {}: {}\n", x, total.abs()));
    } else {
      output.push_str(&format!("Rolling Sigil {}: 0\n", x));
    }
  }
  output.pop();
  if output.len() > 0 {
    if let Err(why) = msg.channel_id.say(output) {
      println!("Error sending message: {:?}", why);
    }
  } else {
    if let Err(why) = msg.channel_id.say("usage: sigil NUMBER") {
      println!("Error sending message: {:?}", why);
    }
  }
});
