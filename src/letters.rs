use super::*;

const THE_LETTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

const LETTER_RANGE: RandRangeU32 =
  RandRangeU32::new(0, (THE_LETTERS.len() - 1) as _);

/// Rolls DragonTown initiative
///
/// * Args: list of +bonus/name entries
/// * The PC entries are implied
/// * Rolls an init roll for each and then sorts the lines and prints
pub fn letters(args: &str) -> String {
  let gen: &mut PCG32 = &mut global_gen();
  let mut output = String::new();
  //
  'exprloop: for exp in args.split_whitespace().take(20) {
    if let Ok(count) = exp.parse::<u8>() {
      write!(output, "{} letters:", count).ok();
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
