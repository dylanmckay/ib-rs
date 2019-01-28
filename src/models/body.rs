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
pub struct Body {
  /// answer to question, true means yes, false means no
  #[serde(rename = "confirmed")]
  confirmed: Option<bool>
}

impl Body {
  pub fn new() -> Body {
    Body {
      confirmed: None
    }
  }

  pub fn set_confirmed(&mut self, confirmed: bool) {
    self.confirmed = Some(confirmed);
  }

  pub fn with_confirmed(mut self, confirmed: bool) -> Body {
    self.confirmed = Some(confirmed);
    self
  }

  pub fn confirmed(&self) -> Option<&bool> {
    self.confirmed.as_ref()
  }

  pub fn reset_confirmed(&mut self) {
    self.confirmed = None;
  }

}



