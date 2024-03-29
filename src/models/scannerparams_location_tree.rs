/* 
 * Client Portal Web API
 *
 * Client Poral Web API
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ScannerparamsLocationTree : Contains list of instruments for which scanner can be ran

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScannerparamsLocationTree {
  /// Contains information like name, supported filters, etc. for an instrument. A location can contain more locations forming a tree-like structure which allows user to control the lcoation at more granular level. locationCode has to be used to specify lcoations while querying a scanner.
  #[serde(rename = "Location")]
  location: Option<Vec<::models::ScannerparamsLocationTreeLocation>>
}

impl ScannerparamsLocationTree {
  /// Contains list of instruments for which scanner can be ran
  pub fn new() -> ScannerparamsLocationTree {
    ScannerparamsLocationTree {
      location: None
    }
  }

  pub fn set_location(&mut self, location: Vec<::models::ScannerparamsLocationTreeLocation>) {
    self.location = Some(location);
  }

  pub fn with_location(mut self, location: Vec<::models::ScannerparamsLocationTreeLocation>) -> ScannerparamsLocationTree {
    self.location = Some(location);
    self
  }

  pub fn location(&self) -> Option<&Vec<::models::ScannerparamsLocationTreeLocation>> {
    self.location.as_ref()
  }

  pub fn reset_location(&mut self) {
    self.location = None;
  }

}



