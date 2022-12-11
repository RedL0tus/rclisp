use std::fmt;
use std::error::Error;

use crate::env::EnvError;
use crate::types::ParamError;

#[derive(Clone, Debug, PartialEq)]
pub enum EvalError {
    UnboundVariable(String),
    UnmatchedNumberOfParameters(usize, usize),
    IllegalFunctionCall,
    ParameterTypeMismatched,
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::UnboundVariable(s) => write!(f, "Unbound variable {}", s),
            Self::UnmatchedNumberOfParameters(exp, act) => write!(f, "Unmatched number of parameters, expecting {} but got {}", exp, act),
            Self::IllegalFunctionCall => write!(f, "Illegal function call"),
            Self::ParameterTypeMismatched => write!(f, "Parameter type mismatched"),
        }
    }
}

impl Error for EvalError {}

impl From<EnvError> for EvalError {
    fn from(e: EnvError) -> Self {
        match e {
            EnvError::SymbolNotFound(s) => Self::UnboundVariable(s),
            _ => unreachable!(),
        }
    }
}

impl From<ParamError> for EvalError {
    fn from(p: ParamError) -> Self {
        match p {
            ParamError::InvalidType => Self::ParameterTypeMismatched,
        }
    }
}
