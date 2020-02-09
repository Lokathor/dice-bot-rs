use super::*;

/// For the champions game I guess?
///
/// I don't play it they just told me to put it in.
///
/// * Input: whitespace-trimmed arg text
/// * Output: The entire response message to show.
pub fn champions(args: &str) -> String {
  let gen: &mut PCG32 = &mut global_gen();
  let mut output = String::new();
  let terms: Vec<i32> =
    args.split_whitespace().filter_map(basic_sum_str).collect();
  for term in terms {
    let mut rolls = [0; 3];
    for roll_mut in rolls.iter_mut() {
      *roll_mut = d6.sample(gen) as i32;
    }
    writeln!(
      output,
      "Rolling Champions {}: {}, [{},{},{}]",
      term,
      if rolls.iter().cloned().sum::<i32>() < term {
        "Success"
      } else {
        "Failure"
      },
      rolls[0],
      rolls[1],
      rolls[2]
    )
    .unwrap();
  }
  output.pop();
  if output.is_empty() {
    String::from("No expressions given.")
  } else {
    output
  }
}
