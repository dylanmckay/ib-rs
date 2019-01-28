use hyper;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

use super::models::*;

mod account_api;
pub use self::account_api::{ AccountApi, AccountApiClient };
mod contract_api;
pub use self::contract_api::{ ContractApi, ContractApiClient };
mod fyi_api;
pub use self::fyi_api::{ FYIApi, FYIApiClient };
mod market_data_api;
pub use self::market_data_api::{ MarketDataApi, MarketDataApiClient };
mod order_api;
pub use self::order_api::{ OrderApi, OrderApiClient };
mod pnl_api;
pub use self::pnl_api::{ PnlApi, PnlApiClient };
mod portfolio_api;
pub use self::portfolio_api::{ PortfolioApi, PortfolioApiClient };
mod portfolio_analyst_api;
pub use self::portfolio_analyst_api::{ PortfolioAnalystApi, PortfolioAnalystApiClient };
mod scanner_api;
pub use self::scanner_api::{ ScannerApi, ScannerApiClient };
mod trades_api;
pub use self::trades_api::{ TradesApi, TradesApiClient };

pub mod configuration;
pub mod client;
