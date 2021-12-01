/* 
 * Client Portal Web API
 *
 * Client Poral Web API
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Body9 {
  #[serde(rename = "acctIds")]
  acct_ids: Option<Vec<String>>,
  #[serde(rename = "conids")]
  conids: Option<Vec<f32>>,
  /// optional defaults to USD.
  #[serde(rename = "currency")]
  currency: Option<String>,
  /// optional, default value is 90
  #[serde(rename = "days")]
  days: Option<f32>
}

impl Body9 {
  pub fn new() -> Body9 {
    Body9 {
      acct_ids: None,
      conids: None,
      currency: None,
      days: None
    }
  }

  pub fn set_acct_ids(&mut self, acct_ids: Vec<String>) {
    self.acct_ids = Some(acct_ids);
  }

  pub fn with_acct_ids(mut self, acct_ids: Vec<String>) -> Body9 {
    self.acct_ids = Some(acct_ids);
    self
  }

  pub fn acct_ids(&self) -> Option<&Vec<String>> {
    self.acct_ids.as_ref()
  }

  pub fn reset_acct_ids(&mut self) {
    self.acct_ids = None;
  }

  pub fn set_conids(&mut self, conids: Vec<f32>) {
    self.conids = Some(conids);
  }

  pub fn with_conids(mut self, conids: Vec<f32>) -> Body9 {
    self.conids = Some(conids);
    self
  }

  pub fn conids(&self) -> Option<&Vec<f32>> {
    self.conids.as_ref()
  }

  pub fn reset_conids(&mut self) {
    self.conids = None;
  }

  pub fn set_currency(&mut self, currency: String) {
    self.currency = Some(currency);
  }

  pub fn with_currency(mut self, currency: String) -> Body9 {
    self.currency = Some(currency);
    self
  }

  pub fn currency(&self) -> Option<&String> {
    self.currency.as_ref()
  }

  pub fn reset_currency(&mut self) {
    self.currency = None;
  }

  pub fn set_days(&mut self, days: f32) {
    self.days = Some(days);
  }

  pub fn with_days(mut self, days: f32) -> Body9 {
    self.days = Some(days);
    self
  }

  pub fn days(&self) -> Option<&f32> {
    self.days.as_ref()
  }

  pub fn reset_days(&mut self) {
    self.days = None;
  }

}


