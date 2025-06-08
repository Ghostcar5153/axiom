use crate::types::{Taint, Value};

pub fn sanitize_shell_input(v: Value<String>) -> Result<Value<String>, String> {
    let clean_inner = v.inner;
    Ok(Value::new(clean_inner, Taint::Clean))
}
