use randomize::Gen32;

use crate::global_gen::GlobalGen;

use super::*;

/// Rolls 4d4+4 six times.
///
/// * Output: The entire response message to show.
pub fn stat2e() -> String {
  let gen: &mut GlobalGen = &mut global_gen();
  let mut output = String::new();
  let roll =
    |gen: &mut GlobalGen| 4 + gen.d4() + gen.d4() + gen.d4() + gen.d4();
  writeln!(output, "Str: {}", roll(gen)).unwrap();
  writeln!(output, "Dex: {}", roll(gen)).unwrap();
  writeln!(output, "Con: {}", roll(gen)).unwrap();
  writeln!(output, "Int: {}", roll(gen)).unwrap();
  writeln!(output, "Wis: {}", roll(gen)).unwrap();
  writeln!(output, "Cha: {}", roll(gen)).unwrap();
  output.pop();
  output
}
