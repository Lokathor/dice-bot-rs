use super::*;

const THE_LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

const LETTER_RANGE: RandRangeU32 =
  RandRangeU32::new(0, (THE_LETTERS.len() - 1) as _);

/// Rolls some random English letters.
pub fn letters(args: &str) -> String {
  let gen: &mut PCG32 = &mut global_gen();
  let mut output = String::new();
  //
  'exprloop: for exp in args.split_whitespace().take(20) {
    if let Ok(count) = exp.parse::<u8>() {
      write!(output, "{} letters: ", count).ok();
      for _ in 0..count {
        output.push(THE_LETTERS[LETTER_RANGE.sample(gen) as usize] as char);
      }
      output.push('\n');
    }
  }
  output.pop();
  if output.is_empty() {
    String::from("No dice expressions given.")
  } else {
    output
  }
}
