/* 
 * Client Portal Web API
 *
 * Production version of the Client Portal Web API
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
  #[serde(rename = "account")]
  account: Option<String>,
  #[serde(rename = "clearing_id")]
  clearing_id: Option<String>,
  #[serde(rename = "clearing_name")]
  clearing_name: Option<String>,
  #[serde(rename = "comission")]
  comission: Option<f32>,
  #[serde(rename = "company_name")]
  company_name: Option<String>,
  #[serde(rename = "conidex")]
  conidex: Option<String>,
  #[serde(rename = "contract_description_1")]
  contract_description_1: Option<String>,
  #[serde(rename = "exchange")]
  exchange: Option<String>,
  #[serde(rename = "execution_id")]
  execution_id: Option<String>,
  #[serde(rename = "net_amount")]
  net_amount: Option<f32>,
  #[serde(rename = "order_description")]
  order_description: Option<String>,
  #[serde(rename = "position")]
  position: Option<String>,
  #[serde(rename = "price")]
  price: Option<String>,
  #[serde(rename = "sec_type")]
  sec_type: Option<String>,
  #[serde(rename = "side")]
  side: Option<String>,
  #[serde(rename = "size")]
  size: Option<String>,
  #[serde(rename = "submitter")]
  submitter: Option<String>,
  #[serde(rename = "symbol")]
  symbol: Option<String>,
  #[serde(rename = "trade_time")]
  trade_time: Option<String>,
  #[serde(rename = "trade_time_r")]
  trade_time_r: Option<f32>
}

impl Trade {
  pub fn new() -> Trade {
    Trade {
      account: None,
      clearing_id: None,
      clearing_name: None,
      comission: None,
      company_name: None,
      conidex: None,
      contract_description_1: None,
      exchange: None,
      execution_id: None,
      net_amount: None,
      order_description: None,
      position: None,
      price: None,
      sec_type: None,
      side: None,
      size: None,
      submitter: None,
      symbol: None,
      trade_time: None,
      trade_time_r: None
    }
  }

  pub fn set_account(&mut self, account: String) {
    self.account = Some(account);
  }

  pub fn with_account(mut self, account: String) -> Trade {
    self.account = Some(account);
    self
  }

  pub fn account(&self) -> Option<&String> {
    self.account.as_ref()
  }

  pub fn reset_account(&mut self) {
    self.account = None;
  }

  pub fn set_clearing_id(&mut self, clearing_id: String) {
    self.clearing_id = Some(clearing_id);
  }

  pub fn with_clearing_id(mut self, clearing_id: String) -> Trade {
    self.clearing_id = Some(clearing_id);
    self
  }

  pub fn clearing_id(&self) -> Option<&String> {
    self.clearing_id.as_ref()
  }

  pub fn reset_clearing_id(&mut self) {
    self.clearing_id = None;
  }

  pub fn set_clearing_name(&mut self, clearing_name: String) {
    self.clearing_name = Some(clearing_name);
  }

  pub fn with_clearing_name(mut self, clearing_name: String) -> Trade {
    self.clearing_name = Some(clearing_name);
    self
  }

  pub fn clearing_name(&self) -> Option<&String> {
    self.clearing_name.as_ref()
  }

  pub fn reset_clearing_name(&mut self) {
    self.clearing_name = None;
  }

  pub fn set_comission(&mut self, comission: f32) {
    self.comission = Some(comission);
  }

  pub fn with_comission(mut self, comission: f32) -> Trade {
    self.comission = Some(comission);
    self
  }

  pub fn comission(&self) -> Option<&f32> {
    self.comission.as_ref()
  }

  pub fn reset_comission(&mut self) {
    self.comission = None;
  }

  pub fn set_company_name(&mut self, company_name: String) {
    self.company_name = Some(company_name);
  }

  pub fn with_company_name(mut self, company_name: String) -> Trade {
    self.company_name = Some(company_name);
    self
  }

  pub fn company_name(&self) -> Option<&String> {
    self.company_name.as_ref()
  }

  pub fn reset_company_name(&mut self) {
    self.company_name = None;
  }

  pub fn set_conidex(&mut self, conidex: String) {
    self.conidex = Some(conidex);
  }

  pub fn with_conidex(mut self, conidex: String) -> Trade {
    self.conidex = Some(conidex);
    self
  }

  pub fn conidex(&self) -> Option<&String> {
    self.conidex.as_ref()
  }

  pub fn reset_conidex(&mut self) {
    self.conidex = None;
  }

  pub fn set_contract_description_1(&mut self, contract_description_1: String) {
    self.contract_description_1 = Some(contract_description_1);
  }

  pub fn with_contract_description_1(mut self, contract_description_1: String) -> Trade {
    self.contract_description_1 = Some(contract_description_1);
    self
  }

  pub fn contract_description_1(&self) -> Option<&String> {
    self.contract_description_1.as_ref()
  }

  pub fn reset_contract_description_1(&mut self) {
    self.contract_description_1 = None;
  }

  pub fn set_exchange(&mut self, exchange: String) {
    self.exchange = Some(exchange);
  }

  pub fn with_exchange(mut self, exchange: String) -> Trade {
    self.exchange = Some(exchange);
    self
  }

  pub fn exchange(&self) -> Option<&String> {
    self.exchange.as_ref()
  }

  pub fn reset_exchange(&mut self) {
    self.exchange = None;
  }

  pub fn set_execution_id(&mut self, execution_id: String) {
    self.execution_id = Some(execution_id);
  }

  pub fn with_execution_id(mut self, execution_id: String) -> Trade {
    self.execution_id = Some(execution_id);
    self
  }

  pub fn execution_id(&self) -> Option<&String> {
    self.execution_id.as_ref()
  }

  pub fn reset_execution_id(&mut self) {
    self.execution_id = None;
  }

  pub fn set_net_amount(&mut self, net_amount: f32) {
    self.net_amount = Some(net_amount);
  }

  pub fn with_net_amount(mut self, net_amount: f32) -> Trade {
    self.net_amount = Some(net_amount);
    self
  }

  pub fn net_amount(&self) -> Option<&f32> {
    self.net_amount.as_ref()
  }

  pub fn reset_net_amount(&mut self) {
    self.net_amount = None;
  }

  pub fn set_order_description(&mut self, order_description: String) {
    self.order_description = Some(order_description);
  }

  pub fn with_order_description(mut self, order_description: String) -> Trade {
    self.order_description = Some(order_description);
    self
  }

  pub fn order_description(&self) -> Option<&String> {
    self.order_description.as_ref()
  }

  pub fn reset_order_description(&mut self) {
    self.order_description = None;
  }

  pub fn set_position(&mut self, position: String) {
    self.position = Some(position);
  }

  pub fn with_position(mut self, position: String) -> Trade {
    self.position = Some(position);
    self
  }

  pub fn position(&self) -> Option<&String> {
    self.position.as_ref()
  }

  pub fn reset_position(&mut self) {
    self.position = None;
  }

  pub fn set_price(&mut self, price: String) {
    self.price = Some(price);
  }

  pub fn with_price(mut self, price: String) -> Trade {
    self.price = Some(price);
    self
  }

  pub fn price(&self) -> Option<&String> {
    self.price.as_ref()
  }

  pub fn reset_price(&mut self) {
    self.price = None;
  }

  pub fn set_sec_type(&mut self, sec_type: String) {
    self.sec_type = Some(sec_type);
  }

  pub fn with_sec_type(mut self, sec_type: String) -> Trade {
    self.sec_type = Some(sec_type);
    self
  }

  pub fn sec_type(&self) -> Option<&String> {
    self.sec_type.as_ref()
  }

  pub fn reset_sec_type(&mut self) {
    self.sec_type = None;
  }

  pub fn set_side(&mut self, side: String) {
    self.side = Some(side);
  }

  pub fn with_side(mut self, side: String) -> Trade {
    self.side = Some(side);
    self
  }

  pub fn side(&self) -> Option<&String> {
    self.side.as_ref()
  }

  pub fn reset_side(&mut self) {
    self.side = None;
  }

  pub fn set_size(&mut self, size: String) {
    self.size = Some(size);
  }

  pub fn with_size(mut self, size: String) -> Trade {
    self.size = Some(size);
    self
  }

  pub fn size(&self) -> Option<&String> {
    self.size.as_ref()
  }

  pub fn reset_size(&mut self) {
    self.size = None;
  }

  pub fn set_submitter(&mut self, submitter: String) {
    self.submitter = Some(submitter);
  }

  pub fn with_submitter(mut self, submitter: String) -> Trade {
    self.submitter = Some(submitter);
    self
  }

  pub fn submitter(&self) -> Option<&String> {
    self.submitter.as_ref()
  }

  pub fn reset_submitter(&mut self) {
    self.submitter = None;
  }

  pub fn set_symbol(&mut self, symbol: String) {
    self.symbol = Some(symbol);
  }

  pub fn with_symbol(mut self, symbol: String) -> Trade {
    self.symbol = Some(symbol);
    self
  }

  pub fn symbol(&self) -> Option<&String> {
    self.symbol.as_ref()
  }

  pub fn reset_symbol(&mut self) {
    self.symbol = None;
  }

  pub fn set_trade_time(&mut self, trade_time: String) {
    self.trade_time = Some(trade_time);
  }

  pub fn with_trade_time(mut self, trade_time: String) -> Trade {
    self.trade_time = Some(trade_time);
    self
  }

  pub fn trade_time(&self) -> Option<&String> {
    self.trade_time.as_ref()
  }

  pub fn reset_trade_time(&mut self) {
    self.trade_time = None;
  }

  pub fn set_trade_time_r(&mut self, trade_time_r: f32) {
    self.trade_time_r = Some(trade_time_r);
  }

  pub fn with_trade_time_r(mut self, trade_time_r: f32) -> Trade {
    self.trade_time_r = Some(trade_time_r);
    self
  }

  pub fn trade_time_r(&self) -> Option<&f32> {
    self.trade_time_r.as_ref()
  }

  pub fn reset_trade_time_r(&mut self) {
    self.trade_time_r = None;
  }

}



