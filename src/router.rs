use std::sync::Mutex;

use anyhow::Error;


include!(concat!(env!("OUT_DIR"), "/lib.rs"));

const MAX_INSTANCES: usize = 64;

static libs_in_use: Mutex<LibTracker> = Mutex::new(LibTracker {
  last_id: 0,
  in_use: [false; MAX_INSTANCES],
});

struct LibTracker {
  pub last_id: usize,
  pub in_use: [bool; MAX_INSTANCES],
}

pub fn lbfgsb<E>(x: Vec<f64>, bounds: &[(f64, f64)], eval_fn: E, m: usize, factr: f64, pgtol: f64, iprint: i64) -> Result<Vec<f64>, Error>
where E: FnMut(&[f64], &mut [f64]) -> Result<f64, Error> {
  // Find a library that isn't currently in use...
  let mut locked = libs_in_use.lock().unwrap();

  // Loop up to 128 times (so we don't just infinitely loop)..
  for _ in 0..MAX_INSTANCES {
    locked.last_id += 1;
    if locked.last_id >= MAX_INSTANCES {
      locked.last_id = 0;
    }

    if !locked.in_use[locked.last_id] {
      break;
    }
  }

  // Couldn't find a library not in use
  if locked.in_use[locked.last_id] {
    return Err(Error::msg("All libraries are in use!"));
  }

  // Set that library as in-use
  let lib_id = locked.last_id;
  locked.in_use[lib_id] = true;
  drop(locked);

  // Run the lib-specific version
  let result = match lib_id {
    0 => lbfgsb_0::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    1 => lbfgsb_1::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    _ => unreachable!(),
  };

  // Unlock that library
  libs_in_use.lock().unwrap().in_use[lib_id] = false;

  result
}
