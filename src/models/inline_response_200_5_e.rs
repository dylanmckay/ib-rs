/* 
 * Client Portal Web API
 *
 * Client Poral Web API
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// InlineResponse2005E : device

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineResponse2005E {
  /// device is enabled or not 0-true, 1-false.
  #[serde(rename = "A")]
  A: Option<String>,
  /// device id
  #[serde(rename = "I")]
  I: Option<String>,
  /// device name
  #[serde(rename = "NM")]
  NM: Option<String>,
  /// unique device id
  #[serde(rename = "UI")]
  UI: Option<String>
}

impl InlineResponse2005E {
  /// device
  pub fn new() -> InlineResponse2005E {
    InlineResponse2005E {
      A: None,
      I: None,
      NM: None,
      UI: None
    }
  }

  pub fn set_A(&mut self, A: String) {
    self.A = Some(A);
  }

  pub fn with_A(mut self, A: String) -> InlineResponse2005E {
    self.A = Some(A);
    self
  }

  pub fn A(&self) -> Option<&String> {
    self.A.as_ref()
  }

  pub fn reset_A(&mut self) {
    self.A = None;
  }

  pub fn set_I(&mut self, I: String) {
    self.I = Some(I);
  }

  pub fn with_I(mut self, I: String) -> InlineResponse2005E {
    self.I = Some(I);
    self
  }

  pub fn I(&self) -> Option<&String> {
    self.I.as_ref()
  }

  pub fn reset_I(&mut self) {
    self.I = None;
  }

  pub fn set_NM(&mut self, NM: String) {
    self.NM = Some(NM);
  }

  pub fn with_NM(mut self, NM: String) -> InlineResponse2005E {
    self.NM = Some(NM);
    self
  }

  pub fn NM(&self) -> Option<&String> {
    self.NM.as_ref()
  }

  pub fn reset_NM(&mut self) {
    self.NM = None;
  }

  pub fn set_UI(&mut self, UI: String) {
    self.UI = Some(UI);
  }

  pub fn with_UI(mut self, UI: String) -> InlineResponse2005E {
    self.UI = Some(UI);
    self
  }

  pub fn UI(&self) -> Option<&String> {
    self.UI.as_ref()
  }

  pub fn reset_UI(&mut self) {
    self.UI = None;
  }

}



