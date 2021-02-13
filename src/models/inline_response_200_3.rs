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
pub struct InlineResponse2003 {
  /// optional, if A doesn't exist, it means user can't toggle this option. 0-off, 1-on.
  #[serde(rename = "A")]
  A: Option<i32>,
  /// fyi code
  #[serde(rename = "FC")]
  FC: Option<String>,
  /// detailed description
  #[serde(rename = "FD")]
  FD: Option<String>,
  /// title
  #[serde(rename = "FN")]
  FN: Option<String>,
  /// disclaimer read, 1 = yes, = 0 no.
  #[serde(rename = "H")]
  H: Option<i32>
}

impl InlineResponse2003 {
  pub fn new() -> InlineResponse2003 {
    InlineResponse2003 {
      A: None,
      FC: None,
      FD: None,
      FN: None,
      H: None
    }
  }

  pub fn set_A(&mut self, A: i32) {
    self.A = Some(A);
  }

  pub fn with_A(mut self, A: i32) -> InlineResponse2003 {
    self.A = Some(A);
    self
  }

  pub fn A(&self) -> Option<&i32> {
    self.A.as_ref()
  }

  pub fn reset_A(&mut self) {
    self.A = None;
  }

  pub fn set_FC(&mut self, FC: String) {
    self.FC = Some(FC);
  }

  pub fn with_FC(mut self, FC: String) -> InlineResponse2003 {
    self.FC = Some(FC);
    self
  }

  pub fn FC(&self) -> Option<&String> {
    self.FC.as_ref()
  }

  pub fn reset_FC(&mut self) {
    self.FC = None;
  }

  pub fn set_FD(&mut self, FD: String) {
    self.FD = Some(FD);
  }

  pub fn with_FD(mut self, FD: String) -> InlineResponse2003 {
    self.FD = Some(FD);
    self
  }

  pub fn FD(&self) -> Option<&String> {
    self.FD.as_ref()
  }

  pub fn reset_FD(&mut self) {
    self.FD = None;
  }

  pub fn set_FN(&mut self, FN: String) {
    self.FN = Some(FN);
  }

  pub fn with_FN(mut self, FN: String) -> InlineResponse2003 {
    self.FN = Some(FN);
    self
  }

  pub fn FN(&self) -> Option<&String> {
    self.FN.as_ref()
  }

  pub fn reset_FN(&mut self) {
    self.FN = None;
  }

  pub fn set_H(&mut self, H: i32) {
    self.H = Some(H);
  }

  pub fn with_H(mut self, H: i32) -> InlineResponse2003 {
    self.H = Some(H);
    self
  }

  pub fn H(&self) -> Option<&i32> {
    self.H.as_ref()
  }

  pub fn reset_H(&mut self) {
    self.H = None;
  }

}



