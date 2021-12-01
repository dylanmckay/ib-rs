use hyper;
use serde;
use serde_json::{self, Value};

#[derive(Debug)]
pub enum Error<T> {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

use super::models::*;

mod account_api;
pub use self::account_api::{ AccountApi, AccountApiClient };
mod alert_api;
pub use self::alert_api::{ AlertApi, AlertApiClient };
mod ccp_beta_api;
pub use self::ccp_beta_api::{ CCPBetaApi, CCPBetaApiClient };
mod contract_api;
pub use self::contract_api::{ ContractApi, ContractApiClient };
mod fyi_api;
pub use self::fyi_api::{ FYIApi, FYIApiClient };
mod ib_cust_api;
pub use self::ib_cust_api::{ IBCustApi, IBCustApiClient };
mod market_data_api;
pub use self::market_data_api::{ MarketDataApi, MarketDataApiClient };
mod order_api;
pub use self::order_api::{ OrderApi, OrderApiClient };
mod pn_l_api;
pub use self::pn_l_api::{ PnLApi, PnLApiClient };
mod portfolio_api;
pub use self::portfolio_api::{ PortfolioApi, PortfolioApiClient };
mod portfolio_analyst_api;
pub use self::portfolio_analyst_api::{ PortfolioAnalystApi, PortfolioAnalystApiClient };
mod scanner_api;
pub use self::scanner_api::{ ScannerApi, ScannerApiClient };
mod session_api;
pub use self::session_api::{ SessionApi, SessionApiClient };
mod streaming_api;
pub use self::streaming_api::{ StreamingApi, StreamingApiClient };
mod trades_api;
pub use self::trades_api::{ TradesApi, TradesApiClient };

pub mod configuration;
pub mod client;
