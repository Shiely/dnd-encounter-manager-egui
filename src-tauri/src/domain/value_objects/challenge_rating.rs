// DOMAIN LAYER: stdlib imports only.
#[derive(Debug, Clone, PartialEq)]
pub struct ChallengeRating {
    pub value: String,
}

impl ChallengeRating {
    pub fn new(value: &str) -> Result<Self, String> {
        let valid = ["0", "1/8", "1/4", "1/2", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];
        if valid.contains(&value) {
            Ok(Self { value: value.to_string() })
        } else {
            Err(format!("Invalid CR: {}", value))
        }
    }

    pub fn decimal(&self) -> f64 {
        match self.value.as_str() {
            "1/8" => 0.125,
            "1/4" => 0.25,
            "1/2" => 0.5,
            other => other.parse().unwrap_or(0.0),
        }
    }
}