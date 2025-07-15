# BoursoBank API

This app aims to be a simple API powered by *[Bourso API](./src/bourso_api/)* to log in to your [BoursoBank](https://www.boursobank.com) account and achieve some basic tasks.

- [Installation](#installation)
  - [From source](#from-source)
- [Usage](#usage)
- [Security](#security)
- [Disclaimer](#disclaimer)

## Installation
### From source
Requires [>=Rust 1.77.2](https://www.rust-lang.org)
```sh
git clone git@github.com:theo-bnts/api.git
cd api
docker compose up -d --build
```

## Usage

The API will be available at `http://localhost:80`.

### Authentication

All routes that require authentication need a `Credentials` object in the request body.

```json
{
    "username": "your_username",
    "password": "your_password"
}
```

### Accounts

*   `POST /accounts`: Get all accounts.
*   `POST /accounts/banking`: Get banking accounts.
*   `POST /accounts/savings`: Get savings accounts.
*   `POST /accounts/trading`: Get trading accounts.
*   `POST /accounts/loans`: Get loans accounts.

### Quotes

*   `POST /quotes`: Get quotes for a symbol.
    *   Body: `{"symbol": "1rTCW8", "length": "30", "interval": "0"}`
*   `GET /quotes/{symbol}`: Get quotes for a symbol.
*   `GET /quotes/{symbol}/highest`: Get the highest quote for a symbol.
*   `GET /quotes/{symbol}/lowest`: Get the lowest quote for a symbol.
*   `GET /quotes/{symbol}/average`: Get the average quote for a symbol.
*   `GET /quotes/{symbol}/volume`: Get the volume for a symbol.
*   `GET /quotes/{symbol}/last`: Get the last quote for a symbol.

### Trade

*   `POST /trade/orders`: Place a new order.
    *   Body: `{"credentials": {"username": "your_username", "password": "your_password"}, "account_id": "your_account_id", "side": "buy", "symbol": "1rTCW8", "quantity": 1}`
*   `POST /trade/positions`: Get all positions for a trading account.
    *   Body: `{"credentials": {"username": "your_username", "password": "your_password"}, "account_id": "your_account_id"}`


## Security
This app runs locally. All outbound/inbound data is sent/received to/from BoursoBank servers **only**. Your password will not be saved locally and will be asked each time you run the app.

## Disclaimer

This script is provided as is, without any warranty. I am not responsible for any loss of funds. Use at your own risk. I am not affiliated with BoursoBank or any other project mentioned in this repository. This is not financial advice.
