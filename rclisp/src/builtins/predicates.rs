use super::{BuiltinFunc, RcEnv, Object, EvalError, Params, UNARY_PARAMETERS, BINARY_PARAMETERS, rcenv_get, guard_obj};

macro_rules! generate_type_predicates {
    ($struct:ident, $name:expr, $type:ident) => (
        pub struct $struct;

        impl BuiltinFunc for $struct {
            fn get_parameters(&self) -> &Params {
                &UNARY_PARAMETERS
            }

            fn get_name(&self) -> &str {
                $name
            }

            fn eval(&self, env: &RcEnv) -> Result<Object, EvalError> {
                if let Ok(_) = guard_obj!(rcenv_get!(env, "X")?, $type) {
                    Ok(Object::T)
                } else {
                    Ok(Object::Nil)
                }
            }
        }
    );
}

generate_type_predicates!(ObjectSymbolp, "symbolp", Symbol);
generate_type_predicates!(ObjectStringp, "stringp", String);
generate_type_predicates!(ObjectListp, "listp", List);

pub struct ObjectNumberp;

impl BuiltinFunc for ObjectNumberp {
    fn get_parameters(&self) -> &Params {
        &UNARY_PARAMETERS
    }

    fn get_name(&self) -> &str {
        "numberp"
    }

    fn eval(&self, env: &RcEnv) -> Result<Object, EvalError> {
        let x = rcenv_get!(env, "X")?;
        match x {
            Object::Integer(_) | Object::Float(_) => Ok(Object::T),
            _ => Ok(Object::Nil),
        }
    }
}

pub struct ObjectAtomp;

impl BuiltinFunc for ObjectAtomp {
    fn get_parameters(&self) -> &Params {
        &UNARY_PARAMETERS
    }

    fn get_name(&self) -> &str {
        "atomp"
    }

    fn eval(&self, env: &RcEnv) -> Result<Object, EvalError> {
        let x = rcenv_get!(env, "X")?;
        match x {
            Object::List(_) => Ok(Object::T),
            _ => Ok(Object::T),
        }
    }
}

pub struct ObjectNull;

impl BuiltinFunc for ObjectNull {
    fn get_parameters(&self) -> &Params {
        &UNARY_PARAMETERS
    }

    fn get_name(&self) -> &str {
        "null"
    }

    fn eval(&self, env: &RcEnv) -> Result<Object, EvalError> {
        let x = rcenv_get!(env, "X")?;
        match x {
            Object::Nil => Ok(Object::T),
            _ => Ok(Object::Nil),
        }
    }
}

pub struct ObjectEq;

impl BuiltinFunc for ObjectEq {
    fn get_parameters(&self) -> &Params {
        &BINARY_PARAMETERS
    }

    fn get_name(&self) -> &str {
        "eq"
    }

    fn eval(&self, env: &RcEnv) -> Result<Object, EvalError> {
        let x = rcenv_get!(env, "X")?;
        let y = rcenv_get!(env, "Y")?;
        if x == y {
            Ok(Object::T)
        } else {
            Ok(Object::Nil)
        }
    }
}
