use super::*;

const SR_POOL_MAX_REPORT: u32 = 30;

pub struct PoolRollOutput {
  pub hits_total: u32,
  pub is_glitch: bool,
  pub roll_list: Option<Vec<u8>>,
}

pub fn glitch_string(hits: u32, is_glitch: bool) -> &'static str {
  match (hits, is_glitch) {
    (0, true) => "CRITICAL GLITCH, ",
    (_, true) => "GLITCH, ",
    _ => "",
  }
}

pub fn sr4(pool_size: u32, six_again: bool) -> PoolRollOutput {
  if pool_size == 0 {
    return PoolRollOutput {
      hits_total: 0,
      is_glitch: false,
      roll_list: None,
    };
  } else {
    let gen: &mut PCG32 = &mut get_global_generator();
    let mut dice_record: Vec<u8> = vec![];
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
        dice_record.push(roll as u8);
      }
      if roll == 6 && six_again {
        // setup the next pass to be a bonus die
        this_is_a_normal_roll = false;
      } else {
        dice_rolled += 1;
        this_is_a_normal_roll = true;
      }
    }
    PoolRollOutput {
      hits_total: hits,
      is_glitch: ones >= (pool_size + 1) / 2,
      roll_list: if dice_record.len() > 0 { Some(dice_record) } else { None },
    }
  }
}

command!(shadowrun(_ctx, msg, args) {
  let mut output = String::new();
  for arg in args.full().split_whitespace().take(10).map(basic_sum_str) {
    match arg {
      Some(dice_count) => {
        let dice_count = dice_count.max(0).min(5_000) as u32;
        let pool_output = sr4(dice_count, false);
        let glitch_string_output = glitch_string(pool_output.hits_total, pool_output.is_glitch);
        let hits = pool_output.hits_total;
        let s_for_hits = if hits != 1 { "s" } else { "" };
        let dice_report_output = match pool_output.roll_list {
          Some(roll_vec) => {
            if roll_vec.len() > 0 {
              let mut report = String::with_capacity(roll_vec.len() * 2 + 2);
              report.push_str(" `(");
              for roll in roll_vec {
                report.push((b'0' + roll) as char);
                report.push(',');
              }
              report.pop();
              report.push_str(")`");
              report
            } else {
              "".to_string()
            }
          },
          None => "".to_string(),
        };
        output.push_str(&format!(
          "Rolled {} dice: {}{} hit{}{}",
          dice_count, glitch_string_output, hits, s_for_hits, dice_report_output
        ));
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
        let pool_output = sr4(dice_count, true);
        let glitch_string_output = glitch_string(pool_output.hits_total, pool_output.is_glitch);
        let hits = pool_output.hits_total;
        let s_for_hits = if hits != 1 { "s" } else { "" };
        let dice_report_output = match pool_output.roll_list {
          Some(roll_vec) => {
            if roll_vec.len() > 0 {
              let mut report = String::with_capacity(roll_vec.len() * 2 + 2);
              report.push_str(" `(");
              for roll in roll_vec {
                report.push((b'0' + roll) as char);
                report.push(',');
              }
              report.pop();
              report.push_str(")`");
              report
            } else {
              "".to_string()
            }
          },
          None => "".to_string(),
        };
        output.push_str(&format!(
          "Rolled {} dice with edge (6-again): {}{} hit{}{}",
          dice_count, glitch_string_output, hits, s_for_hits, dice_report_output
        ));
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

command!(shadowrun_friend(_ctx, msg, args) {
  let mut output = String::new();
  let terms: Vec<i32> = args.full().split_whitespace().take(3).filter_map(basic_sum_str).collect();
  let terms_ref: &[i32] = &terms;
  match terms_ref {
    [conjure, force, soak] => {
      if *conjure < 1 {
        output.push_str("No conjure dice!");
      } else if *force < 1 {
        output.push_str("There's no Force there!")
      } else {
        let dice_count = (*conjure).max(0).min(5_000) as u32;
        let pool_output = sr4(dice_count, false);
        let glitch_string_output = glitch_string(pool_output.hits_total, pool_output.is_glitch);
        let hits = pool_output.hits_total;
        let s_for_hits = if hits != 1 { "s" } else { "" };
        let dice_report_output = match pool_output.roll_list {
          Some(roll_vec) => {
            if roll_vec.len() > 0 {
              let mut report = String::with_capacity(roll_vec.len() * 2 + 2);
              report.push_str(" `(");
              for roll in roll_vec {
                report.push((b'0' + roll) as char);
                report.push(',');
              }
              report.pop();
              report.push_str(")`");
              report
            } else {
              "".to_string()
            }
          },
          None => "".to_string(),
        };
        output.push_str(&format!(
          "You rolled {} dice to conjure: {}{} hit{}{}",
          dice_count, glitch_string_output, hits, s_for_hits, dice_report_output
        ));
        output.push('\n');
        let conjure_hits = pool_output.hits_total;
        //
        let force = (*force).max(0).min(5_000) as u32;
        let dice_count = force;
        let pool_output = sr4(dice_count, false);
        let glitch_string_output = glitch_string(pool_output.hits_total, pool_output.is_glitch);
        let hits = pool_output.hits_total;
        let s_for_hits = if hits != 1 { "s" } else { "" };
        let dice_report_output = match pool_output.roll_list {
          Some(roll_vec) => {
            if roll_vec.len() > 0 {
              let mut report = String::with_capacity(roll_vec.len() * 2 + 2);
              report.push_str(" `(");
              for roll in roll_vec {
                report.push((b'0' + roll) as char);
                report.push(',');
              }
              report.pop();
              report.push_str(")`");
              report
            } else {
              "".to_string()
            }
          },
          None => "".to_string(),
        };
        let services_owed = (conjure_hits as i32 - pool_output.hits_total as i32).max(0);
        output.push_str(&format!(
          "Your friend rolled {} dice to resist: {}{} hit{} ({} services owed){}",
          dice_count, glitch_string_output, hits, s_for_hits, services_owed, dice_report_output
        ));
        output.push('\n');
        let force_hits = pool_output.hits_total;
        //
        let dice_count = (*soak).max(0).min(5_000) as u32;
        let pool_output = sr4(dice_count, false);
        let glitch_string_output = glitch_string(pool_output.hits_total, pool_output.is_glitch);
        let hits = pool_output.hits_total;
        let s_for_hits = if hits != 1 { "s" } else { "" };
        let dice_report_output = match pool_output.roll_list {
          Some(roll_vec) => {
            if roll_vec.len() > 0 {
              let mut report = String::with_capacity(roll_vec.len() * 2 + 2);
              report.push_str(" `(");
              for roll in roll_vec {
                report.push((b'0' + roll) as char);
                report.push(',');
              }
              report.pop();
              report.push_str(")`");
              report
            } else {
              "".to_string()
            }
          },
          None => "".to_string(),
        };
        let net_drain = ((force/2 + force_hits) - pool_output.hits_total).max(0);
        output.push_str(&format!(
          "You rolled {} dice to soak drain: {}{} hit{} ({} net drain){}",
          dice_count, glitch_string_output, hits, s_for_hits, net_drain, dice_report_output
        ));
      }
    }
    _ => {
      output.push_str("Usage: CONJURE FORCE SOAK");
    }
  }
  if let Err(why) = msg.channel_id.say(output) {
    println!("Error sending message: {:?}", why);
  }
});
