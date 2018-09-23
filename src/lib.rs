#[macro_use]
extern crate serenity;

extern crate randomize;
use randomize::*;

pub mod shadowrun;

pub fn basic_sum_str(s: &str) -> Option<i32> {
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
