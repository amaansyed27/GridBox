use serde_json::Value;

pub fn value_to_display(value: &Option<Value>) -> Option<String> {
    match value {
        Some(Value::String(text)) if !text.is_empty() => Some(text.clone()),
        Some(Value::Number(number)) => number
            .as_f64()
            .map(|value| format!("{value:.3}"))
            .or_else(|| Some(number.to_string())),
        Some(Value::Bool(value)) => Some(value.to_string()),
        _ => None,
    }
}
