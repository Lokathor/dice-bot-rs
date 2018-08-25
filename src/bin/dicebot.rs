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

pub(crate) fn basic_sum_str(s: &str) -> Option<i32> {
  if s.len() == 0 {
    return None;
  }
  let mut total = 0;
  let mut current = 0;
  let mut current_is_negative = s.chars().nth(0).unwrap() == '-';
  for ch in s.chars() {
    match ch {
      '0' => {
        current *= 10;
      }
      '1' => {
        current *= 10;
        current += 1
      }
      '2' => {
        current *= 10;
        current += 2
      }
      '3' => {
        current *= 10;
        current += 3
      }
      '4' => {
        current *= 10;
        current += 4
      }
      '5' => {
        current *= 10;
        current += 5
      }
      '6' => {
        current *= 10;
        current += 6
      }
      '7' => {
        current *= 10;
        current += 7
      }
      '8' => {
        current *= 10;
        current += 8
      }
      '9' => {
        current *= 10;
        current += 9
      }
      '+' => {
        total += if current_is_negative { -current } else { current };
        current = 0;
        current_is_negative = false;
      }
      '-' => {
        total += if current_is_negative { -current } else { current };
        current = 0;
        current_is_negative = true;
      }
      _ => return None,
    };
  }
  total += if current_is_negative { -current } else { current };
  Some(total)
}

#[test]
fn basic_sum_str_test() {
  assert_eq!(basic_sum_str("1"), Some(1));
  assert_eq!(basic_sum_str("12"), Some(12));
  assert_eq!(basic_sum_str("4+5"), Some(9));
  assert_eq!(basic_sum_str("8-2"), Some(6));
  assert_eq!(basic_sum_str("abc"), None);
  assert_eq!(basic_sum_str("-2"), Some(-2));
  assert_eq!(basic_sum_str("-2+7"), Some(5));
}

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
        c.cmd(shadowrun_edge)
          .desc("Rolls Shadowrun 4e+Edge style (6-again)")
          .usage("[DICE_COUNT] [...]")
      })
      .command("ed", |c| c.cmd(earthdawn).desc("Rolls an Earthdawn 4e step").usage("[DICE_COUNT] [...]"))
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

command!(after_sundown(_ctx, msg, args) {
  let gen: &mut PCG32 = &mut get_global_generator();
  for arg in args.full().split_whitespace().take(5).map(basic_sum_str) {
    match arg {
      Some(dice_count) => {
        let dice_count = dice_count.max(0).min(5_000) as u32;
        let mut hits = 0;
        const DICE_REPORT_MAXIMUM: u32 = 30;
        let mut dice_record = String::with_capacity(DICE_REPORT_MAXIMUM as usize * 2 + 20);
        dice_record.push(' ');
        dice_record.push('`');
        dice_record.push('(');
        for _ in 0 .. dice_count {
          let roll = d6.sample_with(gen);
          if roll >= 5 {
            hits += 1;
          }
          if dice_count < DICE_REPORT_MAXIMUM {
            dice_record.push(('0' as u8 + roll as u8) as char);
            dice_record.push(',');
          }
        }
        dice_record.pop();
        dice_record.push(')');
        dice_record.push('`');
        let s_for_hits = if hits != 1 {"s"} else {""};
        let dice_report_output = if dice_count < DICE_REPORT_MAXIMUM { &dice_record } else { "" };
        let output = format!("Rolled {} dice: {} hit{}{}", dice_count, hits, s_for_hits, dice_report_output);
        if let Err(why) = msg.channel_id.say(output) {
          println!("Error sending message: {:?}", why);
        }
      },
      None => {
        msg.react(ReactionType::Unicode(EMOJI_QUESTION.to_string())).ok();
      }
    }
  }
});

command!(shadowrun(_ctx, msg, args) {
  let gen: &mut PCG32 = &mut get_global_generator();
  for arg in args.full().split_whitespace().take(5).map(basic_sum_str) {
    match arg {
      Some(dice_count) => {
        let dice_count = dice_count.max(0).min(5_000) as u32;
        let mut hits = 0;
        let mut ones = 0;
        const DICE_REPORT_MAXIMUM: u32 = 30;
        let mut dice_record = String::with_capacity(DICE_REPORT_MAXIMUM as usize * 2 + 20);
        dice_record.push(' ');
        dice_record.push('`');
        dice_record.push('(');
        for _ in 0 .. dice_count {
          let roll = d6.sample_with(gen);
          if roll == 1 {
            ones += 1;
          } else if roll >= 5 {
            hits += 1;
          }
          if dice_count < DICE_REPORT_MAXIMUM {
            dice_record.push(('0' as u8 + roll as u8) as char);
            dice_record.push(',');
          }
        }
        dice_record.pop();
        dice_record.push(')');
        dice_record.push('`');
        let is_glitch = ones >= (dice_count+1) / 2;
        let glitch_string = match (hits, is_glitch) {
          (0, true) => "CRITICAL GLITCH, ",
          (_, true) => "GLITCH, ",
          _ => "",
        };
        let s_for_hits = if hits != 1 {"s"} else {""};
        let dice_report_output = if dice_count < DICE_REPORT_MAXIMUM { &dice_record } else { "" };
        let output = format!("Rolled {} dice: {}{} hit{}{}", dice_count, glitch_string, hits, s_for_hits, dice_report_output);
        if let Err(why) = msg.channel_id.say(output) {
          println!("Error sending message: {:?}", why);
        }
      },
      None => {
        msg.react(ReactionType::Unicode(EMOJI_QUESTION.to_string())).ok();
      }
    }
  }
});

