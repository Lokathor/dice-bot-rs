use super::*;

/// Rolls 4d4+4 dix times.
///
/// * Output: The entire response message to show.
pub fn stat2e() -> String {
  let gen: &mut PCG32 = &mut global_gen();
  let mut output = String::new();
  let roll = |gen: &mut PCG32| {
    4 + d4.sample(gen) + d4.sample(gen) + d4.sample(gen) + d4.sample(gen)
  };
  writeln!(output, "Str: {}", roll(gen)).unwrap();
  writeln!(output, "Dex: {}", roll(gen)).unwrap();
  writeln!(output, "Con: {}", roll(gen)).unwrap();
  writeln!(output, "Int: {}", roll(gen)).unwrap();
  writeln!(output, "Wis: {}", roll(gen)).unwrap();
  writeln!(output, "Cha: {}", roll(gen)).unwrap();
  output.pop();
  output
}
