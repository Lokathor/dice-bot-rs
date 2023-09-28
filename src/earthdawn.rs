use randomize::{BoundedRandU32, Gen32};

use crate::global_gen::GlobalGen;

use super::*;

trait ExplodeGen32 {
  fn xpl4(&mut self) -> i32;
  fn xpl6(&mut self) -> i32;
  fn xpl8(&mut self) -> i32;
  fn xpl10(&mut self) -> i32;
  fn xpl12(&mut self) -> i32;
}

impl<T: Gen32> ExplodeGen32 for T {
  fn xpl4(&mut self) -> i32 {
    let mut x = BoundedRandU32::_4.sample(|| self.next_u32()) as i32;
    let mut t = 0_i32;
    while x == 4 {
      t += 1;
      x = BoundedRandU32::_4.sample(|| self.next_u32()) as i32;
    }
    (t * 4) + x
  }
  fn xpl6(&mut self) -> i32 {
    let mut x = BoundedRandU32::_6.sample(|| self.next_u32()) as i32;
    let mut t = 0_i32;
    while x == 6 {
      t += 1;
      x = BoundedRandU32::_6.sample(|| self.next_u32()) as i32;
    }
    (t * 6) + x
  }
  fn xpl8(&mut self) -> i32 {
    let mut x = BoundedRandU32::_8.sample(|| self.next_u32()) as i32;
    let mut t = 0_i32;
    while x == 8 {
      t += 1;
      x = BoundedRandU32::_8.sample(|| self.next_u32()) as i32;
    }
    (t * 8) + x
  }
  fn xpl10(&mut self) -> i32 {
    let mut x = BoundedRandU32::_10.sample(|| self.next_u32()) as i32;
    let mut t = 0_i32;
    while x == 10 {
      t += 1;
      x = BoundedRandU32::_10.sample(|| self.next_u32()) as i32;
    }
    (t * 10) + x
  }
  fn xpl12(&mut self) -> i32 {
    let mut x = BoundedRandU32::_12.sample(|| self.next_u32()) as i32;
    let mut t = 0_i32;
    while x == 12 {
      t += 1;
      x = BoundedRandU32::_12.sample(|| self.next_u32()) as i32;
    }
    (t * 12) + x
  }
}

/// Rolls a step roll, according to the 4th edition chart.
pub fn step(gen: &mut GlobalGen, mut step: i32, karma: bool) -> i32 {
  if step < 1 {
    0
  } else {
    let mut total = 0;
    while step > 13 {
      total += gen.xpl12();
      step -= 7;
    }
    (total
      + match step {
        1 => (gen.xpl4() - 2).max(1),
        2 => (gen.xpl4() - 1).max(1),
        3 => gen.xpl4(),
        4 => gen.xpl6(),
        5 => gen.xpl8(),
        6 => gen.xpl10(),
        7 => gen.xpl12(),
        8 => gen.xpl6() + gen.xpl6(),
        9 => gen.xpl8() + gen.xpl6(),
        10 => gen.xpl8() + gen.xpl8(),
        11 => gen.xpl10() + gen.xpl8(),
        12 => gen.xpl10() + gen.xpl10(),
        13 => gen.xpl12() + gen.xpl10(),
        _other => unreachable!(),
      }
      + if karma { gen.xpl6() } else { 0 }) as i32
  }
}

pub fn earthdawn(args: &str) -> String {
  let gen: &mut GlobalGen = &mut global_gen();
  let mut output = String::new();

  for step_value in args.split_whitespace().take(10).filter_map(basic_sum_str) {
    let step_roll = step(gen, step_value, false);
    writeln!(output, "Rolled step {}: {}", step_value, step_roll).unwrap();
  }
  output.pop(); // delete the trailing newline

  if output.is_empty() {
    String::from("No dice expressions given.")
  } else {
    output
  }
}

pub fn earthdawn_karma(args: &str) -> String {
  let gen: &mut GlobalGen = &mut global_gen();
  let mut output = String::new();

  for step_value in args.split_whitespace().take(10).filter_map(basic_sum_str) {
    let step_roll = step(gen, step_value, true);
    writeln!(output, "Rolled step {} with karma: {}", step_value, step_roll)
      .unwrap();
  }
  output.pop(); // delete the trailing newline

  if output.is_empty() {
    String::from("No dice expressions given.")
  } else {
    output
  }
}

pub fn earthdawn_target(args: &str) -> String {
  let gen: &mut GlobalGen = &mut global_gen();

  let inputs: Vec<i32> =
    args.split_whitespace().filter_map(basic_sum_str).collect();
  match &inputs as &[i32] {
    [step_value, target] => {
      let step_roll = step(gen, *step_value, false);
      let successes =
        if step_roll > *target { 1 + (step_roll - target) / 5 } else { 0 };
      let es_for_successes = if successes != 1 { "es" } else { "" };
      format!(
        "Rolled step {} vs {}: got {} ({} success{})",
        step_value, target, step_roll, successes, es_for_successes
      )
    }
    _ => String::from("usage: STEP TARGET"),
  }
}
