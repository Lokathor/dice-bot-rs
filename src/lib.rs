#![allow(non_upper_case_globals)]
#![allow(unused_labels)]
#![allow(clippy::comparison_chain)]

use core::{fmt::Write, ops::Not};

mod after_sundown;
use after_sundown::after_sundown;

mod champions;
use champions::champions;

mod dice;
use dice::dice;

mod trek;
use trek::trek;

mod letters;
use letters::letters;

mod earthdawn;
use earthdawn::{earthdawn, earthdawn_karma, earthdawn_target};

mod eote;
use eote::eote;

mod global_gen;
use global_gen::global_gen;

mod shadowrun;
use shadowrun::{
  shadowrun, shadowrun_attack, shadowrun_edge, shadowrun_foe, shadowrun_friend,
};

mod stat2e;
use stat2e::stat2e;

mod sigil;
use sigil::sigil;

mod thaco;
use thaco::thaco;

mod warhammer;
use warhammer::warhammer;

// // //

pub fn bot_handle_this(message: &str) -> Option<String> {
  // remove this if we decide to have more than one prefix
  if message.starts_with(',').not() {
    return None;
  }

  let (cmd, args) = message
    .find(char::is_whitespace)
    .map(|index| {
      let (c, a) = message.split_at(index);
      (c, a.trim())
    })
    .unwrap_or((message, ""));

  Some(match cmd {
    ",dice" | ",roll" => dice(args),
    ",thaco" | ",taco" => thaco(args),
    ",champ" => champions(args),
    ",stat2e" => stat2e(),
    ",sigil" => sigil(args),
    ",as" => after_sundown(args),
    ",eote" => eote(args),
    ",ed" => earthdawn(args),
    ",edk" => earthdawn_karma(args),
    ",edt" => earthdawn_target(args),
    ",sr" => shadowrun(args),
    ",sre" => shadowrun_edge(args),
    ",sra" => shadowrun_attack(args),
    ",friend" => shadowrun_friend(args),
    ",foe" => shadowrun_foe(args),
    ",letters" => letters(args),
    ",trek" => trek(args),
    ",wh" => warhammer(args),
    _ => return None,
  })
}

fn basic_sum_str(s: &str) -> Option<i32> {
  let s = s.trim();
  if s.is_empty() {
    return None;
  }
  let mut total = 0;
  let mut current = 0;
  let mut current_is_negative = s.starts_with('-');
  for ch in s.chars() {
    match ch {
      '0'..='9' => {
        current *= 10;
        current += ch.to_digit(10).unwrap() as i32;
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
