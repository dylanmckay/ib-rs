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
pub struct InlineResponse20017 {
  #[serde(rename = "order_id")]
  order_id: Option<i32>,
  #[serde(rename = "order_status")]
  order_status: Option<String>,
  #[serde(rename = "request_id")]
  request_id: Option<i32>,
  #[serde(rename = "success")]
  success: Option<bool>,
  #[serde(rename = "text")]
  text: Option<String>,
  #[serde(rename = "warning_message")]
  warning_message: Option<String>
}

impl InlineResponse20017 {
  pub fn new() -> InlineResponse20017 {
    InlineResponse20017 {
      order_id: None,
      order_status: None,
      request_id: None,
      success: None,
      text: None,
      warning_message: None
    }
  }

  pub fn set_order_id(&mut self, order_id: i32) {
    self.order_id = Some(order_id);
  }

  pub fn with_order_id(mut self, order_id: i32) -> InlineResponse20017 {
    self.order_id = Some(order_id);
    self
  }

  pub fn order_id(&self) -> Option<&i32> {
    self.order_id.as_ref()
  }

  pub fn reset_order_id(&mut self) {
    self.order_id = None;
  }

  pub fn set_order_status(&mut self, order_status: String) {
    self.order_status = Some(order_status);
  }

  pub fn with_order_status(mut self, order_status: String) -> InlineResponse20017 {
    self.order_status = Some(order_status);
    self
  }

  pub fn order_status(&self) -> Option<&String> {
    self.order_status.as_ref()
  }

  pub fn reset_order_status(&mut self) {
    self.order_status = None;
  }

  pub fn set_request_id(&mut self, request_id: i32) {
    self.request_id = Some(request_id);
  }

  pub fn with_request_id(mut self, request_id: i32) -> InlineResponse20017 {
    self.request_id = Some(request_id);
    self
  }

  pub fn request_id(&self) -> Option<&i32> {
    self.request_id.as_ref()
  }

  pub fn reset_request_id(&mut self) {
    self.request_id = None;
  }

  pub fn set_success(&mut self, success: bool) {
    self.success = Some(success);
  }

  pub fn with_success(mut self, success: bool) -> InlineResponse20017 {
    self.success = Some(success);
    self
  }

  pub fn success(&self) -> Option<&bool> {
    self.success.as_ref()
  }

  pub fn reset_success(&mut self) {
    self.success = None;
  }

  pub fn set_text(&mut self, text: String) {
    self.text = Some(text);
  }

  pub fn with_text(mut self, text: String) -> InlineResponse20017 {
    self.text = Some(text);
    self
  }

  pub fn text(&self) -> Option<&String> {
    self.text.as_ref()
  }

  pub fn reset_text(&mut self) {
    self.text = None;
  }

  pub fn set_warning_message(&mut self, warning_message: String) {
    self.warning_message = Some(warning_message);
  }

  pub fn with_warning_message(mut self, warning_message: String) -> InlineResponse20017 {
    self.warning_message = Some(warning_message);
    self
  }

  pub fn warning_message(&self) -> Option<&String> {
    self.warning_message.as_ref()
  }

  pub fn reset_warning_message(&mut self) {
    self.warning_message = None;
  }

}



