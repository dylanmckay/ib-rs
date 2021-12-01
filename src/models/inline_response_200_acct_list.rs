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
pub struct InlineResponse200AcctList {
  /// For multi-account structures each trading account will numbered from 0 to ...
  #[serde(rename = "0")]
  var_0: Option<String>
}

impl InlineResponse200AcctList {
  pub fn new() -> InlineResponse200AcctList {
    InlineResponse200AcctList {
      var_0: None
    }
  }

  pub fn set_var_0(&mut self, var_0: String) {
    self.var_0 = Some(var_0);
  }

  pub fn with_var_0(mut self, var_0: String) -> InlineResponse200AcctList {
    self.var_0 = Some(var_0);
    self
  }

  pub fn var_0(&self) -> Option<&String> {
    self.var_0.as_ref()
  }

  pub fn reset_var_0(&mut self) {
    self.var_0 = None;
  }

}


