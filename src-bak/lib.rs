#[macro_use]
extern crate serenity;

extern crate randomize;
use randomize::*;

pub mod earthdawn;
pub mod eote;
pub mod shadowrun;

trait ExplodingRange {
  fn explode(&self, &mut PCG32) -> u32;
}

impl ExplodingRange for RandRangeU32 {
  fn explode(&self, gen: &mut PCG32) -> u32 {
    let mut times = 0;
    loop {
      let roll = self.sample(gen);
      if roll == self.high() {
        times += 1;
        continue;
      } else {
        return self.high() * times + roll;
      }
    }
  }
}
