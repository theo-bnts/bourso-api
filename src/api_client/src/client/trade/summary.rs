use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Amount {
    pub currency: Option<String>,
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    pub amount: Amount,
    pub label: String,
    pub quantity: Amount,
    pub symbol: String,
    // TODO: add other field like `valorisation`
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Summary {
    pub positions: Option<Vec<Position>>,
    // TODO: add other fields
}
