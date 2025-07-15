# Bourso API

[![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/azerpas/bourso-api/badge)](https://securityscorecards.dev/viewer/?uri=github.com/azerpas/bourso-api)
[![codecov](https://codecov.io/gh/azerpas/bourso-api/graph/badge.svg?token=I47J55VCB3)](https://codecov.io/gh/azerpas/bourso-api)

<img width="1264" alt="Screenshot of Bourso API" src="https://github.com/azerpas/bourso-api/assets/19282069/9ddbc5aa-7e52-4ab3-8a86-b15bd2328b67">


This app aims to be a simple API powered by *[Bourso API](./src/bourso_api/)* to log in to your [BoursoBank/Boursorama](https://www.boursorama.com) account and achieve some basic tasks.

The first goal of this project was creating an automated [DCA (Dollar Cost Average)](https://www.investopedia.com/terms/d/dollarcostaveraging.asp) solution to buy [ETFs (Exchange Traded Funds)](https://www.investopedia.com/terms/e/etf.asp) on a regular basis with your Bourso account.

- [Installation](#installation)
  - [From source](#from-source)
- [Usage](#usage)
  - [Get your accounts](#get-your-accounts)
  - [Place an order](#place-an-order)
  - [Quote](#quote)
- [Security](#security)
- [Disclaimer](#disclaimer)

## Installation
### From source
Requires [>=Rust 1.77.2](https://www.rust-lang.org)
```sh
git clone git@github.com:azerpas/bourso-api.git
cd bourso-api
docker-compose up -d --build
```

## Usage

The API will be available at `http://localhost:80`.

### Get your accounts
```bash
curl -X POST http://localhost/accounts \
-H "Content-Type: application/json" \
-d '{
    "username": "your_username",
    "password": "your_password"
}'
```

### Place an order
```bash
curl -X POST http://localhost/trade/orders \
-H "Content-Type: application/json" \
-d '{
    "credentials": {
        "username": "your_username",
        "password": "your_password"
    },
    "account_id": "your_account_id",
    "side": "buy",
    "symbol": "1rTCW8",
    "quantity": 1
}'
```

### Quote
```bash
curl -X POST http://localhost/quotes \
-H "Content-Type: application/json" \
-d '{
    "symbol": "1rTCW8"
}'
```

## Security
This app runs locally. All outbound/inbound data is sent/received to/from BoursoBank servers **only**. Your password will not be saved locally and will be asked each time you run the app.

## Disclaimer

This script is provided as is, without any warranty. I am not responsible for any loss of funds. Use at your own risk. I am not affiliated with BoursoBank or any other project mentioned in this repository. This is not financial advice.
