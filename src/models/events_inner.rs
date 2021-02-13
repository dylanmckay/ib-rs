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
pub struct EventsInner {
  #[serde(rename = "conids")]
  conids: Option<Vec<String>>,
  /// will be different for different event types
  #[serde(rename = "data")]
  data: Option<Value>,
  /// for example 11662135
  #[serde(rename = "event_key")]
  event_key: Option<String>,
  #[serde(rename = "event_type")]
  event_type: Option<String>,
  /// for exmple 20180817T040000+0000
  #[serde(rename = "index_date")]
  index_date: Option<String>,
  #[serde(rename = "index_date_type")]
  index_date_type: Option<String>,
  /// for example RSE
  #[serde(rename = "source")]
  source: Option<String>,
  #[serde(rename = "status")]
  status: Option<String>,
  #[serde(rename = "tooltips")]
  tooltips: Option<Value>
}

impl EventsInner {
  pub fn new() -> EventsInner {
    EventsInner {
      conids: None,
      data: None,
      event_key: None,
      event_type: None,
      index_date: None,
      index_date_type: None,
      source: None,
      status: None,
      tooltips: None
    }
  }

  pub fn set_conids(&mut self, conids: Vec<String>) {
    self.conids = Some(conids);
  }

  pub fn with_conids(mut self, conids: Vec<String>) -> EventsInner {
    self.conids = Some(conids);
    self
  }

  pub fn conids(&self) -> Option<&Vec<String>> {
    self.conids.as_ref()
  }

  pub fn reset_conids(&mut self) {
    self.conids = None;
  }

  pub fn set_data(&mut self, data: Value) {
    self.data = Some(data);
  }

  pub fn with_data(mut self, data: Value) -> EventsInner {
    self.data = Some(data);
    self
  }

  pub fn data(&self) -> Option<&Value> {
    self.data.as_ref()
  }

  pub fn reset_data(&mut self) {
    self.data = None;
  }

  pub fn set_event_key(&mut self, event_key: String) {
    self.event_key = Some(event_key);
  }

  pub fn with_event_key(mut self, event_key: String) -> EventsInner {
    self.event_key = Some(event_key);
    self
  }

  pub fn event_key(&self) -> Option<&String> {
    self.event_key.as_ref()
  }

  pub fn reset_event_key(&mut self) {
    self.event_key = None;
  }

  pub fn set_event_type(&mut self, event_type: String) {
    self.event_type = Some(event_type);
  }

  pub fn with_event_type(mut self, event_type: String) -> EventsInner {
    self.event_type = Some(event_type);
    self
  }

  pub fn event_type(&self) -> Option<&String> {
    self.event_type.as_ref()
  }

  pub fn reset_event_type(&mut self) {
    self.event_type = None;
  }

  pub fn set_index_date(&mut self, index_date: String) {
    self.index_date = Some(index_date);
  }

  pub fn with_index_date(mut self, index_date: String) -> EventsInner {
    self.index_date = Some(index_date);
    self
  }

  pub fn index_date(&self) -> Option<&String> {
    self.index_date.as_ref()
  }

  pub fn reset_index_date(&mut self) {
    self.index_date = None;
  }

  pub fn set_index_date_type(&mut self, index_date_type: String) {
    self.index_date_type = Some(index_date_type);
  }

  pub fn with_index_date_type(mut self, index_date_type: String) -> EventsInner {
    self.index_date_type = Some(index_date_type);
    self
  }

  pub fn index_date_type(&self) -> Option<&String> {
    self.index_date_type.as_ref()
  }

  pub fn reset_index_date_type(&mut self) {
    self.index_date_type = None;
  }

  pub fn set_source(&mut self, source: String) {
    self.source = Some(source);
  }

  pub fn with_source(mut self, source: String) -> EventsInner {
    self.source = Some(source);
    self
  }

  pub fn source(&self) -> Option<&String> {
    self.source.as_ref()
  }

  pub fn reset_source(&mut self) {
    self.source = None;
  }

  pub fn set_status(&mut self, status: String) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: String) -> EventsInner {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&String> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

  pub fn set_tooltips(&mut self, tooltips: Value) {
    self.tooltips = Some(tooltips);
  }

  pub fn with_tooltips(mut self, tooltips: Value) -> EventsInner {
    self.tooltips = Some(tooltips);
    self
  }

  pub fn tooltips(&self) -> Option<&Value> {
    self.tooltips.as_ref()
  }

  pub fn reset_tooltips(&mut self) {
    self.tooltips = None;
  }

}



