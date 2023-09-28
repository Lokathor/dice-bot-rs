use randomize::Gen32;

use crate::global_gen::GlobalGen;

use super::*;

pub fn after_sundown(args: &str) -> String {
  let gen: &mut GlobalGen = &mut global_gen();
  let mut output = String::new();
  for dice_count in args.split_whitespace().flat_map(basic_sum_str).take(10) {
    let dice_count = dice_count.max(0).min(5_000) as u32;
    if dice_count > 0 {
      let mut hits = 0;
      const DICE_REPORT_MAXIMUM: u32 = 30;
      let mut dice_record =
        String::with_capacity(DICE_REPORT_MAXIMUM as usize * 2 + 20);
      dice_record.push_str(" `(");
      for _ in 0..dice_count {
        let roll = gen.d6();
        if roll >= 5 {
          hits += 1;
        }
        if dice_count < DICE_REPORT_MAXIMUM {
          dice_record.push((b'0' + roll as u8) as char);
          dice_record.push(',');
        }
      }
      dice_record.pop();
      dice_record.push_str(")`");
      let s_for_hits = if hits != 1 { "s" } else { "" };
      let dice_report_output =
        if dice_count < DICE_REPORT_MAXIMUM { &dice_record } else { "" };
      writeln!(
        output,
        "Rolled {} dice: {} hit{}{}",
        dice_count, hits, s_for_hits, dice_report_output
      )
      .unwrap();
    } else {
      writeln!(output, "No Dice").unwrap();
    }
  }
  output.pop();
  if output.is_empty() {
    String::from("No dice expressions given.")
  } else {
    output
  }
}
