use super::{BuiltinFunc, RcEnv, Object, EvalError, Params, UNARY_PARAMETERS, BINARY_PARAMETERS, rcenv_get, guard_obj};

use crate::types::cons;

pub struct ObjectCons;

impl BuiltinFunc for ObjectCons {
    fn get_parameters(&self) -> &Params {
        &BINARY_PARAMETERS
    }

    fn get_name(&self) -> &str {
        "cons"
    }

    fn eval(&self, env: &RcEnv) -> Result<Object, EvalError> {
        let x = rcenv_get!(env, "X")?;
        let y = rcenv_get!(env, "Y")?;
        Ok(cons(x, y))
    }
}

pub struct ObjectCar;

impl BuiltinFunc for ObjectCar {
    fn get_parameters(&self) -> &Params {
        &UNARY_PARAMETERS
    }

    fn get_name(&self) -> &str {
        "car"
    }

    fn eval(&self, env: &RcEnv) -> Result<Object, EvalError> {
        let x = guard_obj!(rcenv_get!(env, "X")?, List)?;
        Ok(x.car())
    }
}

pub struct ObjectCdr;

impl BuiltinFunc for ObjectCdr {
    fn get_parameters(&self) -> &Params {
        &UNARY_PARAMETERS
    }

    fn get_name(&self) -> &str {
        "cdr"
    }

    fn eval(&self, env: &RcEnv) -> Result<Object, EvalError> {
        let x = guard_obj!(rcenv_get!(env, "X")?, List)?;
        Ok(x.cdr())
    }
}
