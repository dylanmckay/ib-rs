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
pub struct InlineResponse20021 {
  /// Unique account id
  #[serde(rename = "accounts")]
  accounts: Option<Vec<String>>,
  /// Account Id and its alias
  #[serde(rename = "aliases")]
  aliases: Option<Value>,
  #[serde(rename = "selectedAccount")]
  selected_account: Option<String>
}

impl InlineResponse20021 {
  pub fn new() -> InlineResponse20021 {
    InlineResponse20021 {
      accounts: None,
      aliases: None,
      selected_account: None
    }
  }

  pub fn set_accounts(&mut self, accounts: Vec<String>) {
    self.accounts = Some(accounts);
  }

  pub fn with_accounts(mut self, accounts: Vec<String>) -> InlineResponse20021 {
    self.accounts = Some(accounts);
    self
  }

  pub fn accounts(&self) -> Option<&Vec<String>> {
    self.accounts.as_ref()
  }

  pub fn reset_accounts(&mut self) {
    self.accounts = None;
  }

  pub fn set_aliases(&mut self, aliases: Value) {
    self.aliases = Some(aliases);
  }

  pub fn with_aliases(mut self, aliases: Value) -> InlineResponse20021 {
    self.aliases = Some(aliases);
    self
  }

  pub fn aliases(&self) -> Option<&Value> {
    self.aliases.as_ref()
  }

  pub fn reset_aliases(&mut self) {
    self.aliases = None;
  }

  pub fn set_selected_account(&mut self, selected_account: String) {
    self.selected_account = Some(selected_account);
  }

  pub fn with_selected_account(mut self, selected_account: String) -> InlineResponse20021 {
    self.selected_account = Some(selected_account);
    self
  }

  pub fn selected_account(&self) -> Option<&String> {
    self.selected_account.as_ref()
  }

  pub fn reset_selected_account(&mut self) {
    self.selected_account = None;
  }

}



