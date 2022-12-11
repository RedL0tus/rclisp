use super::{Eval, EvalError};
use crate::types::{Object, Lambda, UserLambda, Builtin};
use crate::env::RcEnv;

impl Eval for UserLambda {
    fn eval(self, env: &RcEnv) -> Result<Object, EvalError> {
        self.get_body().eval(env)
    }
}

impl Eval for Builtin {
    fn eval(self, env: &RcEnv) -> Result<Object, EvalError> {
        self.inner.eval(env)
    }
}

impl Eval for Lambda {
    fn eval(self, env: &RcEnv) -> Result<Object, EvalError> {
        match self {
            Self::Named(_, l) | Self::Unnamed(l) => l.eval(env),
            Self::Builtin(b) => b.eval(env),
        }
    }
}
