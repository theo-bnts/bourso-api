use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use bourso_api::{
    account::{Account, AccountKind},
    client::{
        trade::{order::OrderSide, tick::QuoteTab},
        BoursoWebClient,
    },
    get_client,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct QuoteRequest {
    symbol: String,
    length: Option<String>,
    interval: Option<String>,
}

#[derive(Deserialize)]
pub struct OrderRequest {
    credentials: Credentials,
    account_id: String,
    side: OrderSide,
    symbol: String,
    quantity: usize,
}

#[derive(Deserialize)]
pub struct PositionRequest {
    credentials: Credentials,
    account_id: String,
}

pub fn router() -> Router {
    Router::new()
        .route("/accounts", post(get_accounts))
        .route("/accounts/banking", post(get_banking_accounts))
        .route("/accounts/savings", post(get_savings_accounts))
        .route("/accounts/trading", post(get_trading_accounts))
        .route("/accounts/loans", post(get_loans_accounts))
        .route("/quotes", post(get_quotes))
        .route("/quotes/:symbol", get(get_quote_by_symbol))
        .route("/quotes/:symbol/highest", get(get_highest_quote))
        .route("/quotes/:symbol/lowest", get(get_lowest_quote))
        .route("/quotes/:symbol/average", get(get_average_quote))
        .route("/quotes/:symbol/volume", get(get_volume))
        .route("/quotes/:symbol/last", get(get_last_quote))
        .route("/trade/orders", post(create_order))
        .route("/trade/positions", post(get_positions))
}

async fn get_accounts_by_kind(
    credentials: Credentials,
    kind: Option<AccountKind>,
) -> Result<Json<Vec<Account>>, AppError> {
    let mut web_client: BoursoWebClient = get_client();
    web_client.init_session().await?;
    web_client
        .login(&credentials.username, &credentials.password)
        .await?;

    let accounts = web_client.get_accounts(kind).await?;
    Ok(Json(accounts))
}

async fn get_accounts(
    Json(credentials): Json<Credentials>,
) -> Result<Json<Vec<Account>>, AppError> {
    get_accounts_by_kind(credentials, None).await
}

async fn get_banking_accounts(
    Json(credentials): Json<Credentials>,
) -> Result<Json<Vec<Account>>, AppError> {
    get_accounts_by_kind(credentials, Some(AccountKind::Banking)).await
}

async fn get_savings_accounts(
    Json(credentials): Json<Credentials>,
) -> Result<Json<Vec<Account>>, AppError> {
    get_accounts_by_kind(credentials, Some(AccountKind::Savings)).await
}

async fn get_trading_accounts(
    Json(credentials): Json<Credentials>,
) -> Result<Json<Vec<Account>>, AppError> {
    get_accounts_by_kind(credentials, Some(AccountKind::Trading)).await
}

async fn get_loans_accounts(
    Json(credentials): Json<Credentials>,
) -> Result<Json<Vec<Account>>, AppError> {
    get_accounts_by_kind(credentials, Some(AccountKind::Loans)).await
}

async fn get_ticks(
    symbol: &str,
    length: Option<String>,
    interval: Option<String>,
) -> Result<bourso_api::client::trade::tick::Ticks, AppError> {
    let web_client: BoursoWebClient = get_client();

    let quotes = web_client
        .get_ticks(
            symbol,
            length.unwrap_or("30".to_string()).parse()?,
            interval.unwrap_or("0".to_string()).parse()?,
        )
        .await?;
    Ok(quotes)
}

async fn get_quotes(
    Json(quote_request): Json<QuoteRequest>,
) -> Result<Json<bourso_api::client::trade::tick::Ticks>, AppError> {
    let quotes = get_ticks(
        &quote_request.symbol,
        quote_request.length,
        quote_request.interval,
    )
    .await?;
    Ok(Json(quotes))
}

async fn get_quote_by_symbol(
    Path(symbol): Path<String>,
) -> Result<Json<bourso_api::client::trade::tick::Ticks>, AppError> {
    let quotes = get_ticks(&symbol, None, None).await?;
    Ok(Json(quotes))
}

async fn get_highest_quote(Path(symbol): Path<String>) -> Result<Json<f64>, AppError> {
    let quotes = get_ticks(&symbol, None, None).await?;
    Ok(Json(quotes.d.get_highest_value().unwrap_or(0.0)))
}

async fn get_lowest_quote(Path(symbol): Path<String>) -> Result<Json<f64>, AppError> {
    let quotes = get_ticks(&symbol, None, None).await?;
    Ok(Json(quotes.d.get_lowest_value().unwrap_or(0.0)))
}

async fn get_average_quote(Path(symbol): Path<String>) -> Result<Json<f64>, AppError> {
    let quotes = get_ticks(&symbol, None, None).await?;
    Ok(Json(quotes.d.get_average_value().unwrap_or(0.0)))
}

async fn get_volume(Path(symbol): Path<String>) -> Result<Json<f64>, AppError> {
    let quotes = get_ticks(&symbol, None, None).await?;
    Ok(Json(quotes.d.get_volume().unwrap_or(0.0)))
}

async fn get_last_quote(Path(symbol): Path<String>) -> Result<Json<QuoteTab>, AppError> {
    let quotes = get_ticks(&symbol, None, None).await?;
    let last_quote = quotes.d.get_last_quote().unwrap();
    Ok(Json(last_quote))
}

async fn create_order(Json(order_request): Json<OrderRequest>) -> Result<Json<()>, AppError> {
    let mut web_client: BoursoWebClient = get_client();
    web_client.init_session().await?;
    web_client
        .login(
            &order_request.credentials.username,
            &order_request.credentials.password,
        )
        .await?;

    let accounts = web_client.get_accounts(Some(AccountKind::Trading)).await?;
    let account = accounts
        .iter()
        .find(|a| a.id == order_request.account_id)
        .ok_or(anyhow::anyhow!("Account not found"))?;

    web_client
        .order(
            order_request.side,
            account,
            &order_request.symbol,
            order_request.quantity,
            None,
        )
        .await?;

    Ok(Json(()))
}

async fn get_positions(
    Json(position_request): Json<PositionRequest>,
) -> Result<Json<Vec<bourso_api::client::trade::summary::Position>>, AppError> {
    let mut web_client: BoursoWebClient = get_client();
    web_client.init_session().await?;
    web_client
        .login(
            &position_request.credentials.username,
            &position_request.credentials.password,
        )
        .await?;

    let accounts = web_client.get_accounts(Some(AccountKind::Trading)).await?;
    let account = accounts
        .iter()
        .find(|a| a.id == position_request.account_id)
        .ok_or(anyhow::anyhow!("Account not found"))?;

    let summary = web_client.get_trading_summary(account.clone()).await?;

    let mut positions = Vec::new();
    for item in summary {
        if let Some(p) = item.positions {
            positions.extend(p);
        }
    }

    Ok(Json(positions))
}

// ... error handling ...
#[derive(Debug)]
enum AppError {
    BoursoApiError(bourso_api::client::error::ClientError),
    Anyhow(anyhow::Error),
}

impl From<bourso_api::client::error::ClientError> for AppError {
    fn from(inner: bourso_api::client::error::ClientError) -> Self {
        AppError::BoursoApiError(inner)
    }
}

impl From<anyhow::Error> for AppError {
    fn from(inner: anyhow::Error) -> Self {
        AppError::Anyhow(inner)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(inner: std::num::ParseIntError) -> Self {
        AppError::Anyhow(inner.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::BoursoApiError(bourso_api::client::error::ClientError::MfaRequired) => {
                (StatusCode::UNAUTHORIZED, "MFA required".to_string())
            }
            AppError::BoursoApiError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("An internal server error occurred: {}", err),
            ),
            AppError::Anyhow(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("An internal server error occurred: {}", err),
            ),
        };

        (status, Json(serde_json::json!({ "error": error_message }))).into_response()
    }
}
