use randomize::PCG32K;
use std::sync::{Mutex, MutexGuard, OnceLock};

pub type GlobalGen = PCG32K<32>;

static M: OnceLock<Mutex<GlobalGen>> = OnceLock::new();

pub fn global_gen() -> MutexGuard<'static, GlobalGen> {
  let m: &'static Mutex<GlobalGen> = M.get_or_init(|| {
    Mutex::new(
      GlobalGen::from_getrandom().unwrap_or(GlobalGen::new(0, [0; 32])),
    )
  });
  match m.lock() {
    Ok(guard) => guard,
    Err(poison) => poison.into_inner(),
  }
}
