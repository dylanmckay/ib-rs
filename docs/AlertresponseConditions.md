# AlertresponseConditions

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**condition_logic_bind** | **String** | Condition array should end with \&quot;n\&quot;   * a - AND   * o - OR   * n - END  | [optional] [default to null]
**condition_operator** | **String** | optional, operator for the current condition   * &gt;&#x3D; Greater than or equal to   * &lt;&#x3D; Less than or equal to  | [optional] [default to null]
**condition_time_zone** | **String** | only needed for some MTA alert condition | [optional] [default to null]
**condition_trigger_method** | **String** | optional, only some type of conditions have triggerMethod | [optional] [default to null]
**condition_type** | **i32** | Types: 1-Price, 3-Time, 4-Margin, 5-Trade, 6-Volume, 7: MTA market 8: MTA Position, 9: MTA Acc. Daily PN&amp;  | [optional] [default to null]
**condition_value** | **String** | can not be empty, can pass default value \&quot;*\&quot; | [optional] [default to null]
**conidex** | **String** | conid and exchange. Format supports conid or conid@exchange | [optional] [default to null]
**contract_description_1** | **String** | Format contract name | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


