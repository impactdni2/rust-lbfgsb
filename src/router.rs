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
    2 => lbfgsb_2::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    3 => lbfgsb_3::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    4 => lbfgsb_4::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    5 => lbfgsb_5::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    6 => lbfgsb_6::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    7 => lbfgsb_7::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    8 => lbfgsb_8::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    9 => lbfgsb_9::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    10 => lbfgsb_10::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    11 => lbfgsb_11::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    12 => lbfgsb_12::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    13 => lbfgsb_13::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    14 => lbfgsb_14::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    15 => lbfgsb_15::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    16 => lbfgsb_16::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    17 => lbfgsb_17::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    18 => lbfgsb_18::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    19 => lbfgsb_19::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    20 => lbfgsb_20::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    21 => lbfgsb_21::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    22 => lbfgsb_22::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    23 => lbfgsb_23::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    24 => lbfgsb_24::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    25 => lbfgsb_25::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    26 => lbfgsb_26::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    27 => lbfgsb_27::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    28 => lbfgsb_28::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    29 => lbfgsb_29::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    30 => lbfgsb_30::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    31 => lbfgsb_31::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    32 => lbfgsb_32::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    33 => lbfgsb_33::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    34 => lbfgsb_34::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    35 => lbfgsb_35::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    36 => lbfgsb_36::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    37 => lbfgsb_37::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    38 => lbfgsb_38::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    39 => lbfgsb_39::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    40 => lbfgsb_40::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    41 => lbfgsb_41::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    42 => lbfgsb_42::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    43 => lbfgsb_43::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    44 => lbfgsb_44::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    45 => lbfgsb_45::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    46 => lbfgsb_46::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    47 => lbfgsb_47::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    48 => lbfgsb_48::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    49 => lbfgsb_49::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    50 => lbfgsb_50::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    51 => lbfgsb_51::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    52 => lbfgsb_52::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    53 => lbfgsb_53::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    54 => lbfgsb_54::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    55 => lbfgsb_55::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    56 => lbfgsb_56::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    57 => lbfgsb_57::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    58 => lbfgsb_58::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    59 => lbfgsb_59::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    60 => lbfgsb_60::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    61 => lbfgsb_61::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    62 => lbfgsb_62::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    63 => lbfgsb_63::lbfgsb(x, bounds, eval_fn, m, factr, pgtol, iprint),
    _ => unreachable!(),
  };

  // Unlock that library
  libs_in_use.lock().unwrap().in_use[lib_id] = false;

  result
}
