use super::*;

pub fn sigil(args: &str) -> String {
  let gen: &mut PCG32 = &mut global_gen();
  let mut output = String::new();
  let terms: Vec<i32> =
    args.split_whitespace().filter_map(basic_sum_str).collect();
  for term in terms {
    let x = term.abs();
    if x > 0 {
      let mut total = 0;
      for _ in 0..x {
        total += d6.sample(gen) as i32;
        total -= d6.sample(gen) as i32;
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
