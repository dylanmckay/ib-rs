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
pub struct OrderdataWarnings {
  #[serde(rename = "PRICECAP")]
  PRICECAP: Option<String>,
  #[serde(rename = "TIME")]
  TIME: Option<String>
}

impl OrderdataWarnings {
  pub fn new() -> OrderdataWarnings {
    OrderdataWarnings {
      PRICECAP: None,
      TIME: None
    }
  }

  pub fn set_PRICECAP(&mut self, PRICECAP: String) {
    self.PRICECAP = Some(PRICECAP);
  }

  pub fn with_PRICECAP(mut self, PRICECAP: String) -> OrderdataWarnings {
    self.PRICECAP = Some(PRICECAP);
    self
  }

  pub fn PRICECAP(&self) -> Option<&String> {
    self.PRICECAP.as_ref()
  }

  pub fn reset_PRICECAP(&mut self) {
    self.PRICECAP = None;
  }

  pub fn set_TIME(&mut self, TIME: String) {
    self.TIME = Some(TIME);
  }

  pub fn with_TIME(mut self, TIME: String) -> OrderdataWarnings {
    self.TIME = Some(TIME);
    self
  }

  pub fn TIME(&self) -> Option<&String> {
    self.TIME.as_ref()
  }

  pub fn reset_TIME(&mut self) {
    self.TIME = None;
  }

}


