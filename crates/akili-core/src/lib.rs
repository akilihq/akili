use serde_json::Value;

pub trait Tool {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn input_schema(&self) -> Value;
    fn call(&self, args: Value) -> Result<Value, String>;
}

pub trait ToolSet {
    fn name(&self) -> &str;
    fn tools(&self) -> Vec<Box<dyn Tool>>;
}
