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
pub struct InlineResponse20018 {
  #[serde(rename = "ACCTID")]
  ACCTID: Option<::models::Position>
}

impl InlineResponse20018 {
  pub fn new() -> InlineResponse20018 {
    InlineResponse20018 {
      ACCTID: None
    }
  }

  pub fn set_ACCTID(&mut self, ACCTID: ::models::Position) {
    self.ACCTID = Some(ACCTID);
  }

  pub fn with_ACCTID(mut self, ACCTID: ::models::Position) -> InlineResponse20018 {
    self.ACCTID = Some(ACCTID);
    self
  }

  pub fn ACCTID(&self) -> Option<&::models::Position> {
    self.ACCTID.as_ref()
  }

  pub fn reset_ACCTID(&mut self) {
    self.ACCTID = None;
  }

}



