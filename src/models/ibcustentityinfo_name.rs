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
pub struct IbcustentityinfoName {
  #[serde(rename = "firstName")]
  first_name: Option<String>,
  #[serde(rename = "lastName")]
  last_name: Option<String>,
  #[serde(rename = "salutation")]
  salutation: Option<String>
}

impl IbcustentityinfoName {
  pub fn new() -> IbcustentityinfoName {
    IbcustentityinfoName {
      first_name: None,
      last_name: None,
      salutation: None
    }
  }

  pub fn set_first_name(&mut self, first_name: String) {
    self.first_name = Some(first_name);
  }

  pub fn with_first_name(mut self, first_name: String) -> IbcustentityinfoName {
    self.first_name = Some(first_name);
    self
  }

  pub fn first_name(&self) -> Option<&String> {
    self.first_name.as_ref()
  }

  pub fn reset_first_name(&mut self) {
    self.first_name = None;
  }

  pub fn set_last_name(&mut self, last_name: String) {
    self.last_name = Some(last_name);
  }

  pub fn with_last_name(mut self, last_name: String) -> IbcustentityinfoName {
    self.last_name = Some(last_name);
    self
  }

  pub fn last_name(&self) -> Option<&String> {
    self.last_name.as_ref()
  }

  pub fn reset_last_name(&mut self) {
    self.last_name = None;
  }

  pub fn set_salutation(&mut self, salutation: String) {
    self.salutation = Some(salutation);
  }

  pub fn with_salutation(mut self, salutation: String) -> IbcustentityinfoName {
    self.salutation = Some(salutation);
    self
  }

  pub fn salutation(&self) -> Option<&String> {
    self.salutation.as_ref()
  }

  pub fn reset_salutation(&mut self) {
    self.salutation = None;
  }

}



