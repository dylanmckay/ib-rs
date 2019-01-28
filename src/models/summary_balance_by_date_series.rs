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
pub struct SummaryBalanceByDateSeries {
  #[serde(rename = "id")]
  id: Option<String>,
  #[serde(rename = "groupId")]
  group_id: Option<String>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "date")]
  date: Option<Vec<Vec<f32>>>
}

impl SummaryBalanceByDateSeries {
  pub fn new() -> SummaryBalanceByDateSeries {
    SummaryBalanceByDateSeries {
      id: None,
      group_id: None,
      name: None,
      date: None
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> SummaryBalanceByDateSeries {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_group_id(&mut self, group_id: String) {
    self.group_id = Some(group_id);
  }

  pub fn with_group_id(mut self, group_id: String) -> SummaryBalanceByDateSeries {
    self.group_id = Some(group_id);
    self
  }

  pub fn group_id(&self) -> Option<&String> {
    self.group_id.as_ref()
  }

  pub fn reset_group_id(&mut self) {
    self.group_id = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> SummaryBalanceByDateSeries {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_date(&mut self, date: Vec<Vec<f32>>) {
    self.date = Some(date);
  }

  pub fn with_date(mut self, date: Vec<Vec<f32>>) -> SummaryBalanceByDateSeries {
    self.date = Some(date);
    self
  }

  pub fn date(&self) -> Option<&Vec<Vec<f32>>> {
    self.date.as_ref()
  }

  pub fn reset_date(&mut self) {
    self.date = None;
  }

}



