use anyhow::Result;
mod bindings { include!(concat!(env!("OUT_DIR"), "/bindings_0.rs")); }
use bindings::{FG, FG_END};

// [[file:../lbfgsb.note::*util][util:1]]
// #define IS_FG(x) ( ((x)>=FG) ?  ( ((x)<=FG_END) ? 1 : 0 ) : 0 )
pub(crate) fn is_fg(task: i64) -> bool {
  let task = task as u32;
  task >= FG && task <= FG_END
}
// util:1 ends here

// [[file:../lbfgsb.note::*param][param:1]]
/// L-BFGS-B algorithm parameters
pub struct LbfgsbParameter {
  /// On entry m is the maximum number of variable metric corrections allowed
  /// in the limited memory matrix.
  pub m: usize,

  /// The tolerances in the stopping criteria for function value.
  ///
  /// On entry factr >= 0 is specified by the user. The iteration will stop
  /// when
  ///
  ///   (f^k - f^{k+1})/max{|f^k|,|f^{k+1}|,1} <= factr*epsmch
  ///
  /// where epsmch is the machine precision, which is automatically generated
  /// by the code.
  pub factr: f64,

  /// The tolerances in the stopping criteria for gradient.
  ///
  /// On entry pgtol >= 0 is specified by the user. The iteration will stop
  /// when
  ///
  ///   max{|proj g_i | i = 1, ..., n} <= pgtol
  ///
  /// where pg_i is the ith component of the projected gradient.
  pub pgtol: f64,

  // iprint controls the frequency and type of output generated:
  //
  //    iprint<0    no output is generated;
  //    iprint=0    print only one line at the last iteration;
  //    0<iprint<99 print also f and |proj g| every iprint iterations;
  //    iprint=99   print details of every iteration except n-vectors;
  //    iprint=100  print also the changes of active set and final x;
  //    iprint>100  print details of every iteration including x and g;
  //
  // When iprint > 0, the file iterate.dat will be created to summarize the
  // iteration.
  pub iprint: i64,
}

impl Default for LbfgsbParameter {
  fn default() -> Self {
      Self {
          m: 5,
          factr: 1E1,
          pgtol: 1E-5,
          iprint: -1,
      }
  }
}
// param:1 ends here

// [[file:../lbfgsb.note::*problem][problem:1]]
pub struct LbfgsbProblem<E>
where
  E: FnMut(&[f64], &mut [f64]) -> Result<f64>,
{
  pub x: Vec<f64>,
  pub g: Vec<f64>,
  pub f: f64,
  pub l: Vec<f64>,
  pub u: Vec<f64>,
  pub nbd: Vec<i64>,
  pub eval_fn: E,
}

impl<E> LbfgsbProblem<E>
where
  E: FnMut(&[f64], &mut [f64]) -> Result<f64>,
{
  pub fn build(x: Vec<f64>, eval_fn: E) -> Self {
    let n = x.len();
    Self {
      x,
      g: vec![0.0; n],
      f: 0.0,
      l: vec![0.0; n],
      u: vec![0.0; n],
      nbd: vec![0; n],
      eval_fn,
    }
  }

  pub fn reset(&mut self, len: usize) {
    if len > self.x.len() {
      self.x.resize(len, 0.0);
      self.g.resize(len, 0.0);
      self.l.resize(len, 0.0);
      self.u.resize(len, 0.0);
      self.nbd.resize(len, 0);
    }
    
    for i in 0..len {
      self.x[i] = 0.0;
      self.g[i] = 0.0;
      self.l[i] = 0.0;
      self.u[i] = 0.0;
      self.nbd[i] = 0;
    }
  }

  /// Set lower bounds and upper bounds for input variables
  pub fn set_bounds<B>(&mut self, bounds: B)
  where
      B: IntoIterator<Item = (Option<f64>, Option<f64>)>,
  {
      // nbd represents the type of bounds imposed on the variables, and must be
      // specified as follows:
      //
      //   nbd(i)=0 if x(i) is unbounded,
      //          1 if x(i) has only a lower bound,
      //          2 if x(i) has both lower and upper bounds, and
      //          3 if x(i) has only an upper bound.
      for (i, b) in bounds.into_iter().enumerate() {
          match b {
              // both lower and upper bonds
              (Some(l), Some(u)) => {
                  self.l[i] = l;
                  self.u[i] = u;
                  self.nbd[i] = 2;
              }
              // unbounded
              (None, None) => {
                  self.nbd[i] = 0;
              }
              // has only a lower bound
              (Some(l), None) => {
                  self.l[i] = l;
                  self.nbd[i] = 1;
              }
              // has only a upper bound
              (None, Some(u)) => {
                  self.u[i] = u;
                  self.nbd[i] = 3;
              }
          }
      }
  }
}
// problem:1 ends here

