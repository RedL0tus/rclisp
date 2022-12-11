use super::{BuiltinFunc, RcEnv, Object, EvalError, Params, UNARY_PARAMETERS, rcenv_get};

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
                $op(x);
                Ok(Object::Nil)
            }
        }
    );
}

create_print_struct!(ObjectPrint, "print", |x: Object| println!("{}", x.print()));
create_print_struct!(ObjectPrinc, "princ", |x: Object| print!("{}", x.print()));
create_print_struct!(ObjectTerpri, "terpri", |_| println!());
