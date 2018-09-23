use super::*;

const SR_POOL_MAX_REPORT: u32 = 30;

pub fn sr4(pool_size: u32, six_again: bool) -> String {
  if pool_size == 0 {
    return format!("No dice to roll!");
  } else {
    let gen: &mut PCG32 = &mut get_global_generator();
    let mut dice_record = String::with_capacity(SR_POOL_MAX_REPORT as usize * 2 + 20);
    dice_record.push(' ');
    dice_record.push('`');
    dice_record.push('(');
    //
    let mut dice_rolled = 0;
    let mut hits = 0;
    let mut ones = 0;
    let mut this_is_a_normal_roll = true;
    while dice_rolled < pool_size {
      let roll = d6.sample_with(gen);
      if roll == 1 && this_is_a_normal_roll {
        ones += 1;
      } else if roll >= 5 {
        hits += 1;
      }
      if pool_size < SR_POOL_MAX_REPORT {
        dice_record.push(('0' as u8 + roll as u8) as char);
        dice_record.push(',');
      }
      if roll == 6 && six_again {
        // setup the next pass to be a bonus die
        this_is_a_normal_roll = false;
      } else {
        dice_rolled += 1;
        this_is_a_normal_roll = true;
      }
    }
    let is_glitch = ones >= (pool_size + 1) / 2;
    let glitch_string = match (hits, is_glitch) {
      (0, true) => "CRITICAL GLITCH, ",
      (_, true) => "GLITCH, ",
      _ => "",
    };
    let s_for_hits = if hits != 1 { "s" } else { "" };
    //
    dice_record.pop();
    dice_record.push(')');
    dice_record.push('`');
    //
    let dice_report_output = if pool_size < SR_POOL_MAX_REPORT { &dice_record } else { "" };
    format!(
      "Rolled {} dice: {}{} hit{}{}",
      pool_size, glitch_string, hits, s_for_hits, dice_report_output
    )
  }
}

command!(shadowrun(_ctx, msg, args) {
  let mut output = String::new();
  for arg in args.full().split_whitespace().take(10).map(basic_sum_str) {
    match arg {
      Some(dice_count) => {
        let dice_count = dice_count.max(0).min(5_000) as u32;
        output.push_str(&sr4(dice_count, false));
        output.push('\n');
      },
      None => {
        //msg.react(ReactionType::Unicode(EMOJI_QUESTION.to_string())).ok();
      }
    }
  }
  output.pop();
  if output.len() > 0 {
    if let Err(why) = msg.channel_id.say(output) {
      println!("Error sending message: {:?}", why);
    }
  }
});

command!(shadowrun_edge(_ctx, msg, args) {
  let mut output = String::new();
  for arg in args.full().split_whitespace().take(10).map(basic_sum_str) {
    match arg {
      Some(dice_count) => {
        let dice_count = dice_count.max(0).min(5_000) as u32;
        output.push_str(&sr4(dice_count, true));
        output.push('\n');
      },
      None => {
        //msg.react(ReactionType::Unicode(EMOJI_QUESTION.to_string())).ok();
      }
    }
  }
  output.pop();
  if output.len() > 0 {
    if let Err(why) = msg.channel_id.say(output) {
      println!("Error sending message: {:?}", why);
    }
  }
});
