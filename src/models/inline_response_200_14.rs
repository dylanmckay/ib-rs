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
pub struct InlineResponse20014 {
  #[serde(rename = "filter_list")]
  filter_list: Option<Vec<::models::InlineResponse20014FilterList>>,
  #[serde(rename = "instrument_list")]
  instrument_list: Option<Vec<::models::InlineResponse20014InstrumentList>>,
  #[serde(rename = "location_tree")]
  location_tree: Option<Vec<::models::InlineResponse20014LocationTree>>,
  #[serde(rename = "scan_type_list")]
  scan_type_list: Option<Vec<::models::InlineResponse20014ScanTypeList>>
}

impl InlineResponse20014 {
  pub fn new() -> InlineResponse20014 {
    InlineResponse20014 {
      filter_list: None,
      instrument_list: None,
      location_tree: None,
      scan_type_list: None
    }
  }

  pub fn set_filter_list(&mut self, filter_list: Vec<::models::InlineResponse20014FilterList>) {
    self.filter_list = Some(filter_list);
  }

  pub fn with_filter_list(mut self, filter_list: Vec<::models::InlineResponse20014FilterList>) -> InlineResponse20014 {
    self.filter_list = Some(filter_list);
    self
  }

  pub fn filter_list(&self) -> Option<&Vec<::models::InlineResponse20014FilterList>> {
    self.filter_list.as_ref()
  }

  pub fn reset_filter_list(&mut self) {
    self.filter_list = None;
  }

  pub fn set_instrument_list(&mut self, instrument_list: Vec<::models::InlineResponse20014InstrumentList>) {
    self.instrument_list = Some(instrument_list);
  }

  pub fn with_instrument_list(mut self, instrument_list: Vec<::models::InlineResponse20014InstrumentList>) -> InlineResponse20014 {
    self.instrument_list = Some(instrument_list);
    self
  }

  pub fn instrument_list(&self) -> Option<&Vec<::models::InlineResponse20014InstrumentList>> {
    self.instrument_list.as_ref()
  }

  pub fn reset_instrument_list(&mut self) {
    self.instrument_list = None;
  }

  pub fn set_location_tree(&mut self, location_tree: Vec<::models::InlineResponse20014LocationTree>) {
    self.location_tree = Some(location_tree);
  }

  pub fn with_location_tree(mut self, location_tree: Vec<::models::InlineResponse20014LocationTree>) -> InlineResponse20014 {
    self.location_tree = Some(location_tree);
    self
  }

  pub fn location_tree(&self) -> Option<&Vec<::models::InlineResponse20014LocationTree>> {
    self.location_tree.as_ref()
  }

  pub fn reset_location_tree(&mut self) {
    self.location_tree = None;
  }

  pub fn set_scan_type_list(&mut self, scan_type_list: Vec<::models::InlineResponse20014ScanTypeList>) {
    self.scan_type_list = Some(scan_type_list);
  }

  pub fn with_scan_type_list(mut self, scan_type_list: Vec<::models::InlineResponse20014ScanTypeList>) -> InlineResponse20014 {
    self.scan_type_list = Some(scan_type_list);
    self
  }

  pub fn scan_type_list(&self) -> Option<&Vec<::models::InlineResponse20014ScanTypeList>> {
    self.scan_type_list.as_ref()
  }

  pub fn reset_scan_type_list(&mut self) {
    self.scan_type_list = None;
  }

}



