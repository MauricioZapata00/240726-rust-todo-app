use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathematicalResult {
    pub id: String,
    pub result: f64
}