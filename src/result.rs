use crate::error::{EvaluatorError, ExecutorError, ValueError};

pub type ExecutorResult<T> = Result<T, ExecutorError>;
pub type EvaluatorResult<T> = Result<T, EvaluatorError>;
pub type ValueResult<T> = Result<T, ValueError>;
