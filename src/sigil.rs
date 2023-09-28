use randomize::Gen32;

use crate::global_gen::GlobalGen;

use super::*;

pub fn sigil(args: &str) -> String {
  let gen: &mut GlobalGen = &mut global_gen();
  let mut output = String::new();
  let terms: Vec<i32> =
    args.split_whitespace().filter_map(basic_sum_str).collect();
  for term in terms {
    let x = term.abs();
    if x > 0 {
      let mut total = 0_i32;
      for _ in 0..x {
        total += gen.d6() as i32;
        total -= gen.d6() as i32;
      }
      writeln!(output, "Rolling Sigil {}: {}\n", x, total.abs()).unwrap();
    } else {
      writeln!(output, "Rolling Sigil {}: 0\n", x).unwrap();
    }
  }
  output.pop();
  if output.is_empty() {
    String::from("No dice expressions given.")
  } else {
    output
  }
}
