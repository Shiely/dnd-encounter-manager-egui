// DOMAIN LAYER: stdlib imports only.
#[derive(Debug, Clone, PartialEq)]
pub struct DiceExpression {
    pub value: String,
}

impl DiceExpression {
    pub fn new(value: &str) -> Result<Self, String> {
        // Basic validation
        if value.chars().all(|c| c.is_ascii_digit() || c == 'd' || c == '+' || c == '-') {
            Ok(Self { value: value.to_string() })
        } else {
            Err("Invalid dice expression".to_string())
        }
    }
}