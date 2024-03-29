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
pub struct InlineResponse20035 {
  /// Time of session validation
  #[serde(rename = "AUTH_TIME")]
  AUTH_TIME: Option<f32>,
  /// 1 for Live, 2 for Paper
  #[serde(rename = "LOGIN_TYPE")]
  LOGIN_TYPE: Option<f32>,
  /// true if session was validated; false if not.
  #[serde(rename = "RESULT")]
  RESULT: Option<bool>,
  /// User ID
  #[serde(rename = "USER_ID")]
  USER_ID: Option<f32>,
  /// Username
  #[serde(rename = "USER_NAME")]
  USER_NAME: Option<String>,
  /// Time in milliseconds until session expires. Caller needs to call the again to re-validate session
  #[serde(rename = "expire")]
  expire: Option<f32>
}

impl InlineResponse20035 {
  pub fn new() -> InlineResponse20035 {
    InlineResponse20035 {
      AUTH_TIME: None,
      LOGIN_TYPE: None,
      RESULT: None,
      USER_ID: None,
      USER_NAME: None,
      expire: None
    }
  }

  pub fn set_AUTH_TIME(&mut self, AUTH_TIME: f32) {
    self.AUTH_TIME = Some(AUTH_TIME);
  }

  pub fn with_AUTH_TIME(mut self, AUTH_TIME: f32) -> InlineResponse20035 {
    self.AUTH_TIME = Some(AUTH_TIME);
    self
  }

  pub fn AUTH_TIME(&self) -> Option<&f32> {
    self.AUTH_TIME.as_ref()
  }

  pub fn reset_AUTH_TIME(&mut self) {
    self.AUTH_TIME = None;
  }

  pub fn set_LOGIN_TYPE(&mut self, LOGIN_TYPE: f32) {
    self.LOGIN_TYPE = Some(LOGIN_TYPE);
  }

  pub fn with_LOGIN_TYPE(mut self, LOGIN_TYPE: f32) -> InlineResponse20035 {
    self.LOGIN_TYPE = Some(LOGIN_TYPE);
    self
  }

  pub fn LOGIN_TYPE(&self) -> Option<&f32> {
    self.LOGIN_TYPE.as_ref()
  }

  pub fn reset_LOGIN_TYPE(&mut self) {
    self.LOGIN_TYPE = None;
  }

  pub fn set_RESULT(&mut self, RESULT: bool) {
    self.RESULT = Some(RESULT);
  }

  pub fn with_RESULT(mut self, RESULT: bool) -> InlineResponse20035 {
    self.RESULT = Some(RESULT);
    self
  }

  pub fn RESULT(&self) -> Option<&bool> {
    self.RESULT.as_ref()
  }

  pub fn reset_RESULT(&mut self) {
    self.RESULT = None;
  }

  pub fn set_USER_ID(&mut self, USER_ID: f32) {
    self.USER_ID = Some(USER_ID);
  }

  pub fn with_USER_ID(mut self, USER_ID: f32) -> InlineResponse20035 {
    self.USER_ID = Some(USER_ID);
    self
  }

  pub fn USER_ID(&self) -> Option<&f32> {
    self.USER_ID.as_ref()
  }

  pub fn reset_USER_ID(&mut self) {
    self.USER_ID = None;
  }

  pub fn set_USER_NAME(&mut self, USER_NAME: String) {
    self.USER_NAME = Some(USER_NAME);
  }

  pub fn with_USER_NAME(mut self, USER_NAME: String) -> InlineResponse20035 {
    self.USER_NAME = Some(USER_NAME);
    self
  }

  pub fn USER_NAME(&self) -> Option<&String> {
    self.USER_NAME.as_ref()
  }

  pub fn reset_USER_NAME(&mut self) {
    self.USER_NAME = None;
  }

  pub fn set_expire(&mut self, expire: f32) {
    self.expire = Some(expire);
  }

  pub fn with_expire(mut self, expire: f32) -> InlineResponse20035 {
    self.expire = Some(expire);
    self
  }

  pub fn expire(&self) -> Option<&f32> {
    self.expire.as_ref()
  }

  pub fn reset_expire(&mut self) {
    self.expire = None;
  }

}



