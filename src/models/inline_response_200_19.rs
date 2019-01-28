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
pub struct InlineResponse20019 {
  #[serde(rename = "T")]
  T: Option<i32>,
  #[serde(rename = "V")]
  V: Option<i32>
}

impl InlineResponse20019 {
  pub fn new() -> InlineResponse20019 {
    InlineResponse20019 {
      T: None,
      V: None
    }
  }

  pub fn set_T(&mut self, T: i32) {
    self.T = Some(T);
  }

  pub fn with_T(mut self, T: i32) -> InlineResponse20019 {
    self.T = Some(T);
    self
  }

  pub fn T(&self) -> Option<&i32> {
    self.T.as_ref()
  }

  pub fn reset_T(&mut self) {
    self.T = None;
  }

  pub fn set_V(&mut self, V: i32) {
    self.V = Some(V);
  }

  pub fn with_V(mut self, V: i32) -> InlineResponse20019 {
    self.V = Some(V);
    self
  }

  pub fn V(&self) -> Option<&i32> {
    self.V.as_ref()
  }

  pub fn reset_V(&mut self) {
    self.V = None;
  }

}



