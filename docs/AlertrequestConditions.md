# AlertrequestConditions

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**conidex** | **String** | conid and exchange. Format supports conid or conid@exchange | [optional] [default to null]
**logic_bind** | **String** | \&quot;a\&quot; means \&quot;AND\&quot;, \&quot;o\&quot; means \&quot;OR\&quot;, \&quot;n\&quot; means \&quot;END\&quot;, the last one condition in the condition array should \&quot;n\&quot;  | [optional] [default to null]
**operator** | **String** | optional, operator for the current condition, can be &gt;&#x3D; or &lt;&#x3D; | [optional] [default to null]
**time_zone** | **String** | only needed for some MTA alert condition | [optional] [default to null]
**trigger_method** | **String** | optional, only some type of conditions have triggerMethod | [optional] [default to null]
**_type** | **i32** | Types: 1-Price, 3-Time, 4-Margin, 5-Trade, 6-Volume, 7: MTA market 8: MTA Position, 9: MTA Acc. Daily PN&amp;  | [optional] [default to null]
**value** | **String** | can not be empty, can pass default value \&quot;*\&quot; | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


