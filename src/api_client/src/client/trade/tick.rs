use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QuoteTab {
    #[serde(rename = "d")]
    pub date: String,
    #[serde(rename = "c")]
    pub close: f64,
    #[serde(rename = "h")]
    pub high: f64,
    #[serde(rename = "l")]
    pub low: f64,
    #[serde(rename = "o")]
    pub open: f64,
    #[serde(rename = "v")]
    pub volume: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticks {
    pub quote_tab: Vec<QuoteTab>,
}

impl Ticks {
    pub fn get_highest_value(&self) -> Option<f64> {
        self.quote_tab
            .iter()
            .map(|quote| quote.high)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
    }

    pub fn get_lowest_value(&self) -> Option<f64> {
        self.quote_tab
            .iter()
            .map(|quote| quote.low)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
    }

    pub fn get_average_value(&self) -> Option<f64> {
        let sum: f64 = self.quote_tab.iter().map(|quote| quote.close).sum();
        Some(sum / self.quote_tab.len() as f64)
    }

    pub fn get_volume(&self) -> Option<f64> {
        let sum: f64 = self.quote_tab.iter().map(|quote| quote.volume).sum();
        Some(sum)
    }

    pub fn get_last_quote(&self) -> Option<QuoteTab> {
        self.quote_tab.last().cloned()
    }
}
