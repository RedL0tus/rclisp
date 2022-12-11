pub mod arithmetics;
pub mod quote;
pub mod print;
pub mod lambda;
pub mod predicates;
pub mod setq;
pub mod list;
pub mod conditional;

use lazy_static::lazy_static;

pub use crate::types::{Object, BuiltinFunc, List, Params, Parameter, cons, nil, symbol, get_list};
pub use crate::env::{Env, RcEnv};
pub use crate::eval::EvalError;
pub use crate::generate_symbol_list;

pub use arithmetics::{ObjectAdd, ObjectSub, ObjectMul, ObjectDiv};
pub use quote::ObjectQuote;
pub use print::{ObjectPrint, ObjectPrinc, ObjectTerpri};
pub use lambda::{ObjectDefun, ObjectLambda, ObjectFuncall};
pub use predicates::{ObjectSymbolp, ObjectNumberp, ObjectStringp, ObjectAtomp, ObjectListp, ObjectNull, ObjectEq};
pub use setq::ObjectSetq;
pub use list::{ObjectCons, ObjectCar, ObjectCdr};
pub use conditional::ObjectCond;

pub use crate::{guard_obj, rcenv_get};

lazy_static! {
    static ref UNARY_PARAMETERS: Params = Params::from(vec![Parameter::normal("X")]);
    static ref BINARY_PARAMETERS: Params = Params::from(vec![Parameter::normal("X"), Parameter::normal("Y")]);
    static ref TERNARY_PARAMETERS: Params = Params::from(vec![Parameter::normal("X"), Parameter::normal("Y"), Parameter::normal("Z")]);
}

#[macro_export]
macro_rules! guard_obj {
    ($obj:expr, $variant:ident) => (
        if let $crate::types::Object::$variant(v) = $obj {
            Ok(v)
        } else {
            Err($crate::eval::EvalError::ParameterTypeMismatched)
        }
    );
}

macro_rules! insert_builtin {
    ($env:ident, $($x:ident),+) => {
        $({
            let name = symbol($x.get_name());
            let lambda = Object::from($x);
            $env.borrow_mut().insert(&name, lambda);
        });+
    };
}

pub fn generate_default_env() -> RcEnv {
    let env = Env::new().wrap();
    insert_builtin!(env,
        ObjectAdd,
        ObjectSub,
        ObjectMul,
        ObjectDiv,
        ObjectQuote,
        ObjectPrint,
        ObjectPrinc,
        ObjectTerpri,
        ObjectDefun,
        ObjectLambda,
        ObjectFuncall,
        ObjectSymbolp,
        ObjectNumberp,
        ObjectStringp,
        ObjectAtomp,
        ObjectListp,
        ObjectNull,
        ObjectEq,
        ObjectSetq,
        ObjectCons,
        ObjectCar,
        ObjectCdr,
        ObjectCond
    );
    env
}
