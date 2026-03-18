use std::sync::Arc;
use minijinja::{Value, value::Object};

#[derive(Debug)]
pub struct BasicContext {
    pub context: Option<String>,
    pub status: String,
    pub diff: String,
    pub style: String
}

impl Object for BasicContext {
    fn get_value(self: &Arc<Self>, field: &Value) -> Option<Value> {
        match field.as_str()? {
            "context" => Some(Value::from(self.context.as_ref())),
            "status" => Some(Value::from(self.status.as_str())),
            "diff" => Some(Value::from(self.diff.as_str())),
            "style" => Some(Value::from(self.style.as_str())),
            _ => None
        }
    }
}
