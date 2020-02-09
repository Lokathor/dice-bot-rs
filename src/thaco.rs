use super::*;

pub fn thaco(args: &str) -> String {
  let gen: &mut PCG32 = &mut global_gen();
  let mut output = String::new();
  for thaco_value in args.split_whitespace().flat_map(basic_sum_str).take(20) {
    let roll = d20.sample(gen) as i32;
    output.push_str(&format!(
      "THACO {}: Rolled {}, Hits AC {} or greater.\n",
      thaco_value,
      roll,
      thaco_value - roll
    ));
  }
  output.pop();
  if output.is_empty() {
    String::from("No dice expressions given.")
  } else {
    output
  }
}
