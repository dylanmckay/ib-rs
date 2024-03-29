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
pub struct AuthStatus {
  /// Brokerage session is authenticated
  #[serde(rename = "authenticated")]
  authenticated: Option<bool>,
  /// Brokerage session is competing, e.g. user is logged in to IBKR Mobile, WebTrader, TWS or other trading platforms.
  #[serde(rename = "competing")]
  competing: Option<bool>,
  /// Connected to backend
  #[serde(rename = "connected")]
  connected: Option<bool>,
  /// Authentication failed, why.
  #[serde(rename = "fail")]
  fail: Option<String>,
  /// System messages that may affect trading
  #[serde(rename = "message")]
  message: Option<String>,
  /// Prompt messages that may affect trading or the account
  #[serde(rename = "prompts")]
  prompts: Option<Vec<String>>
}

impl AuthStatus {
  pub fn new() -> AuthStatus {
    AuthStatus {
      authenticated: None,
      competing: None,
      connected: None,
      fail: None,
      message: None,
      prompts: None
    }
  }

  pub fn set_authenticated(&mut self, authenticated: bool) {
    self.authenticated = Some(authenticated);
  }

  pub fn with_authenticated(mut self, authenticated: bool) -> AuthStatus {
    self.authenticated = Some(authenticated);
    self
  }

  pub fn authenticated(&self) -> Option<&bool> {
    self.authenticated.as_ref()
  }

  pub fn reset_authenticated(&mut self) {
    self.authenticated = None;
  }

  pub fn set_competing(&mut self, competing: bool) {
    self.competing = Some(competing);
  }

  pub fn with_competing(mut self, competing: bool) -> AuthStatus {
    self.competing = Some(competing);
    self
  }

  pub fn competing(&self) -> Option<&bool> {
    self.competing.as_ref()
  }

  pub fn reset_competing(&mut self) {
    self.competing = None;
  }

  pub fn set_connected(&mut self, connected: bool) {
    self.connected = Some(connected);
  }

  pub fn with_connected(mut self, connected: bool) -> AuthStatus {
    self.connected = Some(connected);
    self
  }

  pub fn connected(&self) -> Option<&bool> {
    self.connected.as_ref()
  }

  pub fn reset_connected(&mut self) {
    self.connected = None;
  }

  pub fn set_fail(&mut self, fail: String) {
    self.fail = Some(fail);
  }

  pub fn with_fail(mut self, fail: String) -> AuthStatus {
    self.fail = Some(fail);
    self
  }

  pub fn fail(&self) -> Option<&String> {
    self.fail.as_ref()
  }

  pub fn reset_fail(&mut self) {
    self.fail = None;
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> AuthStatus {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

  pub fn set_prompts(&mut self, prompts: Vec<String>) {
    self.prompts = Some(prompts);
  }

  pub fn with_prompts(mut self, prompts: Vec<String>) -> AuthStatus {
    self.prompts = Some(prompts);
    self
  }

  pub fn prompts(&self) -> Option<&Vec<String>> {
    self.prompts.as_ref()
  }

  pub fn reset_prompts(&mut self) {
    self.prompts = None;
  }

}



