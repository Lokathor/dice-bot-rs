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

macro_rules! format_the_dice_report {
  ($dest:ident, $dice_output:expr) => {
    match &($dice_output).roll_list {
      Some(roll_vec) => {
        if roll_vec.len() > 0 {
          $dest.push_str(" `(");
          for roll in roll_vec {
            $dest.push((b'0' + roll) as char);
            $dest.push(',');
          }
          $dest.pop();
          $dest.push_str(")`");
        } else {
          "".to_string();
        }
      }
      None => {}
    };
  };
}

macro_rules! do_the_dice_pool {
  ($dest:ident, $line_prefix:expr, $pool_size:expr, $has_edge:expr, $to_do:expr) => {{
    let dice_count = $pool_size.max(0).min(5_000) as u32;
    let pool_output = sr4(dice_count, $has_edge);
    let glitch_string_output = glitch_string(pool_output.hits_total, pool_output.is_glitch);
    let hits = pool_output.hits_total;
    $dest.push_str(&format!(
      "{prefix} {pool_size} {reason}: {glitch_string}{hit_count} hit{s_for_hits}",
      prefix = $line_prefix,
      pool_size = dice_count,
      reason = $to_do,
      glitch_string = glitch_string_output,
      hit_count = hits,
      s_for_hits = if hits != 1 { "s" } else { "" },
    ));
    pool_output
  }};
}

command!(shadowrun(_ctx, msg, args) {
  let mut output = String::new();
  for dice_count in args.full().split_whitespace().take(10).filter_map(basic_sum_str) {
    format_the_dice_report!(output, do_the_dice_pool!(output, "Rolled", dice_count, false, "dice"));
    output.push('\n');
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
  for dice_count in args.full().split_whitespace().take(10).filter_map(basic_sum_str) {
    format_the_dice_report!(output, do_the_dice_pool!(output, "Rolled", dice_count, true, "dice with edge (6-again)"));
    output.push('\n');
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
  let terms: Vec<i32> = args.full().split_whitespace().filter_map(basic_sum_str).collect();
  match &terms as &[i32] {
    [conjure, force, soak] => {
      let conjure = *conjure;
      let force = *force;
      let soak = *soak;
      if conjure < 1 {
        output.push_str("No conjure dice!");
      } else if force < 1 {
        output.push_str("There's no Force there!")
      } else {
        let conjure_output = do_the_dice_pool!(output, "You rolled", conjure, false, "dice to conjure");
        {
          format_the_dice_report!(output, conjure_output);
          output.push('\n');
        }
        let conjure_hits = conjure_output.hits_total;
        //
        let force_output = do_the_dice_pool!(output, "Your friend rolled", force, false, "dice to resist");
        {
          let services_owed = (conjure_hits as i32 - force_output.hits_total as i32).max(0);
          let s_for_services_owed = if services_owed != 1 { "s" } else { "" };
          output.push_str(&format!(" ({} service{} owed)", services_owed, s_for_services_owed));
          format_the_dice_report!(output, force_output);
          output.push('\n');
        }
        let force_hits = force_output.hits_total as i32;
        //
        let soak_output = do_the_dice_pool!(output, "Your rolled", soak, false, "dice to soak");
        {
          let net_drain = ((force/2 + force_hits) - soak_output.hits_total as i32).max(0);
          output.push_str(&format!(" ({} net drain)", net_drain));
          format_the_dice_report!(output, soak_output);
        }
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

command!(shadowrun_foe(_ctx, msg, args) {
  let mut output = String::new();
  let terms: Vec<i32> = args.full().split_whitespace().filter_map(basic_sum_str).collect();
  match &terms as &[i32] {
    [bind, force, soak] => {
      let bind = *bind;
      let force = *force;
      let soak = *soak;
      if bind < 1 {
        output.push_str("No binding dice!");
      } else if force < 1 {
        output.push_str("There's no Force there!")
      } else {
        let bind_output = do_the_dice_pool!(output, "You rolled", bind, false, "dice to bind");
        {
          format_the_dice_report!(output, bind_output);
          output.push('\n');
        }
        let bind_hits = bind_output.hits_total;
        //
        let force_dice = force*2;
        let force_output = do_the_dice_pool!(output, "Your victim rolled", force_dice, false, "dice to resist");
        {
          let binding_net_hits = (bind_hits as i32 - force_output.hits_total as i32).max(0);
          if binding_net_hits == 0 {
            output.push_str(" (failed to bind!)\n");
          } else {
            let s_for_binding_net_hits = if binding_net_hits > 1 { "s" } else { "" };
            output.push_str(&format!(" ({} net hit{})", binding_net_hits, s_for_binding_net_hits));
            format_the_dice_report!(output, force_output);
            output.push('\n');
          }
        }
        let force_hits = force_output.hits_total as i32;
        //
        let soak_output = do_the_dice_pool!(output, "Your rolled", soak, false, "dice to soak");
        {
          let net_drain = ((force/2 + force_hits) - soak_output.hits_total as i32).max(0);
          output.push_str(&format!(" ({} net drain)", net_drain));
          format_the_dice_report!(output, soak_output);
        }
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

command!(shadowrun_attack(_ctx, msg, args) {
  let mut output = String::new();
  let terms: Vec<i32> = args.full().split_whitespace().filter_map(basic_sum_str).collect();
  match &terms as &[i32] {
    [attack, evade, damage, soak] => {
      if *attack < 1 {
        output = "No attack dice!".to_owned();
      } else {
        let attack = *attack as u32;
        let evade = (*evade).max(0) as u32;
        let damage = *damage as u32;
        let soak = (*soak).max(0) as u32;
        //
        let attack_output = do_the_dice_pool!(output, "You rolled", attack, false, "dice to attack");
        {
          format_the_dice_report!(output, attack_output);
          output.push('\n');
        }
        let attack_hits = attack_output.hits_total;
        //
        let evade_output = do_the_dice_pool!(output, "They rolled", evade, false, "dice to evade");
        let evade_hits = evade_output.hits_total;
        let attack_net_hits = attack_hits as i32 - evade_hits as i32;
        if attack_net_hits < 0 {
          output.push_str(" (you missed!) ");
          format_the_dice_report!(output, evade_output);
        } else if attack_net_hits == 0 {
          output.push_str(" (grazing hit, no damage) ");
          format_the_dice_report!(output, evade_output);
        } else {
          let s_for_net_hits = if attack_net_hits != 1 { "s" } else { "" };
          let modified_damage = damage as i32 + attack_net_hits;
          output.push_str(&format!(" ({} net hit{}, modified damage is {})",attack_net_hits, s_for_net_hits, modified_damage));
          format_the_dice_report!(output, evade_output);
          output.push('\n');
          //
          let soak_output = do_the_dice_pool!(output, "They rolled", soak, false, "dice to soak");
          {
            let damage_after_soak = (modified_damage - soak_output.hits_total as i32).max(0);
            output.push_str(&format!(" ({} damage after soak)",damage_after_soak));
            format_the_dice_report!(output, evade_output);
            output.push('\n');
          }
        }
      }
    }
    _ => {
      output.push_str("Usage: ATTACK EVADE DAMAGE SOAK");
    }
  }
  if let Err(why) = msg.channel_id.say(output) {
    println!("Error sending message: {:?}", why);
  }
});
