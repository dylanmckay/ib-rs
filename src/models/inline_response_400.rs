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
pub struct InlineResponse400 {
  #[serde(rename = "error")]
  error: Option<String>,
  #[serde(rename = "statusCode")]
  status_code: Option<i32>
}

impl InlineResponse400 {
  pub fn new() -> InlineResponse400 {
    InlineResponse400 {
      error: None,
      status_code: None
    }
  }

  pub fn set_error(&mut self, error: String) {
    self.error = Some(error);
  }

  pub fn with_error(mut self, error: String) -> InlineResponse400 {
    self.error = Some(error);
    self
  }

  pub fn error(&self) -> Option<&String> {
    self.error.as_ref()
  }

  pub fn reset_error(&mut self) {
    self.error = None;
  }

  pub fn set_status_code(&mut self, status_code: i32) {
    self.status_code = Some(status_code);
  }

  pub fn with_status_code(mut self, status_code: i32) -> InlineResponse400 {
    self.status_code = Some(status_code);
    self
  }

  pub fn status_code(&self) -> Option<&i32> {
    self.status_code.as_ref()
  }

  pub fn reset_status_code(&mut self) {
    self.status_code = None;
  }

}



