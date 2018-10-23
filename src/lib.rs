#[macro_use]
extern crate serenity;

extern crate randomize;
use randomize::*;

extern crate meval;

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
  if s.contains("/") {
      return None
  }
  if s.contains("*") {
      return None
  }
  match meval::eval_str(s) {
    Ok(x) => Some(x as i32),
    _ => None,
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn basic_sum_str_test_nums() {
    assert_eq!(basic_sum_str("1"), Some(1));
    assert_eq!(basic_sum_str("12"), Some(12));
    assert_eq!(basic_sum_str("-2"), Some(-2));
  }

  #[test]
  fn basic_sum_str_test_equations() {
    assert_eq!(basic_sum_str("-2+7"), Some(5));
    assert_eq!(basic_sum_str("8-2"), Some(6));
    assert_eq!(basic_sum_str("4+5"), Some(9));
  }

  #[test]
  fn basic_sum_str_test_too_many_operands() {
    assert_eq!(basic_sum_str("--23"), Some(23));
    assert_eq!(basic_sum_str("++54"), Some(54));
    assert_eq!(basic_sum_str("-------123"), Some(-123));
  }

  #[test]
  fn basic_sum_str_test_not_an_expression() {
    assert_eq!(basic_sum_str("abc"), None);
    assert_eq!(basic_sum_str("😝"), None);
  }

  #[test]
  fn basic_sum_str_no_mult_or_div() {
      assert_eq!(basic_sum_str("3/2"), None);
      assert_eq!(basic_sum_str("45*3.145"), None);
  }
}
