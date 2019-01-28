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
pub struct InlineResponse2009LocationTree {
  #[serde(rename = "display_name")]
  display_name: Option<String>,
  #[serde(rename = "type")]
  _type: Option<String>,
  #[serde(rename = "locations")]
  locations: Option<Vec<::models::InlineResponse2009Locations>>
}

impl InlineResponse2009LocationTree {
  pub fn new() -> InlineResponse2009LocationTree {
    InlineResponse2009LocationTree {
      display_name: None,
      _type: None,
      locations: None
    }
  }

  pub fn set_display_name(&mut self, display_name: String) {
    self.display_name = Some(display_name);
  }

  pub fn with_display_name(mut self, display_name: String) -> InlineResponse2009LocationTree {
    self.display_name = Some(display_name);
    self
  }

  pub fn display_name(&self) -> Option<&String> {
    self.display_name.as_ref()
  }

  pub fn reset_display_name(&mut self) {
    self.display_name = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> InlineResponse2009LocationTree {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

  pub fn set_locations(&mut self, locations: Vec<::models::InlineResponse2009Locations>) {
    self.locations = Some(locations);
  }

  pub fn with_locations(mut self, locations: Vec<::models::InlineResponse2009Locations>) -> InlineResponse2009LocationTree {
    self.locations = Some(locations);
    self
  }

  pub fn locations(&self) -> Option<&Vec<::models::InlineResponse2009Locations>> {
    self.locations.as_ref()
  }

  pub fn reset_locations(&mut self) {
    self.locations = None;
  }

}



