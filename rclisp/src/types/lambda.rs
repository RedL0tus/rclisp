use std::fmt;
use std::sync::Arc;
use std::error::Error;
use std::ops::{Deref, DerefMut};
use std::collections::HashSet;

use super::{List, Object};
use crate::env::RcEnv;
use crate::eval::EvalError;

pub trait BuiltinFunc: Sync + Send {
    fn eval(&self, env: &RcEnv) -> Result<Object, EvalError>;
    fn get_parameters(&self) -> &Params;
    fn get_name(&self) -> &str;
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParamError {
    InvalidType,
}

// TODO: Add support for keyed parameters
// TODO: Add support for type annotation
#[derive(Clone, Debug, PartialEq)]
pub enum Parameter {
    Normal(String),
    Plain(String),
    Rest(String),
    Optional(String, Object), // (Name, Default)
}

#[derive(Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct Params {
    inner: Vec<Parameter>,
}

// Use a separate struct so that I don't have to implement all traits manually
#[derive(Clone)]
pub struct Builtin {
    pub(crate) inner: Arc<dyn BuiltinFunc>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Lambda {
    Unnamed(UserLambda),
    Named(String, UserLambda),
    Builtin(Builtin),
}

#[derive(Clone, Debug, PartialEq)]
pub struct UserLambda {
    parameters: Params,
    body: List,
}

impl Parameter {
    pub fn normal<S: AsRef<str>>(name: S) -> Self {
        Self::Normal(name.as_ref().to_string())
    }

    pub fn plain<S: AsRef<str>>(name: S) -> Self {
        Self::Plain(name.as_ref().to_string())
    }

    pub fn rest<S: AsRef<str>>(name: S) -> Self {
        Self::Rest(name.as_ref().to_string())
    }

    pub fn optional<S: AsRef<str>>(name: S, default: Object) -> Self {
        Self::Optional(name.as_ref().to_string(), default)
    }

    pub fn get_name(&self) -> &str {
        match self {
            Self::Normal(n) | Self::Optional(n, _) | Self::Plain(n) | Self::Rest(n) => n,
        }
    }

    pub fn is_rest(&self) -> bool {
        matches!(self, Parameter::Rest(_))
    }
}

impl Params {
    pub fn len_required(&self) -> usize {
        self.inner.iter().fold(0, |acc, p| 
            if let Parameter::Optional(_, _) = p { acc } else { acc + 1 })
    }

    pub fn validate(&self) -> bool {
        let mut set = HashSet::new();
        let mut met_optional = false;
        let len = self.len();
        for param in &self.inner {
            let name = param.get_name();
            if set.contains(name) {
                return false;
            }
            set.insert(name);
            match param {
                Parameter::Normal(_) | Parameter::Plain(_) => {
                    if met_optional {
                        return false;
                    }
                },
                Parameter::Optional(_, _) => {
                    met_optional = true;
                },
                Parameter::Rest(_) => {
                    if len != set.len() {
                        return false;
                    }
                },
            }
        }
        true
    }
}

impl UserLambda {
    pub fn new(parameters: Params, body: List) -> Self {
        Self {
            parameters,
            body,
        }
    }

    pub fn get_parameters(&self) -> &Params {
        &self.parameters
    }

    pub fn get_body(self) -> List {
        self.body
    }
}

impl Builtin {
    pub fn new<F: BuiltinFunc + 'static>(inner: F) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }

    // pub fn get_name(&self) -> Object {
    //     Object::Symbol(self.inner.get_name().into())
    // }

    pub fn get_parameters(&self) -> &Params {
        self.inner.get_parameters()
    }
}

impl Lambda {
    pub fn get_parameters(&self) -> &Params {
        match self {
            Self::Named(_, l) | Self::Unnamed(l) => l.get_parameters(),
            Self::Builtin(l) => l.get_parameters(),
        }
    }
}

impl Deref for Params {
    type Target = Vec<Parameter>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Params {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl fmt::Display for ParamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            ParamError::InvalidType => write!(f, "Invalid param object type"),
        }
    }
}

impl Error for ParamError {}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Parameter::Normal(s) => write!(f, "({})", s),
            Parameter::Plain(s) => write!(f, "(&plain {})", s),
            Parameter::Rest(s) => write!(f, "(&rest {})", s),
            Parameter::Optional(name, def) => write!(f, "(&optional {} {})", name, def),
        }
    }
}

impl fmt::Display for Params {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let params: Vec<String> = self.inner.iter().map(|p| p.to_string()).collect();
        write!(f, "({})", params.join(", "))
    }
}

impl fmt::Debug for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "(BUILTIN:{} {})", self.inner.get_name(), self.get_parameters())
    }
}

impl fmt::Display for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl PartialEq<Builtin> for Builtin {
    fn eq(&self, _: &Builtin) -> bool {
        false
    }
}

impl fmt::Display for UserLambda {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.parameters)
    }
}

impl fmt::Display for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let str = match self {
            Self::Unnamed(l) => format!("(LAMBDA {})", l.get_parameters()),
            Self::Named(name, l) => format!("(NAMED-LAMBDA:{} {})", name, l.get_parameters()),
            Self::Builtin(b) => format!("{}", b),
        };
        write!(f, "#<FUNCTION {}>", str)
    }
}

impl From<Vec<Parameter>> for Params {
    fn from(params: Vec<Parameter>) -> Self {
        Self {
            inner: params,
        }
    }
}

impl TryFrom<List> for Params {
    type Error = ParamError;

    fn try_from(params: List) -> Result<Self, Self::Error> {
        let mut p = vec![];
        for param in params.into_iter() {
            if let Object::Symbol(s) = param {
                p.push(Parameter::normal(s));
            } else {
                return Err(ParamError::InvalidType);
            }
        }
        Ok(Self::from(p))
    }
}

impl From<Lambda> for Object {
    fn from(lambda: Lambda) -> Self {
        Self::Lambda(Box::new(lambda))
    }
}

impl<B: BuiltinFunc + 'static> From<B> for Builtin {
    fn from(b: B) -> Self {
        Self::new(b)
    }
}

impl From<Builtin> for Lambda {
    fn from(builtin: Builtin) -> Self {
        Self::Builtin(builtin)
    }
}

impl From<Builtin> for Object {
    fn from(b: Builtin) -> Self {
        Self::from(Lambda::from(b))
    }
}

impl<B: BuiltinFunc + 'static> From<B> for Object {
    fn from(b: B) -> Self {
        Self::from(Builtin::from(b))
    }
}
