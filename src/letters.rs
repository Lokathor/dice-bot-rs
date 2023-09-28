use randomize::BoundedRandU16;

use crate::global_gen::GlobalGen;

use super::*;

const THE_LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

const LETTER_RANGE: BoundedRandU16 =
  BoundedRandU16::new((THE_LETTERS.len() - 1) as _);

/// Rolls some random English letters.
pub fn letters(args: &str) -> String {
  let gen: &mut GlobalGen = &mut global_gen();
  let mut output = String::new();
  //
  'exprloop: for exp in args.split_whitespace().take(20) {
    if let Ok(count) = exp.parse::<u8>() {
      write!(output, "{} letters: ", count).ok();
      for _ in 0..count {
        output.push(
          THE_LETTERS[LETTER_RANGE.sample(|| gen.next_u32() as u16) as usize]
            as char,
        );
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
