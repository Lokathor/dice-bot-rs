use super::*;

#[derive(Debug, Clone, Copy)]
pub enum Symbol {
  Success,
  Failure,
  Advantage,
  Disadvantage,
  Triumph,
  Despair,
  Light,
  Dark,
}
use self::Symbol::*;

// goods
static BLANK: &'static [Symbol] = &[];
static TWO_ADVANTAGE: &'static [Symbol] = &[Advantage, Advantage];
static ONE_ADVANTAGE: &'static [Symbol] = &[Advantage];
static ADVANTAGE_SUCCESS: &'static [Symbol] = &[Advantage, Success];
static ONE_SUCCESS: &'static [Symbol] = &[Success];
static TWO_SUCCESS: &'static [Symbol] = &[Success, Success];
static THE_TRIUMPH: &'static [Symbol] = &[Success, Triumph];
// bads
static ONE_DISADVANTAGE: &'static [Symbol] = &[Disadvantage];
static TWO_DISADVANTAGE: &'static [Symbol] = &[Disadvantage, Disadvantage];
static ONE_FAILURE: &'static [Symbol] = &[Failure];
static TWO_FAILURE: &'static [Symbol] = &[Failure, Failure];
static DISADVANTAGE_FAILURE: &'static [Symbol] = &[Disadvantage, Failure];
static THE_DESPAIR: &'static [Symbol] = &[Failure, Despair];
// force
static ONE_DARK: &'static [Symbol] = &[Dark];
static TWO_DARK: &'static [Symbol] = &[Dark, Dark];
static ONE_LIGHT: &'static [Symbol] = &[Light];
static TWO_LIGHT: &'static [Symbol] = &[Light, Light];

fn blue(gen: &mut PCG32) -> &'static [Symbol] {
  match d6.sample(gen) {
    1 | 2 => BLANK,
    3 => TWO_ADVANTAGE,
    4 => ONE_ADVANTAGE,
    5 => ADVANTAGE_SUCCESS,
    6 => ONE_SUCCESS,
    _ => unreachable!(),
  }
}

fn black(gen: &mut PCG32) -> &'static [Symbol] {
  match d6.sample(gen) {
    1 | 2 => BLANK,
    3 | 4 => ONE_FAILURE,
    5 | 6 => ONE_DISADVANTAGE,
    _ => unreachable!(),
  }
}

fn green(gen: &mut PCG32) -> &'static [Symbol] {
  match d8.sample(gen) {
    1 => BLANK,
    2 | 3 => ONE_SUCCESS,
    4 => TWO_SUCCESS,
    5 | 6 => ONE_ADVANTAGE,
    7 => ADVANTAGE_SUCCESS,
    8 => TWO_ADVANTAGE,
    _ => unreachable!(),
  }
}

fn purple(gen: &mut PCG32) -> &'static [Symbol] {
  match d8.sample(gen) {
    1 => BLANK,
    2 => ONE_FAILURE,
    3 => TWO_FAILURE,
    4 | 5 | 6 => ONE_DISADVANTAGE,
    7 => TWO_DISADVANTAGE,
    8 => DISADVANTAGE_FAILURE,
    _ => unreachable!(),
  }
}

fn yellow(gen: &mut PCG32) -> &'static [Symbol] {
  match d12.sample(gen) {
    1 => BLANK,
    2 | 3 => ONE_SUCCESS,
    4 | 5 => TWO_SUCCESS,
    6 => ONE_ADVANTAGE,
    7 | 8 | 9 => ADVANTAGE_SUCCESS,
    10 | 11 => TWO_ADVANTAGE,
    12 => THE_TRIUMPH,
    _ => unreachable!(),
  }
}

fn red(gen: &mut PCG32) -> &'static [Symbol] {
  match d12.sample(gen) {
    1 => BLANK,
    2 | 3 => ONE_FAILURE,
    4 | 5 => TWO_FAILURE,
    6 | 7 => ONE_DISADVANTAGE,
    8 | 9 => DISADVANTAGE_FAILURE,
    10 | 11 => TWO_DISADVANTAGE,
    12 => THE_DESPAIR,
    _ => unreachable!(),
  }
}

