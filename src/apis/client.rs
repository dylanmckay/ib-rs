use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  account_api: Box<::apis::AccountApi>,
  contract_api: Box<::apis::ContractApi>,
  fyi_api: Box<::apis::FYIApi>,
  market_data_api: Box<::apis::MarketDataApi>,
  order_api: Box<::apis::OrderApi>,
  pnl_api: Box<::apis::PnlApi>,
  portfolio_api: Box<::apis::PortfolioApi>,
  portfolio_analyst_api: Box<::apis::PortfolioAnalystApi>,
  scanner_api: Box<::apis::ScannerApi>,
  trades_api: Box<::apis::TradesApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      account_api: Box::new(::apis::AccountApiClient::new(rc.clone())),
      contract_api: Box::new(::apis::ContractApiClient::new(rc.clone())),
      fyi_api: Box::new(::apis::FYIApiClient::new(rc.clone())),
      market_data_api: Box::new(::apis::MarketDataApiClient::new(rc.clone())),
      order_api: Box::new(::apis::OrderApiClient::new(rc.clone())),
      pnl_api: Box::new(::apis::PnlApiClient::new(rc.clone())),
      portfolio_api: Box::new(::apis::PortfolioApiClient::new(rc.clone())),
      portfolio_analyst_api: Box::new(::apis::PortfolioAnalystApiClient::new(rc.clone())),
      scanner_api: Box::new(::apis::ScannerApiClient::new(rc.clone())),
      trades_api: Box::new(::apis::TradesApiClient::new(rc.clone())),
    }
  }

  pub fn account_api(&self) -> &::apis::AccountApi{
    self.account_api.as_ref()
  }

  pub fn contract_api(&self) -> &::apis::ContractApi{
    self.contract_api.as_ref()
  }

  pub fn fyi_api(&self) -> &::apis::FYIApi{
    self.fyi_api.as_ref()
  }

  pub fn market_data_api(&self) -> &::apis::MarketDataApi{
    self.market_data_api.as_ref()
  }

  pub fn order_api(&self) -> &::apis::OrderApi{
    self.order_api.as_ref()
  }

  pub fn pnl_api(&self) -> &::apis::PnlApi{
    self.pnl_api.as_ref()
  }

  pub fn portfolio_api(&self) -> &::apis::PortfolioApi{
    self.portfolio_api.as_ref()
  }

  pub fn portfolio_analyst_api(&self) -> &::apis::PortfolioAnalystApi{
    self.portfolio_analyst_api.as_ref()
  }

  pub fn scanner_api(&self) -> &::apis::ScannerApi{
    self.scanner_api.as_ref()
  }

  pub fn trades_api(&self) -> &::apis::TradesApi{
    self.trades_api.as_ref()
  }


}
