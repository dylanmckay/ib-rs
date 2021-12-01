# IservercontractconidalgosParameters

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_value** | **bool** | User configured preset for this parameter. | [optional] [default to null]
**description** | **String** | Detailed description of the parameter. | [optional] [default to null]
**enabled_conditions** | **String** | The rules that UI should apply to algo parameters depending on chosen order type:  * MKT:speedUp:&#x3D;:no - hide SpeedUp param when MKT is chosen for order type.  * LMT:strategyType:&lt;&gt;:empty - strategyType param cannot be empty when LMT is chosen for order type.  * MKT:strategyType:&#x3D;:Marketable - set strategyType param to Marketable and disable (no other choice) when MKT is chosen for order type.  | [optional] [default to null]
**gui_rank** | **f32** | The order in UI, used when building dynamic UI so that more important parameters are presented first. | [optional] [default to null]
**id** | **String** | The algo parameter | [default to null]
**legal_strings** | **String** | The list of choices | [optional] [default to null]
**max_value** | **f32** | Largest value, only applies to parameters with valueClassName&#x3D;Double. | [optional] [default to null]
**min_value** | **f32** | Smallest value, only applies to parameters with valueClassName&#x3D;Double. | [optional] [default to null]
**name** | **String** | Descriptive name of the parameter. | [optional] [default to null]
**price_market_rule** | **bool** | If true, must specify parameter using market rule format. Only applies to parameters with valueClassName&#x3D;Double. | [optional] [default to null]
**required** | **bool** | If true a value must be entered. | [optional] [default to null]
**value_class_name** | **String** | Format of the parameter. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


