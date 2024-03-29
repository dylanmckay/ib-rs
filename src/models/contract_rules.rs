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
pub struct ContractRules {
  /// default quantity you can use to place an order
  #[serde(rename = "defaultSize")]
  default_size: Option<f32>,
  #[serde(rename = "displaySize")]
  display_size: Option<String>,
  #[serde(rename = "increment")]
  increment: Option<String>,
  /// default limit price you can use to prefill your order
  #[serde(rename = "limitPrice")]
  limit_price: Option<f32>,
  #[serde(rename = "orderTypes")]
  order_types: Option<Vec<String>>,
  #[serde(rename = "orderTypesOutside")]
  order_types_outside: Option<Vec<String>>,
  /// if you can preview the order or not with the whatif endpoint
  #[serde(rename = "preview")]
  preview: Option<bool>,
  #[serde(rename = "sizeIncrement")]
  size_increment: Option<f32>,
  /// default stop price you can use to prefill your order
  #[serde(rename = "stopprice")]
  stopprice: Option<f32>,
  #[serde(rename = "tifTypes")]
  tif_types: Option<Vec<String>>
}

impl ContractRules {
  pub fn new() -> ContractRules {
    ContractRules {
      default_size: None,
      display_size: None,
      increment: None,
      limit_price: None,
      order_types: None,
      order_types_outside: None,
      preview: None,
      size_increment: None,
      stopprice: None,
      tif_types: None
    }
  }

  pub fn set_default_size(&mut self, default_size: f32) {
    self.default_size = Some(default_size);
  }

  pub fn with_default_size(mut self, default_size: f32) -> ContractRules {
    self.default_size = Some(default_size);
    self
  }

  pub fn default_size(&self) -> Option<&f32> {
    self.default_size.as_ref()
  }

  pub fn reset_default_size(&mut self) {
    self.default_size = None;
  }

  pub fn set_display_size(&mut self, display_size: String) {
    self.display_size = Some(display_size);
  }

  pub fn with_display_size(mut self, display_size: String) -> ContractRules {
    self.display_size = Some(display_size);
    self
  }

  pub fn display_size(&self) -> Option<&String> {
    self.display_size.as_ref()
  }

  pub fn reset_display_size(&mut self) {
    self.display_size = None;
  }

  pub fn set_increment(&mut self, increment: String) {
    self.increment = Some(increment);
  }

  pub fn with_increment(mut self, increment: String) -> ContractRules {
    self.increment = Some(increment);
    self
  }

  pub fn increment(&self) -> Option<&String> {
    self.increment.as_ref()
  }

  pub fn reset_increment(&mut self) {
    self.increment = None;
  }

  pub fn set_limit_price(&mut self, limit_price: f32) {
    self.limit_price = Some(limit_price);
  }

  pub fn with_limit_price(mut self, limit_price: f32) -> ContractRules {
    self.limit_price = Some(limit_price);
    self
  }

  pub fn limit_price(&self) -> Option<&f32> {
    self.limit_price.as_ref()
  }

  pub fn reset_limit_price(&mut self) {
    self.limit_price = None;
  }

  pub fn set_order_types(&mut self, order_types: Vec<String>) {
    self.order_types = Some(order_types);
  }

  pub fn with_order_types(mut self, order_types: Vec<String>) -> ContractRules {
    self.order_types = Some(order_types);
    self
  }

  pub fn order_types(&self) -> Option<&Vec<String>> {
    self.order_types.as_ref()
  }

  pub fn reset_order_types(&mut self) {
    self.order_types = None;
  }

  pub fn set_order_types_outside(&mut self, order_types_outside: Vec<String>) {
    self.order_types_outside = Some(order_types_outside);
  }

  pub fn with_order_types_outside(mut self, order_types_outside: Vec<String>) -> ContractRules {
    self.order_types_outside = Some(order_types_outside);
    self
  }

  pub fn order_types_outside(&self) -> Option<&Vec<String>> {
    self.order_types_outside.as_ref()
  }

  pub fn reset_order_types_outside(&mut self) {
    self.order_types_outside = None;
  }

  pub fn set_preview(&mut self, preview: bool) {
    self.preview = Some(preview);
  }

  pub fn with_preview(mut self, preview: bool) -> ContractRules {
    self.preview = Some(preview);
    self
  }

  pub fn preview(&self) -> Option<&bool> {
    self.preview.as_ref()
  }

  pub fn reset_preview(&mut self) {
    self.preview = None;
  }

  pub fn set_size_increment(&mut self, size_increment: f32) {
    self.size_increment = Some(size_increment);
  }

  pub fn with_size_increment(mut self, size_increment: f32) -> ContractRules {
    self.size_increment = Some(size_increment);
    self
  }

  pub fn size_increment(&self) -> Option<&f32> {
    self.size_increment.as_ref()
  }

  pub fn reset_size_increment(&mut self) {
    self.size_increment = None;
  }

  pub fn set_stopprice(&mut self, stopprice: f32) {
    self.stopprice = Some(stopprice);
  }

  pub fn with_stopprice(mut self, stopprice: f32) -> ContractRules {
    self.stopprice = Some(stopprice);
    self
  }

  pub fn stopprice(&self) -> Option<&f32> {
    self.stopprice.as_ref()
  }

  pub fn reset_stopprice(&mut self) {
    self.stopprice = None;
  }

  pub fn set_tif_types(&mut self, tif_types: Vec<String>) {
    self.tif_types = Some(tif_types);
  }

  pub fn with_tif_types(mut self, tif_types: Vec<String>) -> ContractRules {
    self.tif_types = Some(tif_types);
    self
  }

  pub fn tif_types(&self) -> Option<&Vec<String>> {
    self.tif_types.as_ref()
  }

  pub fn reset_tif_types(&mut self) {
    self.tif_types = None;
  }

}



