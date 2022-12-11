use lazy_static::lazy_static;

use super::{BuiltinFunc, RcEnv, Object, EvalError, Params, Parameter, symbol, cons, guard_obj, rcenv_get};

use crate::types::{Lambda, UserLambda};
use crate::eval::Eval;

lazy_static! {
    static ref DEFUN_PARAMETERS: Params = Params::from(vec![Parameter::plain("X"), Parameter::plain("Y"), Parameter::rest("Z")]);
    static ref LAMBDA_PARAMETERS: Params = Params::from(vec![Parameter::plain("X"), Parameter::plain("Y")]);
    static ref FUNCALL_PARAMETERS: Params = Params::from(vec![Parameter::normal("X"), Parameter::plain("Y")]);
}

pub struct ObjectDefun;

impl BuiltinFunc for ObjectDefun {
    fn get_parameters(&self) -> &Params {
        &DEFUN_PARAMETERS
    }

    fn get_name(&self) -> &str {
        "defun"
    }

    fn eval(&self, env: &RcEnv) -> Result<Object, EvalError> {
        let name = guard_obj!(rcenv_get!(env, "X")?, Symbol)?;
        let params = *guard_obj!(rcenv_get!(env, "Y")?, List)?;
        let body = *guard_obj!(rcenv_get!(env, "Z")?, List)?;

        let p = Params::try_from(params)?;

        let lambda = Object::Lambda(Box::new(Lambda::Named(name.clone(), UserLambda::new(p, body))));
        
        env.borrow_mut().insert_global(&symbol(name), lambda);

        Ok(Object::Nil)
    }
}

pub struct ObjectLambda;

impl BuiltinFunc for ObjectLambda {
    fn get_parameters(&self) -> &Params {
        &LAMBDA_PARAMETERS
    }

    fn get_name(&self) -> &str {
        "lambda"
    }

    fn eval(&self, env: &RcEnv) -> Result<Object, EvalError> {
        // trace!("{:#?}", env);
        let params = *guard_obj!(rcenv_get!(env, "X")?, List)?;
        let body = *guard_obj!(rcenv_get!(env, "Y")?, List)?;

        let p = Params::try_from(params)?;

        Ok(Object::Lambda(Box::new(Lambda::Unnamed(UserLambda::new(p, body)))))
    }
}

pub struct ObjectFuncall;

impl BuiltinFunc for ObjectFuncall {
    fn get_parameters(&self) -> &Params {
        &FUNCALL_PARAMETERS
    }

    fn get_name(&self) -> &str {
        "funcall"
    }

    fn eval(&self, env: &RcEnv) -> Result<Object, EvalError> {
        let lambda = *guard_obj!(rcenv_get!(env, "X")?, Lambda)?;
        // trace!("Running lambda: {}", lambda);
        let params = *guard_obj!(rcenv_get!(env, "Y")?, List)?;
        cons(lambda.into(), params.into()).eval(env)
    }
}
