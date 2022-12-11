use lazy_static::lazy_static;

use crate::rcenv_get;

use super::{BuiltinFunc, RcEnv, Object, EvalError, Params, Parameter, guard_obj};

lazy_static! {
    static ref SETQ_PARAMETERS: Params = Params::from(vec![Parameter::plain("X"), Parameter::normal("Y")]);
}

pub struct ObjectSetq;

impl BuiltinFunc for ObjectSetq {
    fn get_parameters(&self) -> &Params {
        &SETQ_PARAMETERS
    }

    fn get_name(&self) -> &str {
        "setq"
    }

    fn eval(&self, env: &RcEnv) -> Result<Object, EvalError> {
        let key = guard_obj!(rcenv_get!(env, "X")?, Symbol)?;
        let value = rcenv_get!(env, "Y")?;
        env.borrow_mut().insert_global(&Object::Symbol(key), value);
        Ok(Object::Nil)
    }
}