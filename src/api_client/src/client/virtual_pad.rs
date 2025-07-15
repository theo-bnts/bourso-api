use anyhow::{anyhow, Context, Result};
use regex::Regex;
use serde_json::Value;

pub fn extract_data_matrix_keys(res: &str) -> Result<Vec<String>> {
    let re = Regex::new(r#"data-matrix-keys="([^"]*)""#)?;
    let caps = re.captures(res).context("No data-matrix-keys found")?;
    let keys_str = caps.get(1).context("No data-matrix-keys found")?.as_str();

    let keys: Vec<String> = serde_json::from_str(keys_str)?;
    Ok(keys)
}

pub fn extract_challenge_token(res: &str) -> Result<String> {
    let re = Regex::new(r#"name="challengeToken" value="([^"]*)""#)?;
    let caps = re.captures(res).context("No challengeToken found")?;
    let token = caps.get(1).context("No challengeToken found")?.as_str();
    Ok(token.to_string())
}

pub fn password_to_virtual_pad_keys(
    virtual_pad_ids: Vec<String>,
    password: &str,
) -> Result<Value> {
    let mut keys = Vec::new();
    for (i, c) in password.chars().enumerate() {
        let pad_id = virtual_pad_ids
            .iter()
            .find(|&id| id.contains(c))
            .context(format!("Could not find pad for character: {}", c))?;
        keys.push(serde_json::json!({
            "id": pad_id,
            "val": c.to_string(),
            "rank": i + 1,
        }));
    }
    Ok(serde_json::Value::Array(keys))
}
