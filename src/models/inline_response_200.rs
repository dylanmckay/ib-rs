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
pub struct InlineResponse200 {
  #[serde(rename = "E")]
  E: Option<Vec<::models::InlineResponse200E>>,
  /// Email option is enabled or not 0-off, 1-on.
  #[serde(rename = "M")]
  M: Option<i32>
}

impl InlineResponse200 {
  pub fn new() -> InlineResponse200 {
    InlineResponse200 {
      E: None,
      M: None
    }
  }

  pub fn set_E(&mut self, E: Vec<::models::InlineResponse200E>) {
    self.E = Some(E);
  }

  pub fn with_E(mut self, E: Vec<::models::InlineResponse200E>) -> InlineResponse200 {
    self.E = Some(E);
    self
  }

  pub fn E(&self) -> Option<&Vec<::models::InlineResponse200E>> {
    self.E.as_ref()
  }

  pub fn reset_E(&mut self) {
    self.E = None;
  }

  pub fn set_M(&mut self, M: i32) {
    self.M = Some(M);
  }

  pub fn with_M(mut self, M: i32) -> InlineResponse200 {
    self.M = Some(M);
    self
  }

  pub fn M(&self) -> Option<&i32> {
    self.M.as_ref()
  }

  pub fn reset_M(&mut self) {
    self.M = None;
  }

}