command!(shadowrun_edge(_ctx, msg, args) {
  let gen: &mut PCG32 = &mut get_global_generator();
  for arg in args.full().split_whitespace().take(5).map(basic_sum_str) {
    match arg {
      Some(dice_count) => {
        let dice_count = dice_count.max(0).min(5_000) as u32;
        let mut hits = 0;
        let mut ones = 0;
        let mut dice_rolled = 0;
        const DICE_REPORT_MAXIMUM: u32 = 30;
        let mut dice_record = String::with_capacity(DICE_REPORT_MAXIMUM as usize * 2 + 20);
        dice_record.push(' ');
        dice_record.push('`');
        dice_record.push('(');
        let mut this_is_a_normal_roll = true;
        while dice_rolled < dice_count {
          let roll = d6.sample_with(gen);
          if roll == 1 && this_is_a_normal_roll {
            ones += 1;
          } else if roll >= 5 {
            hits += 1;
          }
          if dice_count < DICE_REPORT_MAXIMUM {
            dice_record.push(('0' as u8 + roll as u8) as char);
            dice_record.push(',');
          }
          if roll == 6 {
            // setup the next pass to be a bonus die
            this_is_a_normal_roll = false;
          } else {
            dice_rolled += 1;
            this_is_a_normal_roll = true;
          }
        }
        dice_record.pop();
        dice_record.push(')');
        dice_record.push('`');
        let is_glitch = ones >= (dice_count+1) / 2;
        let glitch_string = match (hits, is_glitch) {
          (0, true) => "CRITICAL GLITCH, ",
          (_, true) => "GLITCH, ",
          _ => "",
        };
        let s_for_hits = if hits != 1 {"s"} else {""};
        let dice_report_output = if dice_count < DICE_REPORT_MAXIMUM { &dice_record } else { "" };
        let output = format!("Rolled {} dice with 6-again: {}{} hit{}{}", dice_count, glitch_string, hits, s_for_hits, dice_report_output);
        if let Err(why) = msg.channel_id.say(output) {
          println!("Error sending message: {:?}", why);
        }
      },
      None => {
        msg.react(ReactionType::Unicode(EMOJI_QUESTION.to_string())).ok();
      }
    }
  }
});

command!(earthdawn(_ctx, msg, args) {
  let gen: &mut PCG32 = &mut get_global_generator();
  for arg in args.full().split_whitespace().take(5).map(basic_sum_str) {
    match arg {
      Some(step_value) => {
        let step_roll = step(gen, step_value);
        let output = format!("Rolled step {}: {}", step_value, step_roll);
        if let Err(why) = msg.channel_id.say(output) {
          println!("Error sending message: {:?}", why);
        }
      },
      None => {
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

trait ExplodingRange {
  fn explode(&self, &mut PCG32) -> u32;
}

impl ExplodingRange for RandRangeInclusive32 {
  fn explode(&self, gen: &mut PCG32) -> u32 {
    let mut times = 0;
    loop {
      let roll = self.sample_with(gen);
      if roll == self.high() {
        times += 1;
        continue;
      } else {
        return self.high() * times + roll;
      }
    }
  }
}

/// Rolls a step roll, according to the 4th edition chart.
pub fn step(gen: &mut PCG32, mut step: i32) -> i32 {
  if step < 1 {
    0
  } else {
    let mut total = 0;
    while step > 13 {
      total += d12.explode(gen);
      step -= 7;
    }
    (total + match step {
      1 => (d4.explode(gen) as i32 - 2).max(1) as u32,
      2 => (d4.explode(gen) as i32 - 1).max(1) as u32,
      3 => d4.explode(gen),
      4 => d6.explode(gen),
      5 => d8.explode(gen),
      6 => d10.explode(gen),
      7 => d12.explode(gen),
      8 => d6.explode(gen) + d6.explode(gen),
      9 => d8.explode(gen) + d6.explode(gen),
      10 => d8.explode(gen) + d8.explode(gen),
      11 => d10.explode(gen) + d8.explode(gen),
      12 => d10.explode(gen) + d10.explode(gen),
      13 => d12.explode(gen) + d10.explode(gen),
      _other => unreachable!(),
    }) as i32
  }
}
