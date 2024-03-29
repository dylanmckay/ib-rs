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
pub struct InlineResponse20027 {
  #[serde(rename = "filter_list")]
  filter_list: Option<Vec<::models::InlineResponse20027FilterList>>,
  #[serde(rename = "instrument_list")]
  instrument_list: Option<Vec<::models::InlineResponse20027InstrumentList>>,
  #[serde(rename = "location_tree")]
  location_tree: Option<Vec<::models::InlineResponse20027LocationTree>>,
  #[serde(rename = "scan_type_list")]
  scan_type_list: Option<Vec<::models::InlineResponse20027ScanTypeList>>
}

impl InlineResponse20027 {
  pub fn new() -> InlineResponse20027 {
    InlineResponse20027 {
      filter_list: None,
      instrument_list: None,
      location_tree: None,
      scan_type_list: None
    }
  }

  pub fn set_filter_list(&mut self, filter_list: Vec<::models::InlineResponse20027FilterList>) {
    self.filter_list = Some(filter_list);
  }

  pub fn with_filter_list(mut self, filter_list: Vec<::models::InlineResponse20027FilterList>) -> InlineResponse20027 {
    self.filter_list = Some(filter_list);
    self
  }

  pub fn filter_list(&self) -> Option<&Vec<::models::InlineResponse20027FilterList>> {
    self.filter_list.as_ref()
  }

  pub fn reset_filter_list(&mut self) {
    self.filter_list = None;
  }

  pub fn set_instrument_list(&mut self, instrument_list: Vec<::models::InlineResponse20027InstrumentList>) {
    self.instrument_list = Some(instrument_list);
  }

  pub fn with_instrument_list(mut self, instrument_list: Vec<::models::InlineResponse20027InstrumentList>) -> InlineResponse20027 {
    self.instrument_list = Some(instrument_list);
    self
  }

  pub fn instrument_list(&self) -> Option<&Vec<::models::InlineResponse20027InstrumentList>> {
    self.instrument_list.as_ref()
  }

  pub fn reset_instrument_list(&mut self) {
    self.instrument_list = None;
  }

  pub fn set_location_tree(&mut self, location_tree: Vec<::models::InlineResponse20027LocationTree>) {
    self.location_tree = Some(location_tree);
  }

  pub fn with_location_tree(mut self, location_tree: Vec<::models::InlineResponse20027LocationTree>) -> InlineResponse20027 {
    self.location_tree = Some(location_tree);
    self
  }

  pub fn location_tree(&self) -> Option<&Vec<::models::InlineResponse20027LocationTree>> {
    self.location_tree.as_ref()
  }

  pub fn reset_location_tree(&mut self) {
    self.location_tree = None;
  }

  pub fn set_scan_type_list(&mut self, scan_type_list: Vec<::models::InlineResponse20027ScanTypeList>) {
    self.scan_type_list = Some(scan_type_list);
  }

  pub fn with_scan_type_list(mut self, scan_type_list: Vec<::models::InlineResponse20027ScanTypeList>) -> InlineResponse20027 {
    self.scan_type_list = Some(scan_type_list);
    self
  }

  pub fn scan_type_list(&self) -> Option<&Vec<::models::InlineResponse20027ScanTypeList>> {
    self.scan_type_list.as_ref()
  }

  pub fn reset_scan_type_list(&mut self) {
    self.scan_type_list = None;
  }

}



