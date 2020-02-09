use super::*;

/// Rolls a step roll, according to the 4th edition chart.
pub fn step(gen: &mut PCG32, mut step: i32, karma: bool) -> i32 {
  if step < 1 {
    0
  } else {
    let mut total = 0;
    while step > 13 {
      total += d12.explode(gen);
      step -= 7;
    }
    (total
      + match step {
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
      }
      + if karma { d6.explode(gen) } else { 0 }) as i32
  }
}

pub fn earthdawn(args: &str) -> String {
  let gen: &mut PCG32 = &mut global_gen();
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
  let gen: &mut PCG32 = &mut global_gen();
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
  let gen: &mut PCG32 = &mut global_gen();

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
