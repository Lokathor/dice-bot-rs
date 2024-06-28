use randomize::BoundedRandU32;

use crate::{
  basic_sum_str,
  global_gen::{global_gen, GlobalGen},
};
use core::fmt::Write;

pub fn warhammer(args: &str) -> String {
  let gen: &mut GlobalGen = &mut global_gen();
  let mut output = String::new();
  let terms: Vec<i32> =
    args.split_whitespace().filter_map(basic_sum_str).collect();
  let range = BoundedRandU32::_6;
  for term in &terms {
    if *term < 1 {
      writeln!(output, "=Rolling {term}: no dice rolled").ok();
    } else {
      let mut one = 0;
      let mut two = 0;
      let mut three = 0;
      let mut four = 0;
      let mut five = 0;
      let mut six = 0;
      for _ in 0..*term {
        match 1 + range.sample(|| gen.next_u32()) as i32 {
          1 => one += 1,
          2 => two += 1,
          3 => three += 1,
          4 => four += 1,
          5 => five += 1,
          6 => six += 1,
          _ => drop(writeln!(output, "ERROR please tell Lokathor")),
        }
      }
      writeln!(
        output,
        "=Rolling {term}:\
      1) {one}\
      2) {two}\
      3) {three}\
      4) {four}\
      5) {five}\
      6) {six}"
      )
      .ok();
    }
  }
  output
}
