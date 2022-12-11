use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

use crate::types::Object;

#[macro_export]
macro_rules! rcenv_get {
    ($env:expr, $key:expr) => {
        $env.borrow().get(&$crate::types::symbol($key))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EnvError {
    SymbolNotFound(String),
    NotASymbol(String),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    inner: HashMap<String, Object>,
}

pub type RcEnv = Rc<RefCell<Env>>;

impl fmt::Display for EnvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::SymbolNotFound(s) => write!(f, "Symbol \"{}\" not found", s),
            Self::NotASymbol(s) => write!(f, "{} is not a symbol", s),
        }
    }
}

impl Error for EnvError {}

impl Env {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn inherit(parent: &Rc<RefCell<Self>>) -> Self {
        Self {
            parent: Some(parent.clone()),
            inner: HashMap::new(),
        }
    }

    pub fn wrap(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }

    pub fn get_str<S: AsRef<str>>(&self, key: S) -> Result<Object, EnvError> {
        if let Some(self_res) = self.inner.get(key.as_ref()) {
            Ok(self_res.clone())
        } else if let Some(parent) = &self.parent {
            parent.borrow().get_str(key)
        } else {
            Err(EnvError::SymbolNotFound(key.as_ref().into()))
        }
    }

    pub fn get(&self, key: &Object) -> Result<Object, EnvError> {
        let k = if let Object::Symbol(s) = key {
            s.to_uppercase()
        } else {
            return Err(EnvError::NotASymbol(key.to_string()));
        };
        self.get_str(k)
    }

    pub fn insert_str<S: AsRef<str>>(&mut self, key: S, value: Object) {
        self.inner.insert(key.as_ref().to_string(), value);
    }

    pub fn insert(&mut self, key: &Object, value: Object) {
        if let Object::Symbol(s) = key {
            self.insert_str(s.to_uppercase(), value)
        } else {
            unreachable!();
        }
    }

    pub fn insert_global_str<S: AsRef<str>>(&mut self, key: S, value: Object) {
        if self.parent.is_none() {
            self.insert_str(key, value);
            return;
        }
        if self.inner.get(key.as_ref()).is_some() {
            self.inner.remove(key.as_ref());
        }
        if let Some(parent) = &self.parent {
            parent.borrow_mut().insert_global_str(key, value);
        }
    }

    pub fn insert_global(&mut self, key: &Object, value: Object) {
        if let Object::Symbol(k) = key {
            self.insert_global_str(k.to_uppercase(), value);
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Env, Object};

    #[test]
    fn test_env_simple() {
        let env = Env::new().wrap();
        env.borrow_mut()
            .insert(&Object::Symbol("A".into()), Object::Symbol("AValue".into()));
        assert_eq!(
            env.borrow().get(&Object::Symbol("A".into())),
            Ok(Object::Symbol("AValue".into()))
        );
    }

    #[test]
    fn test_env_inherit() {
        let p = Env::new().wrap();
        p.borrow_mut()
            .insert(&Object::Symbol("A".into()), Object::Symbol("AValue".into()));
        let c = Env::inherit(&p).wrap();
        assert_eq!(
            c.borrow().get(&Object::Symbol("A".into())),
            Ok(Object::Symbol("AValue".into()))
        );
        p.borrow_mut()
            .insert(&Object::Symbol("B".into()), Object::Symbol("BValue".into()));
        assert_eq!(
            p.borrow().get(&Object::Symbol("B".into())),
            Ok(Object::Symbol("BValue".into()))
        );
        assert_eq!(
            c.borrow().get(&Object::Symbol("B".into())),
            Ok(Object::Symbol("BValue".into()))
        );
        c.borrow_mut().insert(
            &Object::Symbol("B".into()),
            Object::Symbol("BValueChild".into()),
        );
        assert_eq!(
            c.borrow().get(&Object::Symbol("B".into())),
            Ok(Object::Symbol("BValueChild".into()))
        );
    }

    #[test]
    fn test_env_insert_global() {
        let p = Env::new().wrap();
        p.borrow_mut()
            .insert(&Object::Symbol("A".into()), Object::Symbol("AValue1".into()));
        let c = Env::inherit(&p).wrap();
        assert_eq!(
            c.borrow().get(&Object::Symbol("A".into())),
            Ok(Object::Symbol("AValue1".into()))
        );
        p.borrow_mut()
            .insert(&Object::Symbol("A".into()), Object::Symbol("AValue2".into()));
        assert_eq!(
            c.borrow().get(&Object::Symbol("A".into())),
            Ok(Object::Symbol("AValue2".into()))
        );
        assert_eq!(
            p.borrow().get(&Object::Symbol("A".into())),
            Ok(Object::Symbol("AValue2".into()))
        );
    }
}
