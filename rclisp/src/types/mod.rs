mod lambda;
mod list;

use std::fmt;

pub use lambda::{Lambda, UserLambda, Builtin, BuiltinFunc, Parameter, Params, ParamError};
pub use list::{cons, List};

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Nil,
    T,
    Integer(isize),
    Float(f64),
    String(String),
    Symbol(String),
    List(Box<List>),
    Lambda(Box<Lambda>),
    Quote(Box<Object>),
}

impl Object {
    pub fn print(&self) -> String {
        match self {
            Object::String(s) => s.into(),
            Object::Quote(o) => o.to_string(),
            _ => self.to_string(),
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Object::Nil => write!(f, "NIL"),
            Object::T => write!(f, "T"),
            Object::Integer(n) => write!(f, "{}", n),
            Object::Float(n) => write!(f, "{}", n),
            Object::String(s) => write!(f, "\"{}\"", s),
            Object::Symbol(atom) => write!(f, "{}", atom),
            Object::List(list) => write!(f, "{}", list),
            Object::Lambda(l) => write!(f, "{}", l),
            Object::Quote(o) => write!(f, "'{}", o),
        }
    }
}

impl From<Vec<Object>> for Object {
    fn from(v: Vec<Object>) -> Object {
        let mut v = v;
        if v.is_empty() {
            nil()
        } else if v.len() == 1 && matches!(v[0], Object::List(_)) {
            v[0].to_owned()
        } else {
            let mut ret = cons(v.pop().unwrap(), nil());
            for t in v.into_iter().rev() {
                ret = cons(t.to_owned(), ret);
            }
            ret
        }
    }
}

#[inline]
pub fn symbol<S: AsRef<str>>(name: S) -> Object {
    Object::Symbol(name.as_ref().to_string())
}

#[inline]
pub fn nil() -> Object {
    Object::Nil
}

#[inline]
pub fn quote(obj: Object) -> Object {
    Object::Quote(Box::new(obj))
}

#[inline]
pub fn int(int: isize) -> Object {
    Object::Integer(int)
}

#[inline]
pub fn get_list(obj: Object) -> List {
    if let Object::List(l) = obj {
        *l
    } else {
        unreachable!()
    }
}
