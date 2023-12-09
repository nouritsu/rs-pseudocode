use crate::{error::ExecutorError, eval, result::ExecutorResult, Stmt, Value};
use std::collections::HashMap;

pub fn exec(stmt: &Stmt, env: &mut HashMap<String, Value>) -> ExecutorResult<Option<Value>> {
    Ok(match stmt {
        Stmt::Expression(expr) => Some(eval(expr, env).map_err(|e| ExecutorError::Evaluation(e))?),
        Stmt::Declare(_, _) => todo!(),
        Stmt::Assign(_, _) => todo!(),
        Stmt::Output(exprs) => {
            println!(
                "{}",
                exprs
                    .iter()
                    .map(|expr| eval(expr, env).map(|v| v.to_string()))
                    .collect::<Result<Vec<String>, _>>()
                    .map_err(|e| ExecutorError::Evaluation(e))?
                    .join(" ")
            );
            None
        }
        Stmt::Input(_) => todo!(),
    })
}
