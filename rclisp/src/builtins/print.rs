use super::{BuiltinFunc, RcEnv, Object, EvalError, Params, EMPTY_PARAMETERS, UNARY_PARAMETERS, rcenv_get};

macro_rules! create_print_struct {
    ($struct:ident, $name:expr, $op:expr) => (
        pub struct $struct;

        impl BuiltinFunc for $struct {
            fn get_parameters(&self) -> &Params {
                &UNARY_PARAMETERS
            }

            fn get_name(&self) -> &str {
                $name
            }

            fn eval(&self, env: &RcEnv) -> Result<Object, EvalError> {
                let x = rcenv_get!(env, "X")?;
                $op(&x);
                Ok(x)
            }
        }
    );
}

create_print_struct!(ObjectPrint, "print", |x: &Object| println!("{}", x));
create_print_struct!(ObjectPrinc, "princ", |x: &Object| print!("{}", x.print()));

pub struct ObjectTerpri;

impl BuiltinFunc for ObjectTerpri {
    fn get_parameters(&self) -> &Params {
        &EMPTY_PARAMETERS
    }

    fn get_name(&self) -> &str {
        "terpri"
    }

    fn eval(&self, _env: &RcEnv) -> Result<Object, EvalError> {
        println!("");
        Ok(Object::Nil)
    }
}
