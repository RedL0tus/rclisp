use lazy_static::lazy_static;
use log::trace;

use super::{BuiltinFunc, RcEnv, Object, EvalError, Params, Parameter, rcenv_get, guard_obj};

use crate::eval::Eval;

lazy_static! {
    static ref COND_PARAMETERS: Params = Params::from(vec![Parameter::rest("X")]);
}

pub struct ObjectCond;

impl BuiltinFunc for ObjectCond {
    fn get_parameters(&self) -> &Params {
        &COND_PARAMETERS
    }

    fn get_name(&self) -> &str {
        "cond"
    }

    fn eval(&self, env: &RcEnv) -> Result<Object, EvalError> {
        let mut lst = rcenv_get!(env, "X")?;
        trace!("X: {:?}", lst);
        while lst != Object::Nil {
            let (clause, cdr) = guard_obj!(lst, List)?.unpack();
            trace!("current clause: {}", clause);
            let (cond, body) = guard_obj!(clause, List)?.unpack();
            if cond.eval(env)? == Object::T {
                trace!("running: {}", body);
                return body.eval(env);
            }
            lst = cdr;
        }
        Ok(Object::Nil)
    }
}