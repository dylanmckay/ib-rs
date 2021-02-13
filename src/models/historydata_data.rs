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
pub struct HistorydataData {
  /// close price
  #[serde(rename = "c")]
  c: Option<f32>,
  /// high price
  #[serde(rename = "h")]
  h: Option<f32>,
  /// low price
  #[serde(rename = "l")]
  l: Option<f32>,
  /// open price
  #[serde(rename = "o")]
  o: Option<f32>,
  /// unix time stamp
  #[serde(rename = "t")]
  t: Option<f32>,
  /// volume
  #[serde(rename = "v")]
  v: Option<f32>
}

impl HistorydataData {
  pub fn new() -> HistorydataData {
    HistorydataData {
      c: None,
      h: None,
      l: None,
      o: None,
      t: None,
      v: None
    }
  }

  pub fn set_c(&mut self, c: f32) {
    self.c = Some(c);
  }

  pub fn with_c(mut self, c: f32) -> HistorydataData {
    self.c = Some(c);
    self
  }

  pub fn c(&self) -> Option<&f32> {
    self.c.as_ref()
  }

  pub fn reset_c(&mut self) {
    self.c = None;
  }

  pub fn set_h(&mut self, h: f32) {
    self.h = Some(h);
  }

  pub fn with_h(mut self, h: f32) -> HistorydataData {
    self.h = Some(h);
    self
  }

  pub fn h(&self) -> Option<&f32> {
    self.h.as_ref()
  }

  pub fn reset_h(&mut self) {
    self.h = None;
  }

  pub fn set_l(&mut self, l: f32) {
    self.l = Some(l);
  }

  pub fn with_l(mut self, l: f32) -> HistorydataData {
    self.l = Some(l);
    self
  }

  pub fn l(&self) -> Option<&f32> {
    self.l.as_ref()
  }

  pub fn reset_l(&mut self) {
    self.l = None;
  }

  pub fn set_o(&mut self, o: f32) {
    self.o = Some(o);
  }

  pub fn with_o(mut self, o: f32) -> HistorydataData {
    self.o = Some(o);
    self
  }

  pub fn o(&self) -> Option<&f32> {
    self.o.as_ref()
  }

  pub fn reset_o(&mut self) {
    self.o = None;
  }

  pub fn set_t(&mut self, t: f32) {
    self.t = Some(t);
  }

  pub fn with_t(mut self, t: f32) -> HistorydataData {
    self.t = Some(t);
    self
  }

  pub fn t(&self) -> Option<&f32> {
    self.t.as_ref()
  }

  pub fn reset_t(&mut self) {
    self.t = None;
  }

  pub fn set_v(&mut self, v: f32) {
    self.v = Some(v);
  }

  pub fn with_v(mut self, v: f32) -> HistorydataData {
    self.v = Some(v);
    self
  }

  pub fn v(&self) -> Option<&f32> {
    self.v.as_ref()
  }

  pub fn reset_v(&mut self) {
    self.v = None;
  }

}