fn white(gen: &mut PCG32) -> &'static [Symbol] {
  match d12.sample(gen) {
    1 | 2 | 3 | 4 | 5 | 6 => ONE_DARK,
    7 => TWO_DARK,
    8 | 9 => ONE_LIGHT,
    10 | 11 | 12 => TWO_LIGHT,
    _ => unreachable!(),
  }
}

command!(eote(_ctx, msg, args) {
  let gen: &mut PCG32 = &mut global_gen();
  let mut output = String::new();
  let terms: Vec<&str> = args.full().split_whitespace().collect();
  'termloop: for term in terms {
    let mut pool_string = String::new();
    for ch in term.chars() {
      match ch {
        'u' | 'U' => pool_string.push('U'),
        'b' | 'B' => pool_string.push('B'),
        'g' | 'G' => pool_string.push('G'),
        'p' | 'P' => pool_string.push('P'),
        'y' | 'Y' => pool_string.push('Y'),
        'r' | 'R' => pool_string.push('R'),
        'w' | 'W' => pool_string.push('W'),
        _ => continue 'termloop,
      }
    }
    let mut successes = 0i32;
    let mut advantages = 0i32;
    let mut triumphs = 0i32;
    let mut despairs = 0i32;
    let mut lights = 0i32;
    let mut darks = 0i32;
    for pool_die in pool_string.chars() {
      let roll_result = match pool_die {
        'U' => blue(gen),
        'B' => black(gen),
        'G' => green(gen),
        'P' => purple(gen),
        'Y' => yellow(gen),
        'R' => red(gen),
        'W' => white(gen),
        _ => unreachable!(),
      };
      for symbol in roll_result {
        match symbol {
          Success => successes += 1,
          Failure => successes -= 1,
          Advantage => advantages += 1,
          Disadvantage => advantages -= 1,
          Triumph => triumphs += 1,
          Despair => despairs += 1,
          Light => lights += 1,
          Dark => darks += 1,
        }
      };
    }
    let mut symbol_total_string = String::new();
    if successes > 0 {
      symbol_total_string.push_str(&format!("{} Success",successes));
      if successes > 1 {
        symbol_total_string.push_str("es, ");
      } else {
        symbol_total_string.push_str(", ");
      }
    } else if successes < 0 {
      symbol_total_string.push_str(&format!("{} Failure",successes.abs()));
      if successes < -1 {
        symbol_total_string.push_str("s, ");
      } else {
        symbol_total_string.push_str(", ");
      }
    } else {
      symbol_total_string.push_str("0 Failures, ");
    }
    if advantages > 0 {
      symbol_total_string.push_str(&format!("{} Advantage",advantages));
      if advantages > 1 {
        symbol_total_string.push_str("s, ");
      } else {
        symbol_total_string.push_str(", ");
      }
    } else if advantages < 0 {
      symbol_total_string.push_str(&format!("{} Disadvantage",advantages.abs()));
      if advantages < -1 {
        symbol_total_string.push_str("s, ");
      } else {
        symbol_total_string.push_str(", ");
      }
    }
    for (quantity, symbol) in &[(triumphs, Triumph), (despairs, Despair), (lights, Light), (darks, Dark)] {
      if *quantity > 0 {
        symbol_total_string.push_str(&format!("{} {:?}",quantity, symbol));
        if *quantity > 1 {
          symbol_total_string.push_str("s, ");
        } else {
          symbol_total_string.push_str(", ");
        }
      }
    }
    for _ in 0 .. 2 {
      symbol_total_string.pop();
    }
    output.push_str(&format!("Rolled {}: {}\n", pool_string, symbol_total_string));
  }
  output.pop();
  if output.len() > 0 {
    if let Err(why) = msg.channel_id.say(output) {
      println!("Error sending message: {:?}", why);
    }
  } else {
    if let Err(why) = msg.channel_id.say("usage: eote POOL (black = b, blue = u)") {
      println!("Error sending message: {:?}", why);
    }
  }
});
