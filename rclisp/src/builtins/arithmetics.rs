use std::ops::{Add, Sub, Mul, Div};

use super::{BuiltinFunc, RcEnv, Object, EvalError, Params, BINARY_PARAMETERS, rcenv_get};

macro_rules! create_arithmetic_struct {
    ($struct:ident, $name:expr, $op:ident) => (
        pub struct $struct;

        impl BuiltinFunc for $struct {
            fn get_parameters(&self) -> &Params {
                &BINARY_PARAMETERS
            }

            fn get_name(&self) -> &str {
                $name
            }

            fn eval(&self, env: &RcEnv) -> Result<Object, EvalError> {
                let x = rcenv_get!(env, "X")?;
                let y = rcenv_get!(env, "Y")?;
                match (x, y) {
                    (Object::Integer(x), Object::Integer(y)) => Ok(Object::Integer(x.$op(y))),
                    (Object::Float(x), Object::Float(y)) => Ok(Object::Float(x.$op(y))),
                    (Object::Integer(x), Object::Float(y)) | (Object::Float(y), Object::Integer(x)) => Ok(Object::Float((x as f64).$op(y))),
                    _ => Err(EvalError::ParameterTypeMismatched),
                }
            }
        }
    );
}

create_arithmetic_struct!(ObjectAdd, "+", add);
create_arithmetic_struct!(ObjectSub, "-", sub);
create_arithmetic_struct!(ObjectMul, "*", mul);
create_arithmetic_struct!(ObjectDiv, "/", div);
