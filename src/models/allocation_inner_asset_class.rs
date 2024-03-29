/* 
 * Client Portal Web API
 *
 * Client Poral Web API
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AllocationInnerAssetClass : portfolio allocation by asset class

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AllocationInnerAssetClass {
  #[serde(rename = "long")]
  long: Option<::models::AllocationInnerAssetClassLong>,
  #[serde(rename = "short")]
  short: Option<::models::AllocationInnerAssetClassShort>
}

impl AllocationInnerAssetClass {
  /// portfolio allocation by asset class
  pub fn new() -> AllocationInnerAssetClass {
    AllocationInnerAssetClass {
      long: None,
      short: None
    }
  }

  pub fn set_long(&mut self, long: ::models::AllocationInnerAssetClassLong) {
    self.long = Some(long);
  }

  pub fn with_long(mut self, long: ::models::AllocationInnerAssetClassLong) -> AllocationInnerAssetClass {
    self.long = Some(long);
    self
  }

  pub fn long(&self) -> Option<&::models::AllocationInnerAssetClassLong> {
    self.long.as_ref()
  }

  pub fn reset_long(&mut self) {
    self.long = None;
  }

  pub fn set_short(&mut self, short: ::models::AllocationInnerAssetClassShort) {
    self.short = Some(short);
  }

  pub fn with_short(mut self, short: ::models::AllocationInnerAssetClassShort) -> AllocationInnerAssetClass {
    self.short = Some(short);
    self
  }

  pub fn short(&self) -> Option<&::models::AllocationInnerAssetClassShort> {
    self.short.as_ref()
  }

  pub fn reset_short(&mut self) {
    self.short = None;
  }

}



