use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecutorError {
    #[error("tried to declare variable more than once")]
    RepeatDeclare,

    #[error("unable to evaluate, reason: {0}")]
    Evaluation(EvaluatorError),
}

#[derive(Error, Debug)]
pub enum EvaluatorError {
    #[error("{0}")]
    Value(ValueError),

    #[error("variable {0} not found")]
    VariableNotFound(String),
}

#[derive(Error, Debug)]
pub enum ValueError {
    #[error("invalid type")]
    InvalidType,

    #[error("operation not supported")]
    InvalidOperation,
}
