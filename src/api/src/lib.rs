use axum::{
    extract::State,
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
        .route("/quotes", post(get_quotes))
        .route("/trade/orders", post(create_order))
        .route("/trade/positions", post(get_positions))
}

async fn get_accounts(
    Json(credentials): Json<Credentials>,
) -> Result<Json<Vec<Account>>, AppError> {
    let mut web_client: BoursoWebClient = get_client();
    web_client.init_session().await?;
    web_client
        .login(&credentials.username, &credentials.password)
        .await?;

    let accounts = web_client.get_accounts(None).await?;
    Ok(Json(accounts))
}

async fn get_quotes(
    Json(quote_request): Json<QuoteRequest>,
) -> Result<Json<QuoteTab>, AppError> {
    let web_client: BoursoWebClient = get_client();

    let quotes = web_client
        .get_ticks(
            &quote_request.symbol,
            quote_request.length.unwrap_or("30".to_string()).parse()?,
            quote_request
                .interval
                .unwrap_or("0".to_string())
                .parse()?,
        )
        .await?;

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
