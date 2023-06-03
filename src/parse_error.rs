use crate::exceptions::{ExceptionRaise, InternalRunError, RunError};
use std::borrow::Cow;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ParseError {
    Todo(&'static str),
    Parsing(String),
    Internal(Cow<'static, str>),
    PreEvalExc(ExceptionRaise),
    PreEvalInternal(InternalRunError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Todo(s) => write!(f, "TODO: {s}"),
            Self::Internal(s) => write!(f, "Internal parsing error: {s}"),
            Self::Parsing(s) => write!(f, "Error parsing AST: {s}"),
            Self::PreEvalExc(s) => write!(f, "Pre eval exception: {s}"),
            Self::PreEvalInternal(s) => write!(f, "Pre eval internal error: {s}"),
        }
    }
}

impl ParseError {
    pub(crate) fn pre_eval(run_error: RunError) -> Self {
        match run_error {
            RunError::Exc(e) => Self::PreEvalExc(e),
            RunError::Internal(e) => Self::PreEvalInternal(e),
        }
    }
}

pub type ParseResult<T> = Result<T, ParseError>;

impl From<InternalRunError> for ParseError {
    fn from(internal_run_error: InternalRunError) -> Self {
        Self::PreEvalInternal(internal_run_error)
    }
}
