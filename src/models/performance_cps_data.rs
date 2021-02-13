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
pub struct PerformanceCpsData {
  #[serde(rename = "baseCurrency")]
  base_currency: Option<String>,
  /// end date-- yyyyMMdd
  #[serde(rename = "end")]
  end: Option<String>,
  #[serde(rename = "id")]
  id: Option<String>,
  /// for example-- acctid
  #[serde(rename = "idType")]
  id_type: Option<String>,
  /// each value stands for price change percent of corresponding date in dates array
  #[serde(rename = "returns")]
  returns: Option<Vec<f32>>,
  /// start date-- yyyyMMdd
  #[serde(rename = "start")]
  start: Option<String>
}

impl PerformanceCpsData {
  pub fn new() -> PerformanceCpsData {
    PerformanceCpsData {
      base_currency: None,
      end: None,
      id: None,
      id_type: None,
      returns: None,
      start: None
    }
  }

  pub fn set_base_currency(&mut self, base_currency: String) {
    self.base_currency = Some(base_currency);
  }

  pub fn with_base_currency(mut self, base_currency: String) -> PerformanceCpsData {
    self.base_currency = Some(base_currency);
    self
  }

  pub fn base_currency(&self) -> Option<&String> {
    self.base_currency.as_ref()
  }

  pub fn reset_base_currency(&mut self) {
    self.base_currency = None;
  }

  pub fn set_end(&mut self, end: String) {
    self.end = Some(end);
  }

  pub fn with_end(mut self, end: String) -> PerformanceCpsData {
    self.end = Some(end);
    self
  }

  pub fn end(&self) -> Option<&String> {
    self.end.as_ref()
  }

  pub fn reset_end(&mut self) {
    self.end = None;
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> PerformanceCpsData {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_id_type(&mut self, id_type: String) {
    self.id_type = Some(id_type);
  }

  pub fn with_id_type(mut self, id_type: String) -> PerformanceCpsData {
    self.id_type = Some(id_type);
    self
  }

  pub fn id_type(&self) -> Option<&String> {
    self.id_type.as_ref()
  }

  pub fn reset_id_type(&mut self) {
    self.id_type = None;
  }

  pub fn set_returns(&mut self, returns: Vec<f32>) {
    self.returns = Some(returns);
  }

  pub fn with_returns(mut self, returns: Vec<f32>) -> PerformanceCpsData {
    self.returns = Some(returns);
    self
  }

  pub fn returns(&self) -> Option<&Vec<f32>> {
    self.returns.as_ref()
  }

  pub fn reset_returns(&mut self) {
    self.returns = None;
  }

  pub fn set_start(&mut self, start: String) {
    self.start = Some(start);
  }

  pub fn with_start(mut self, start: String) -> PerformanceCpsData {
    self.start = Some(start);
    self
  }

  pub fn start(&self) -> Option<&String> {
    self.start.as_ref()
  }

  pub fn reset_start(&mut self) {
    self.start = None;
  }

}



