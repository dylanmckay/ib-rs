/* 
 * Client Portal Web API
 *
 * Client Poral Web API
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ScannerparamsInstrumentList : Contains list of instruments for which scanner can be ran

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScannerparamsInstrumentList {
  /// Contains information like name, supported filters, etc. for an instrument
  #[serde(rename = "Instrument")]
  instrument: Option<Vec<::models::ScannerparamsInstrumentListInstrument>>
}

impl ScannerparamsInstrumentList {
  /// Contains list of instruments for which scanner can be ran
  pub fn new() -> ScannerparamsInstrumentList {
    ScannerparamsInstrumentList {
      instrument: None
    }
  }

  pub fn set_instrument(&mut self, instrument: Vec<::models::ScannerparamsInstrumentListInstrument>) {
    self.instrument = Some(instrument);
  }

  pub fn with_instrument(mut self, instrument: Vec<::models::ScannerparamsInstrumentListInstrument>) -> ScannerparamsInstrumentList {
    self.instrument = Some(instrument);
    self
  }

  pub fn instrument(&self) -> Option<&Vec<::models::ScannerparamsInstrumentListInstrument>> {
    self.instrument.as_ref()
  }

  pub fn reset_instrument(&mut self) {
    self.instrument = None;
  }

}


