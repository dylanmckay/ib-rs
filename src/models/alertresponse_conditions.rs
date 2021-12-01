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
pub struct AlertresponseConditions {
  /// Condition array should end with \"n\"   * a - AND   * o - OR   * n - END 
  #[serde(rename = "condition_logic_bind")]
  condition_logic_bind: Option<String>,
  /// optional, operator for the current condition   * >= Greater than or equal to   * <= Less than or equal to 
  #[serde(rename = "condition_operator")]
  condition_operator: Option<String>,
  /// only needed for some MTA alert condition
  #[serde(rename = "condition_time_zone")]
  condition_time_zone: Option<String>,
  /// optional, only some type of conditions have triggerMethod
  #[serde(rename = "condition_trigger_method")]
  condition_trigger_method: Option<String>,
  /// Types: 1-Price, 3-Time, 4-Margin, 5-Trade, 6-Volume, 7: MTA market 8: MTA Position, 9: MTA Acc. Daily PN& 
  #[serde(rename = "condition_type")]
  condition_type: Option<i32>,
  /// can not be empty, can pass default value \"*\"
  #[serde(rename = "condition_value")]
  condition_value: Option<String>,
  /// conid and exchange. Format supports conid or conid@exchange
  #[serde(rename = "conidex")]
  conidex: Option<String>,
  /// Format contract name
  #[serde(rename = "contract_description_1")]
  contract_description_1: Option<String>
}

impl AlertresponseConditions {
  pub fn new() -> AlertresponseConditions {
    AlertresponseConditions {
      condition_logic_bind: None,
      condition_operator: None,
      condition_time_zone: None,
      condition_trigger_method: None,
      condition_type: None,
      condition_value: None,
      conidex: None,
      contract_description_1: None
    }
  }

  pub fn set_condition_logic_bind(&mut self, condition_logic_bind: String) {
    self.condition_logic_bind = Some(condition_logic_bind);
  }

  pub fn with_condition_logic_bind(mut self, condition_logic_bind: String) -> AlertresponseConditions {
    self.condition_logic_bind = Some(condition_logic_bind);
    self
  }

  pub fn condition_logic_bind(&self) -> Option<&String> {
    self.condition_logic_bind.as_ref()
  }

  pub fn reset_condition_logic_bind(&mut self) {
    self.condition_logic_bind = None;
  }

  pub fn set_condition_operator(&mut self, condition_operator: String) {
    self.condition_operator = Some(condition_operator);
  }

  pub fn with_condition_operator(mut self, condition_operator: String) -> AlertresponseConditions {
    self.condition_operator = Some(condition_operator);
    self
  }

  pub fn condition_operator(&self) -> Option<&String> {
    self.condition_operator.as_ref()
  }

  pub fn reset_condition_operator(&mut self) {
    self.condition_operator = None;
  }

  pub fn set_condition_time_zone(&mut self, condition_time_zone: String) {
    self.condition_time_zone = Some(condition_time_zone);
  }

  pub fn with_condition_time_zone(mut self, condition_time_zone: String) -> AlertresponseConditions {
    self.condition_time_zone = Some(condition_time_zone);
    self
  }

  pub fn condition_time_zone(&self) -> Option<&String> {
    self.condition_time_zone.as_ref()
  }

  pub fn reset_condition_time_zone(&mut self) {
    self.condition_time_zone = None;
  }

  pub fn set_condition_trigger_method(&mut self, condition_trigger_method: String) {
    self.condition_trigger_method = Some(condition_trigger_method);
  }

  pub fn with_condition_trigger_method(mut self, condition_trigger_method: String) -> AlertresponseConditions {
    self.condition_trigger_method = Some(condition_trigger_method);
    self
  }

  pub fn condition_trigger_method(&self) -> Option<&String> {
    self.condition_trigger_method.as_ref()
  }

  pub fn reset_condition_trigger_method(&mut self) {
    self.condition_trigger_method = None;
  }

  pub fn set_condition_type(&mut self, condition_type: i32) {
    self.condition_type = Some(condition_type);
  }

  pub fn with_condition_type(mut self, condition_type: i32) -> AlertresponseConditions {
    self.condition_type = Some(condition_type);
    self
  }

  pub fn condition_type(&self) -> Option<&i32> {
    self.condition_type.as_ref()
  }

  pub fn reset_condition_type(&mut self) {
    self.condition_type = None;
  }

  pub fn set_condition_value(&mut self, condition_value: String) {
    self.condition_value = Some(condition_value);
  }

  pub fn with_condition_value(mut self, condition_value: String) -> AlertresponseConditions {
    self.condition_value = Some(condition_value);
    self
  }

  pub fn condition_value(&self) -> Option<&String> {
    self.condition_value.as_ref()
  }

  pub fn reset_condition_value(&mut self) {
    self.condition_value = None;
  }

  pub fn set_conidex(&mut self, conidex: String) {
    self.conidex = Some(conidex);
  }

  pub fn with_conidex(mut self, conidex: String) -> AlertresponseConditions {
    self.conidex = Some(conidex);
    self
  }

  pub fn conidex(&self) -> Option<&String> {
    self.conidex.as_ref()
  }

  pub fn reset_conidex(&mut self) {
    self.conidex = None;
  }

  pub fn set_contract_description_1(&mut self, contract_description_1: String) {
    self.contract_description_1 = Some(contract_description_1);
  }

  pub fn with_contract_description_1(mut self, contract_description_1: String) -> AlertresponseConditions {
    self.contract_description_1 = Some(contract_description_1);
    self
  }

  pub fn contract_description_1(&self) -> Option<&String> {
    self.contract_description_1.as_ref()
  }

  pub fn reset_contract_description_1(&mut self) {
    self.contract_description_1 = None;
  }

}


