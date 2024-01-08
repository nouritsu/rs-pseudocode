use crate::common::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    env: HashMap<String, Value>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
            parent: None,
        }
    }

    pub fn create_scope(&mut self) {
        self.parent = Some(Box::new(self.clone()));
        self.env = HashMap::new();
    }

    pub fn end_scope(&mut self) {
        if let Some(parent) = &self.parent {
            self.env = parent.env.clone();
            self.parent = None;
        }
    }

    pub fn get_value(&self, var: &str) -> Option<Value> {
        match self.env.get(var) {
            Some(val) => Some(val.clone()),
            None => {
                if let Some(parent) = &self.parent {
                    parent.get_value(var)
                } else {
                    None
                }
            }
        }
    }
}
