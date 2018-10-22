#[macro_use]
extern crate serenity;

extern crate randomize;
use randomize::*;

pub mod earthdawn;
pub mod eote;
pub mod shadowrun;

trait ExplodingRange {
  fn explode(&self, &mut PCG32) -> u32;
}

impl ExplodingRange for RandRangeU32 {
  fn explode(&self, gen: &mut PCG32) -> u32 {
    let mut times = 0;
    loop {
      let roll = self.sample(gen);
      if roll == self.high() {
        times += 1;
        continue;
      } else {
        return self.high() * times + roll;
      }
    }
  }
}

pub fn basic_sum_str(s: &str) -> Option<i32> {
  if s.len() == 0 {
    return None;
  }
  let mut total = 0;
  let mut current = 0;
  let mut current_is_negative = s.chars().nth(0).unwrap() == '-';
  for ch in s.chars() {
    match ch {
      '0' ... '9' => {
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
