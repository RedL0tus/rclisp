mod error;
mod list;
mod lambda;

use crate::env::{Env, RcEnv};
use crate::types::Object;

pub use error::EvalError;

pub trait Eval {
    fn eval(self, env: &RcEnv) -> Result<Object, EvalError>;
}

impl Eval for Object {
    fn eval(self, env: &RcEnv) -> Result<Object, EvalError> {
        match self {
            Self::Symbol(s) => Ok(env.borrow().get_str(s)?),
            Self::Quote(o) => Ok(*o),
            Self::Nil | Self::T | Self::Integer(_) | Self::Float(_) | Self::String(_) | Self::Lambda(_) => Ok(self),
            // Self::Lambda(_) => Ok(Object::Nil),
            Self::List(l) => l.eval(env),
        }
    }
}
