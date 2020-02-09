use core::fmt::Write;
use core::ops::Not;

use randomize::{RandRangeU32, PCG32};

mod dice;
use dice::dice;

mod champions;
use champions::champions;

mod stat2e;
use stat2e::stat2e;

mod global_gen;
use global_gen::global_gen;

/*

TODO:
* Dice Pools: sr / sre / sra / friend / foe / as
* Steps: ed / edk / edt
* Standard: thaco / taco / eote / champ / stat2e
* Weirdness: sigil

*/

const d4: RandRangeU32 = RandRangeU32::new(1, 4);
const d6: RandRangeU32 = RandRangeU32::new(1, 6);
const d8: RandRangeU32 = RandRangeU32::new(1, 8);
const d10: RandRangeU32 = RandRangeU32::new(1, 10);
const d12: RandRangeU32 = RandRangeU32::new(1, 12);
const d20: RandRangeU32 = RandRangeU32::new(1, 20);

pub fn bot_handle_this(message: &str) -> Option<String> {
  if message.starts_with(",dice") || message.starts_with(",roll") {
    return Some(dice(message[5..].trim()));
  } else if message.starts_with(",champ") {
    return Some(champions(message[6..].trim()));
  } else if message.starts_with(",stat2e") {
    return Some(stat2e());
  }
  None
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
