use std::sync::Mutex;

use anyhow::Error;

use crate::shared::{LbfgsbParameter, LbfgsbProblem};


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

pub fn lbfgsb<'a, E>(problem: &'a mut LbfgsbProblem<E>, param: LbfgsbParameter) -> Result<(), Error>
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
    0 => lbfgsb_0::lbfgsb(problem, param),
    1 => lbfgsb_1::lbfgsb(problem, param),
    2 => lbfgsb_2::lbfgsb(problem, param),
    3 => lbfgsb_3::lbfgsb(problem, param),
    4 => lbfgsb_4::lbfgsb(problem, param),
    5 => lbfgsb_5::lbfgsb(problem, param),
    6 => lbfgsb_6::lbfgsb(problem, param),
    7 => lbfgsb_7::lbfgsb(problem, param),
    8 => lbfgsb_8::lbfgsb(problem, param),
    9 => lbfgsb_9::lbfgsb(problem, param),
    10 => lbfgsb_10::lbfgsb(problem, param),
    11 => lbfgsb_11::lbfgsb(problem, param),
    12 => lbfgsb_12::lbfgsb(problem, param),
    13 => lbfgsb_13::lbfgsb(problem, param),
    14 => lbfgsb_14::lbfgsb(problem, param),
    15 => lbfgsb_15::lbfgsb(problem, param),
    16 => lbfgsb_16::lbfgsb(problem, param),
    17 => lbfgsb_17::lbfgsb(problem, param),
    18 => lbfgsb_18::lbfgsb(problem, param),
    19 => lbfgsb_19::lbfgsb(problem, param),
    20 => lbfgsb_20::lbfgsb(problem, param),
    21 => lbfgsb_21::lbfgsb(problem, param),
    22 => lbfgsb_22::lbfgsb(problem, param),
    23 => lbfgsb_23::lbfgsb(problem, param),
    24 => lbfgsb_24::lbfgsb(problem, param),
    25 => lbfgsb_25::lbfgsb(problem, param),
    26 => lbfgsb_26::lbfgsb(problem, param),
    27 => lbfgsb_27::lbfgsb(problem, param),
    28 => lbfgsb_28::lbfgsb(problem, param),
    29 => lbfgsb_29::lbfgsb(problem, param),
    30 => lbfgsb_30::lbfgsb(problem, param),
    31 => lbfgsb_31::lbfgsb(problem, param),
    32 => lbfgsb_32::lbfgsb(problem, param),
    33 => lbfgsb_33::lbfgsb(problem, param),
    34 => lbfgsb_34::lbfgsb(problem, param),
    35 => lbfgsb_35::lbfgsb(problem, param),
    36 => lbfgsb_36::lbfgsb(problem, param),
    37 => lbfgsb_37::lbfgsb(problem, param),
    38 => lbfgsb_38::lbfgsb(problem, param),
    39 => lbfgsb_39::lbfgsb(problem, param),
    40 => lbfgsb_40::lbfgsb(problem, param),
    41 => lbfgsb_41::lbfgsb(problem, param),
    42 => lbfgsb_42::lbfgsb(problem, param),
    43 => lbfgsb_43::lbfgsb(problem, param),
    44 => lbfgsb_44::lbfgsb(problem, param),
    45 => lbfgsb_45::lbfgsb(problem, param),
    46 => lbfgsb_46::lbfgsb(problem, param),
    47 => lbfgsb_47::lbfgsb(problem, param),
    48 => lbfgsb_48::lbfgsb(problem, param),
    49 => lbfgsb_49::lbfgsb(problem, param),
    50 => lbfgsb_50::lbfgsb(problem, param),
    51 => lbfgsb_51::lbfgsb(problem, param),
    52 => lbfgsb_52::lbfgsb(problem, param),
    53 => lbfgsb_53::lbfgsb(problem, param),
    54 => lbfgsb_54::lbfgsb(problem, param),
    55 => lbfgsb_55::lbfgsb(problem, param),
    56 => lbfgsb_56::lbfgsb(problem, param),
    57 => lbfgsb_57::lbfgsb(problem, param),
    58 => lbfgsb_58::lbfgsb(problem, param),
    59 => lbfgsb_59::lbfgsb(problem, param),
    60 => lbfgsb_60::lbfgsb(problem, param),
    61 => lbfgsb_61::lbfgsb(problem, param),
    62 => lbfgsb_62::lbfgsb(problem, param),
    63 => lbfgsb_63::lbfgsb(problem, param),
    _ => unreachable!(),
  };

  // Unlock that library
  libs_in_use.lock().unwrap().in_use[lib_id] = false;
  
  result
}
