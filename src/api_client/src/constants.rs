pub const BASE_URL: &str = "https://clients.boursobank.com";
pub const SAVINGS_PATTERN: &str = r#"<a href="/epargne/assurance-vie/contrat-boursorama-vie/gerer/s-[a-z0-9]+">"#;
pub const BANKING_PATTERN: &str = r#"<a href="/compte-bancaire/boursorama-banque/gerer/s-[a-z0-9]+">"#;
pub const TRADING_PATTERN: &str = r#"<a href="/compte-titres/pea/gerer/s-[a-z0-9]+">"#;
pub const LOANS_PATTERN: &str = r#"<a href="/credit/immobilier/gerer/s-[a-z0-9]+">"#;
pub const ACCOUNT_PATTERN: &str = r#"<a href="[^"]*?/s-([a-z0-9]+)">"#;
