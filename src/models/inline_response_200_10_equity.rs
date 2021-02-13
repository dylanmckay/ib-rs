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
pub struct InlineResponse20010Equity {
  #[serde(rename = "after")]
  after: Option<String>,
  #[serde(rename = "change")]
  change: Option<String>,
  #[serde(rename = "current")]
  current: Option<String>
}

impl InlineResponse20010Equity {
  pub fn new() -> InlineResponse20010Equity {
    InlineResponse20010Equity {
      after: None,
      change: None,
      current: None
    }
  }

  pub fn set_after(&mut self, after: String) {
    self.after = Some(after);
  }

  pub fn with_after(mut self, after: String) -> InlineResponse20010Equity {
    self.after = Some(after);
    self
  }

  pub fn after(&self) -> Option<&String> {
    self.after.as_ref()
  }

  pub fn reset_after(&mut self) {
    self.after = None;
  }

  pub fn set_change(&mut self, change: String) {
    self.change = Some(change);
  }

  pub fn with_change(mut self, change: String) -> InlineResponse20010Equity {
    self.change = Some(change);
    self
  }

  pub fn change(&self) -> Option<&String> {
    self.change.as_ref()
  }

  pub fn reset_change(&mut self) {
    self.change = None;
  }

  pub fn set_current(&mut self, current: String) {
    self.current = Some(current);
  }

  pub fn with_current(mut self, current: String) -> InlineResponse20010Equity {
    self.current = Some(current);
    self
  }

  pub fn current(&self) -> Option<&String> {
    self.current.as_ref()
  }

  pub fn reset_current(&mut self) {
    self.current = None;
  }

}


