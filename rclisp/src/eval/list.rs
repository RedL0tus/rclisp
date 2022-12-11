use log::trace;

use crate::types::{List, Parameter, nil, cons};
use super::{Eval, EvalError, RcEnv, Object, Env};

fn collect_parameters(rest: Vec<Object>) -> Object {
    let mut v = rest;
    if v.is_empty() {
        nil()
    } else {
        let mut ret = cons(v.pop().unwrap(), nil());
        for t in v.into_iter().rev() {
            ret = cons(t.to_owned(), ret);
        }
        ret
    }
}

impl Eval for List {
    fn eval(self, env: &RcEnv) -> Result<Object, EvalError> {
        trace!("eval list: {}", self);

        // Empty list == nil
        let len = self.len();
        if len == 0 {
            return Ok(Object::Nil);
        }

        // Check function call format
        let (car, cdr) = self.unpack();
        trace!("car: {}, cdr: {}", car, cdr);
        let first_result = car.eval(env)?;
        if cdr == Object::Nil {
            return Ok(first_result);
        }
        let lambda = if let Object::Lambda(l) = first_result {
            *l
        } else if let Object::List(l) = cdr {
            let mut ret = Object::Nil;
            for obj in l.into_iter() {
                ret = obj.eval(env)?;
            }
            return Ok(ret);
        } else {
            return cdr.eval(env);
        };
        trace!("Calling {}", lambda);
        let cdr = if let Object::List(l) = cdr {
            *l
        } else {
            return Err(EvalError::IllegalFunctionCall)
        };

        // Check parameter count
        let params = lambda.get_parameters();
        let len = len.saturating_sub(1); // Avoid overflow
        if len < params.len_required() {
            return Err(EvalError::UnmatchedNumberOfParameters(params.len(), len));
        }

        // If the lambda requires no parameters
        if params.is_empty() {
            return lambda.eval(env);
        }

        // Bind parameters
        let new_env = Env::inherit(env).wrap();
        let mut values = cdr.into_iter();
        for param in params.iter() {
            let value = values.next();
            let (name, val) = match param {
                Parameter::Rest(name) => {
                    let mut rest = vec![value.unwrap()];
                    let mut iter_rest: Vec<Object> = values.collect();
                    rest.append(&mut iter_rest);
                    new_env.borrow_mut().insert_str(name, collect_parameters(rest));
                    return lambda.eval(&new_env);
                },
                Parameter::Normal(name) => {
                    let val = value.expect("Failed to find matching normal value").eval(env)?;
                    (name, val)
                },
                Parameter::Plain(name) => {
                    let val = value.expect("Failed to find matching plain value");
                    (name, val)
                },
                Parameter::Optional(name, default) => {
                    let val = if let Some(v) = value {
                        v
                    } else {
                        default.clone()
                    };
                    trace!("Binding {} to {}", name, val);
                    (name, val)
                },
            };
            new_env.borrow_mut().insert_str(name, val);
        }
        lambda.eval(&new_env)
    }
}
