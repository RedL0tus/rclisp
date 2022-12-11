use lazy_static::lazy_static;

use super::{BuiltinFunc, RcEnv, Object, EvalError, Params, Parameter, symbol};

lazy_static! {
    static ref QUOTE_PARAMETERS: Params = Params::from(vec![Parameter::plain("X")]);
}

pub struct ObjectQuote;

impl BuiltinFunc for ObjectQuote {
    fn get_parameters(&self) -> &Params {
        &QUOTE_PARAMETERS
    }

    fn get_name(&self) -> &str {
        "quote"
    }

    fn eval(&self, env: &RcEnv) -> Result<Object, EvalError> {
        let x = env.borrow().get(&symbol("X"))?;
        Ok(Object::Quote(Box::new(x)))
    }
}
