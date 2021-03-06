/* 
 * Client Portal Web API
 *
 * Production version of the Client Portal Web API
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Account : account information

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
  #[serde(rename = "accountAlias")]
  account_alias: Option<String>,
  #[serde(rename = "accountId")]
  account_id: Option<String>,
  #[serde(rename = "accountStatus")]
  account_status: Option<f32>,
  #[serde(rename = "accountTitle")]
  account_title: Option<String>,
  #[serde(rename = "accountVan")]
  account_van: Option<String>,
  #[serde(rename = "covestor")]
  covestor: Option<bool>,
  #[serde(rename = "currency")]
  currency: Option<String>,
  #[serde(rename = "desc")]
  desc: Option<String>,
  #[serde(rename = "displayName")]
  display_name: Option<String>,
  #[serde(rename = "faclient")]
  faclient: Option<bool>,
  #[serde(rename = "id")]
  id: Option<String>,
  #[serde(rename = "master")]
  master: Option<::models::AccountMaster>,
  #[serde(rename = "parent")]
  parent: Option<String>,
  #[serde(rename = "tradingType")]
  trading_type: Option<String>,
  #[serde(rename = "type")]
  _type: Option<String>
}

impl Account {
  /// account information
  pub fn new() -> Account {
    Account {
      account_alias: None,
      account_id: None,
      account_status: None,
      account_title: None,
      account_van: None,
      covestor: None,
      currency: None,
      desc: None,
      display_name: None,
      faclient: None,
      id: None,
      master: None,
      parent: None,
      trading_type: None,
      _type: None
    }
  }

  pub fn set_account_alias(&mut self, account_alias: String) {
    self.account_alias = Some(account_alias);
  }

  pub fn with_account_alias(mut self, account_alias: String) -> Account {
    self.account_alias = Some(account_alias);
    self
  }

  pub fn account_alias(&self) -> Option<&String> {
    self.account_alias.as_ref()
  }

  pub fn reset_account_alias(&mut self) {
    self.account_alias = None;
  }

  pub fn set_account_id(&mut self, account_id: String) {
    self.account_id = Some(account_id);
  }

  pub fn with_account_id(mut self, account_id: String) -> Account {
    self.account_id = Some(account_id);
    self
  }

  pub fn account_id(&self) -> Option<&String> {
    self.account_id.as_ref()
  }

  pub fn reset_account_id(&mut self) {
    self.account_id = None;
  }

  pub fn set_account_status(&mut self, account_status: f32) {
    self.account_status = Some(account_status);
  }

  pub fn with_account_status(mut self, account_status: f32) -> Account {
    self.account_status = Some(account_status);
    self
  }

  pub fn account_status(&self) -> Option<&f32> {
    self.account_status.as_ref()
  }

  pub fn reset_account_status(&mut self) {
    self.account_status = None;
  }

  pub fn set_account_title(&mut self, account_title: String) {
    self.account_title = Some(account_title);
  }

  pub fn with_account_title(mut self, account_title: String) -> Account {
    self.account_title = Some(account_title);
    self
  }

  pub fn account_title(&self) -> Option<&String> {
    self.account_title.as_ref()
  }

  pub fn reset_account_title(&mut self) {
    self.account_title = None;
  }

  pub fn set_account_van(&mut self, account_van: String) {
    self.account_van = Some(account_van);
  }

  pub fn with_account_van(mut self, account_van: String) -> Account {
    self.account_van = Some(account_van);
    self
  }

  pub fn account_van(&self) -> Option<&String> {
    self.account_van.as_ref()
  }

  pub fn reset_account_van(&mut self) {
    self.account_van = None;
  }

  pub fn set_covestor(&mut self, covestor: bool) {
    self.covestor = Some(covestor);
  }

  pub fn with_covestor(mut self, covestor: bool) -> Account {
    self.covestor = Some(covestor);
    self
  }

  pub fn covestor(&self) -> Option<&bool> {
    self.covestor.as_ref()
  }

  pub fn reset_covestor(&mut self) {
    self.covestor = None;
  }

  pub fn set_currency(&mut self, currency: String) {
    self.currency = Some(currency);
  }

  pub fn with_currency(mut self, currency: String) -> Account {
    self.currency = Some(currency);
    self
  }

  pub fn currency(&self) -> Option<&String> {
    self.currency.as_ref()
  }

  pub fn reset_currency(&mut self) {
    self.currency = None;
  }

  pub fn set_desc(&mut self, desc: String) {
    self.desc = Some(desc);
  }

  pub fn with_desc(mut self, desc: String) -> Account {
    self.desc = Some(desc);
    self
  }

  pub fn desc(&self) -> Option<&String> {
    self.desc.as_ref()
  }

  pub fn reset_desc(&mut self) {
    self.desc = None;
  }

  pub fn set_display_name(&mut self, display_name: String) {
    self.display_name = Some(display_name);
  }

  pub fn with_display_name(mut self, display_name: String) -> Account {
    self.display_name = Some(display_name);
    self
  }

  pub fn display_name(&self) -> Option<&String> {
    self.display_name.as_ref()
  }

  pub fn reset_display_name(&mut self) {
    self.display_name = None;
  }

  pub fn set_faclient(&mut self, faclient: bool) {
    self.faclient = Some(faclient);
  }

  pub fn with_faclient(mut self, faclient: bool) -> Account {
    self.faclient = Some(faclient);
    self
  }

  pub fn faclient(&self) -> Option<&bool> {
    self.faclient.as_ref()
  }

  pub fn reset_faclient(&mut self) {
    self.faclient = None;
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> Account {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_master(&mut self, master: ::models::AccountMaster) {
    self.master = Some(master);
  }

  pub fn with_master(mut self, master: ::models::AccountMaster) -> Account {
    self.master = Some(master);
    self
  }

  pub fn master(&self) -> Option<&::models::AccountMaster> {
    self.master.as_ref()
  }

  pub fn reset_master(&mut self) {
    self.master = None;
  }

  pub fn set_parent(&mut self, parent: String) {
    self.parent = Some(parent);
  }

  pub fn with_parent(mut self, parent: String) -> Account {
    self.parent = Some(parent);
    self
  }

  pub fn parent(&self) -> Option<&String> {
    self.parent.as_ref()
  }

  pub fn reset_parent(&mut self) {
    self.parent = None;
  }

  pub fn set_trading_type(&mut self, trading_type: String) {
    self.trading_type = Some(trading_type);
  }

  pub fn with_trading_type(mut self, trading_type: String) -> Account {
    self.trading_type = Some(trading_type);
    self
  }

  pub fn trading_type(&self) -> Option<&String> {
    self.trading_type.as_ref()
  }

  pub fn reset_trading_type(&mut self) {
    self.trading_type = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> Account {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

}



