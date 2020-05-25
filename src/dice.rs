use super::*;

use core::cmp::Ordering;

/// Rolls `XdY` style dice
///
/// * Input: whitespace-trimmed arg text after `,dice` or whatever command you
///   use to activate this.
/// * Output: The entire response message to show.
pub fn dice(args: &str) -> String {
  let gen: &mut PCG32 = &mut global_gen();
  let mut output = String::new();
  let mut lines = 0;
  'exprloop: for dice_expression_str in args.split_whitespace() {
    if lines >= 20 {
      writeln!(output, "`Additional input skipped`", parsed_string, total)
        .unwrap();
      break;
    }
    let plus_only_form = dice_expression_str.replace("-", "+-");
    let mut total: i32 = 0;
    let mut sub_expressions = vec![];
    for sub_expression in plus_only_form.split('+').take(70) {
      if sub_expression.is_empty() {
        continue;
      }
      let mut d_iter = sub_expression.split('d');
      let num_dice: i32 = match d_iter.next() {
        Some(num_dice_str) => {
          if num_dice_str.is_empty() {
            1
          } else {
            match num_dice_str.parse::<i32>() {
              Ok(num) => num.max(-5_000).min(5_000),
              Err(_) => {
                continue 'exprloop;
              }
            }
          }
        }
        None => {
          continue 'exprloop;
        }
      };
      let num_sides: u32 = match d_iter.next() {
        Some(num_sides_str) => match num_sides_str.parse::<u32>() {
          Ok(num) => num.min(4_000_000),
          Err(_) => {
            continue 'exprloop;
          }
        },
        None => 1,
      };
      if d_iter.next().is_some() {
        continue 'exprloop;
      }
      if num_sides == 0 {
        // do nothing with 0-sided dice
      } else if num_sides == 1 {
        total += num_dice;
        sub_expressions.push(format!("{}", num_dice));
      } else {
        let range = match num_sides {
          4 => d4,
          6 => d6,
          8 => d8,
          10 => d10,
          12 => d12,
          20 => d20,
          _ => RandRangeU32::new(1, num_sides),
        };
        match num_dice.cmp(&0) {
          Ordering::Greater => {
            for _ in 0..num_dice {
              total += range.sample(gen) as i32;
            }
            sub_expressions.push(format!("{}d{}", num_dice, num_sides));
          }
          Ordering::Less => {
            for _ in 0..num_dice.abs() {
              total -= range.sample(gen) as i32;
            }
            sub_expressions.push(format!("{}d{}", num_dice, num_sides));
          }
          _ => (),
        }
      }
    }
    if sub_expressions.is_empty().not() {
      let mut parsed_string = sub_expressions[0].clone();
      for sub_expression in sub_expressions.into_iter().skip(1) {
        if sub_expression.starts_with('-') {
          parsed_string.push_str(&sub_expression);
        } else {
          parsed_string.push('+');
          parsed_string.push_str(&sub_expression);
        }
      }
      writeln!(output, "Rolled {}: {}", parsed_string, total).unwrap();
      lines += 1;
    } else {
      // pass
    }
  }
  output.pop();
  if output.is_empty() {
    String::from("No dice expressions given.")
  } else {
    output
  }
}
