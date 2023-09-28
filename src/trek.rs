use randomize::BoundedRandU32;

use crate::global_gen::GlobalGen;

use super::*;

#[rustfmt::skip]
const TNG_CHARACTERS: &[&str] =
  &[
  "Jean-Luc Picard", "William T. Riker", "Deanna Troi", "Geordi La Forge", "Worf", "Beverly Crusher", "Data",
  "Jean-Luc Picard", "William T. Riker", "Deanna Troi", "Geordi La Forge", "Worf", "Beverly Crusher", "Data",
  "Jean-Luc Picard", "William T. Riker", "Deanna Troi", "Geordi La Forge", "Worf", "Beverly Crusher", "Data",
  "Jean-Luc Picard", "William T. Riker", "Deanna Troi", "Geordi La Forge", "Worf", "Beverly Crusher", "Data",
  //
  "Jean-Luc Picard", "William T. Riker", "Deanna Troi", "Geordi La Forge", "Worf", "Beverly Crusher", "Data",
  "Jean-Luc Picard", "William T. Riker", "Deanna Troi", "Geordi La Forge", "Worf", "Beverly Crusher", "Data",
  "Jean-Luc Picard", "William T. Riker", "Deanna Troi", "Geordi La Forge", "Worf", "Beverly Crusher", "Data",
  "Jean-Luc Picard", "William T. Riker", "Deanna Troi", "Geordi La Forge", "Worf", "Beverly Crusher", "Data",
  //
  "Random Red-shirt",
  "Random Gold-shirt", "Random Gold-shirt",
  "Random Blue-shirt",
  //
  "Chancellor Gowron", "Alexander Rozhenko",
  "Lore",
  "Lwaxana Troi",
  "Wesley Crusher",
  "Alynna Nechayev",
  ];

#[rustfmt::skip]
const DS9_CHARACTERS: &[&str] =
  &[
  "Benjamin Sisko", "Julian Bashir", "Jadzia Dax", "Kira Nerys", "Miles Edward O'Brien", "Odo", "Quark",
  "Benjamin Sisko", "Julian Bashir", "Jadzia Dax", "Kira Nerys", "Miles Edward O'Brien", "Odo", "Quark",
  "Benjamin Sisko", "Julian Bashir", "Jadzia Dax", "Kira Nerys", "Miles Edward O'Brien", "Odo", "Quark",
  "Benjamin Sisko", "Julian Bashir", "Jadzia Dax", "Kira Nerys", "Miles Edward O'Brien", "Odo", "Quark",
  //
  "Benjamin Sisko", "Julian Bashir", "Jadzia Dax", "Kira Nerys", "Miles Edward O'Brien", "Odo", "Quark",
  "Benjamin Sisko", "Julian Bashir", "Jadzia Dax", "Kira Nerys", "Miles Edward O'Brien", "Odo", "Quark",
  "Benjamin Sisko", "Julian Bashir", "Jadzia Dax", "Kira Nerys", "Miles Edward O'Brien", "Odo", "Quark",
  "Benjamin Sisko", "Julian Bashir", "Jadzia Dax", "Kira Nerys", "Miles Edward O'Brien", "Odo", "Quark",
  //
  "Garak", "Garak", "Garak", "Garak",
  //
  "Jake Sisko", "Nog",
  "Keiko O'Brien",
  ];

pub fn trek(args: &str) -> String {
  let gen: &mut GlobalGen = &mut global_gen();
  let mut output = String::new();
  //
  'exprloop: for exp in args.split_whitespace().take(20) {
    match exp {
      "tng" => {
        let out = {
          let b = BoundedRandU32::new(TNG_CHARACTERS.len() as u32);
          b.sample(|| gen.next_u32()) as usize
        };
        output.push_str(TNG_CHARACTERS[out]);
        output.push('\n');
      }
      "ds9" => {
        let out = {
          let b = BoundedRandU32::new(DS9_CHARACTERS.len() as u32);
          b.sample(|| gen.next_u32()) as usize
        };
        output.push_str(DS9_CHARACTERS[out]);
        output.push('\n');
      }
      _ => output.push_str("?/n"),
    }
  }
  output.pop();
  if output.is_empty() {
    String::from("No dice expressions given.")
  } else {
    output
  }
}
