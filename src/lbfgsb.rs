// [[file:../lbfgsb.note::*imports][imports:1]]

#[allow(clippy::all)]
mod bindings { include!(concat!(env!("OUT_DIR"), "/bindings_0.rs")); }
use bindings::{integer, logical, NEW_X};

extern "C" {
    #[allow(clashing_extern_declarations)]
    pub fn setulb(
        n: *const integer,
        m: *const integer,
        x: *mut f64,
        l: *const f64,
        u: *const f64,
        nbd: *const integer,
        f: *mut f64,
        g: *mut f64,
        factr: *const f64,
        pgtol: *const f64,
        wa: *mut f64,
        iwa: *mut integer,
        task: *mut integer,
        iprint: *const integer,
        csave: *mut integer,
        lsave: *mut logical,
        isave: *mut integer,
        dsave: *mut f64,
    ) -> ::std::os::raw::c_int;
}

use anyhow::Result;

use crate::shared::{is_fg, LbfgsbParameter, LbfgsbProblem, LbfgsbState};
// imports:1 ends here

impl<'a, E> LbfgsbState<'a, E>
where
    E: FnMut(&[f64], &mut [f64]) -> Result<f64>,
{
    pub(crate) fn minimize(&mut self) -> Result<()> {
        let f = &mut self.problem.f;
        let x = &mut self.problem.x;
        let g = &mut self.problem.g;
        let l = &self.problem.l;
        let u = &self.problem.u;
        let nbd = &self.problem.nbd;

        let param = &self.param;
        let n = x.len();
        let m = param.m;
        loop {
            unsafe {
                #[allow(clashing_extern_declarations)]
                setulb(
                    &(n as i64),             //x
                    &(m as i64),             //x
                    x.as_mut_ptr(),          //x
                    l.as_ptr(),              //x
                    u.as_ptr(),              //x
                    nbd.as_ptr(),            //x
                    f,                       //x
                    g.as_mut_ptr(),          //x
                    &param.factr,            //x
                    &param.pgtol,            //x
                    self.wa.as_mut_ptr(),    //x
                    self.iwa.as_mut_ptr(),   //x
                    &mut self.task,          //x
                    &param.iprint,           //x
                    self.csave.as_mut_ptr(), //x
                    self.lsave.as_mut_ptr(), //x
                    self.isave.as_mut_ptr(), //x
                    self.dsave.as_mut_ptr(), //x
                );
            }
            if is_fg(self.task) {
                // the minimization routine has returned to request the
                // function f and gradient g values at the current x.
                // Compute function value f for the sample problem.
                *f = (self.problem.eval_fn)(x, g)?;
            // go back to the minimization routine.
            } else if self.task == NEW_X as i64 {
                // the minimization routine has returned with a new iterate, and we have
                // opted to continue the iteration.
            } else {
                // If task is neither FG nor NEW_X we terminate execution.
                break;
            }
        }

        Ok(())
    }
}
// 9e5b03b1 ends here

// [[file:../lbfgsb.note::*pub][pub:1]]
/// Minimize a scalar function of one or more variables using the L-BFGS-B
/// algorithm.
///
/// # Parameters
///
/// - bounds: a slice of tuple setting lower and upper bounds.
/// - eval_fn: a closure evaluating f(x) and g(x). Returning Err value will cancel minimization.
///
/// # Return
///
/// - Returns final state containing x, f(x), g(x).
pub fn lbfgsb<E>(problem: &mut LbfgsbProblem<E>, params: LbfgsbParameter) -> Result<()>
where
    E: FnMut(&[f64], &mut [f64]) -> Result<f64>,
{
    let mut state = LbfgsbState::new(problem, params);
    state.minimize()
    // Ok(state.x().to_vec())
}
// pub:1 ends here
