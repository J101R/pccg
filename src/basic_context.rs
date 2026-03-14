use std::sync::Arc;
use minijinja::{Value, value::Object};

#[derive(Debug)]
pub struct BasicContext {
    pub(crate) context: Option<String>,
    pub(crate) status: String,
    pub(crate) diff: String,
}

impl Object for BasicContext {
    fn get_value(self: &Arc<Self>, field: &Value) -> Option<Value> {
        match field.as_str()? {
            "context" => Some(Value::from(self.context.clone())),
            "status" => Some(Value::from(self.status.clone())),
            "diff" => Some(Value::from(self.diff.clone())),
            _ => None
        }
    }
}
