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
pub struct InlineResponse20023OrderTypes {
  /// list of available order types
  #[serde(rename = "0")]
  var_0: Option<String>
}

impl InlineResponse20023OrderTypes {
  pub fn new() -> InlineResponse20023OrderTypes {
    InlineResponse20023OrderTypes {
      var_0: None
    }
  }

  pub fn set_var_0(&mut self, var_0: String) {
    self.var_0 = Some(var_0);
  }

  pub fn with_var_0(mut self, var_0: String) -> InlineResponse20023OrderTypes {
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


