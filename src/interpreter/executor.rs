use crate::common::{EvalError, Stmt, Value};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Error {
    EvaluationError(EvalError),
}

pub fn exec(stmt: &Stmt, env: &mut HashMap<String, Value>) -> Result<Option<Value>, Error> {
    match stmt {
        Stmt::Expression(a) => Ok(Some(a.eval(env).map_err(Error::EvaluationError)?)),
        Stmt::Declare(_, _) => todo!(),
        Stmt::Assign(_, _) => todo!(),
        Stmt::Output(vs) => {
            println!(
                "{}",
                vs.iter()
                    .map(|expr| expr
                        .eval(env)
                        .map(|v| v.to_string())
                        .map_err(Error::EvaluationError))
                    .collect::<Result<Vec<String>, Error>>()?
                    .join(" ")
            );
            Ok(None)
        }
        Stmt::Input(_) => todo!(),
    }
}
