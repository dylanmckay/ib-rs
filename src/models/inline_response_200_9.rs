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
pub struct InlineResponse2009 {
  #[serde(rename = "id")]
  id: Option<String>,
  /// Please note here, if the message is a question, you have to reply to question in order to submit the order successfully. See more in the \"/iserver/reply/{replyid}\" end-point. 
  #[serde(rename = "message")]
  message: Option<Vec<String>>
}

impl InlineResponse2009 {
  pub fn new() -> InlineResponse2009 {
    InlineResponse2009 {
      id: None,
      message: None
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> InlineResponse2009 {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_message(&mut self, message: Vec<String>) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: Vec<String>) -> InlineResponse2009 {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&Vec<String>> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

}



