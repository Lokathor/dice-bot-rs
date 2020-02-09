use super::*;

use core::{
  ptr::null_mut,
  sync::atomic::{AtomicPtr, Ordering},
};
use std::sync::{Mutex, MutexGuard};

static M: AtomicPtr<Mutex<PCG32>> = AtomicPtr::new(null_mut());

pub fn global_gen() -> MutexGuard<'static, PCG32> {
  let p: *mut Mutex<PCG32> = M.load(Ordering::SeqCst);
  if p.is_null() {
    let mut arr = [0_u64, 0];
    getrandom::getrandom(bytemuck::bytes_of_mut(&mut arr)).unwrap();
    let [state, inc] = arr;
    let b = Box::new(Mutex::new(PCG32::seed(state, inc)));
    let new_p: *mut Mutex<PCG32> = Box::leak(b);
    if M.compare_and_swap(null_mut(), new_p, Ordering::SeqCst).is_null() {
      // success: leave the mutex in a leaked state
    } else {
      // failed, re-box that mutex.
      unsafe { Box::from_raw(new_p) };
    }
    return global_gen();
  }
  match unsafe { (*p).lock() } {
    Ok(guard) => guard,
    Err(poison) => poison.into_inner(),
  }
}
