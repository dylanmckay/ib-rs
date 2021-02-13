/* 
 * Client Portal Web API
 *
 * Production version of the Client Portal Web API
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AllocationInnerAssetClassLong : long positions allocation

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AllocationInnerAssetClassLong {
  #[serde(rename = "BOND")]
  BOND: Option<f32>,
  #[serde(rename = "CASH")]
  CASH: Option<f32>,
  #[serde(rename = "FUT")]
  FUT: Option<f32>,
  #[serde(rename = "OPT")]
  OPT: Option<f32>,
  #[serde(rename = "STK")]
  STK: Option<f32>,
  #[serde(rename = "WAR")]
  WAR: Option<f32>
}

impl AllocationInnerAssetClassLong {
  /// long positions allocation
  pub fn new() -> AllocationInnerAssetClassLong {
    AllocationInnerAssetClassLong {
      BOND: None,
      CASH: None,
      FUT: None,
      OPT: None,
      STK: None,
      WAR: None
    }
  }

  pub fn set_BOND(&mut self, BOND: f32) {
    self.BOND = Some(BOND);
  }

  pub fn with_BOND(mut self, BOND: f32) -> AllocationInnerAssetClassLong {
    self.BOND = Some(BOND);
    self
  }

  pub fn BOND(&self) -> Option<&f32> {
    self.BOND.as_ref()
  }

  pub fn reset_BOND(&mut self) {
    self.BOND = None;
  }

  pub fn set_CASH(&mut self, CASH: f32) {
    self.CASH = Some(CASH);
  }

  pub fn with_CASH(mut self, CASH: f32) -> AllocationInnerAssetClassLong {
    self.CASH = Some(CASH);
    self
  }

  pub fn CASH(&self) -> Option<&f32> {
    self.CASH.as_ref()
  }

  pub fn reset_CASH(&mut self) {
    self.CASH = None;
  }

  pub fn set_FUT(&mut self, FUT: f32) {
    self.FUT = Some(FUT);
  }

  pub fn with_FUT(mut self, FUT: f32) -> AllocationInnerAssetClassLong {
    self.FUT = Some(FUT);
    self
  }

  pub fn FUT(&self) -> Option<&f32> {
    self.FUT.as_ref()
  }

  pub fn reset_FUT(&mut self) {
    self.FUT = None;
  }

  pub fn set_OPT(&mut self, OPT: f32) {
    self.OPT = Some(OPT);
  }

  pub fn with_OPT(mut self, OPT: f32) -> AllocationInnerAssetClassLong {
    self.OPT = Some(OPT);
    self
  }

  pub fn OPT(&self) -> Option<&f32> {
    self.OPT.as_ref()
  }

  pub fn reset_OPT(&mut self) {
    self.OPT = None;
  }

  pub fn set_STK(&mut self, STK: f32) {
    self.STK = Some(STK);
  }

  pub fn with_STK(mut self, STK: f32) -> AllocationInnerAssetClassLong {
    self.STK = Some(STK);
    self
  }

  pub fn STK(&self) -> Option<&f32> {
    self.STK.as_ref()
  }

  pub fn reset_STK(&mut self) {
    self.STK = None;
  }

  pub fn set_WAR(&mut self, WAR: f32) {
    self.WAR = Some(WAR);
  }

  pub fn with_WAR(mut self, WAR: f32) -> AllocationInnerAssetClassLong {
    self.WAR = Some(WAR);
    self
  }

  pub fn WAR(&self) -> Option<&f32> {
    self.WAR.as_ref()
  }

  pub fn reset_WAR(&mut self) {
    self.WAR = None;
  }

}



