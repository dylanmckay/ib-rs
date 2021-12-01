# InlineResponse20014Orders

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acct** | **String** | Account number | [optional] [default to null]
**bg_color** | **String** | Background color in hex format | [optional] [default to null]
**cash_ccy** | **String** | Cash currency | [optional] [default to null]
**company_name** | **String** | Company Name | [optional] [default to null]
**conid** | **f32** | Contract identifier | [optional] [default to null]
**conidex** | **String** | conid and exchange. Format supports conid or conid@exchange | [optional] [default to null]
**description1** | **String** | Formatted ticker description | [optional] [default to null]
**fg_color** | **String** | Foreground color in hex format | [optional] [default to null]
**filled_quantity** | **f32** | Quantity filled | [optional] [default to null]
**last_execution_time** | **f32** | Last status update in format YYMMDDhhmms based in GMT | [optional] [default to null]
**last_execution_time_r** | **f32** | Last status update unix time in ms | [optional] [default to null]
**listing_exchange** | **String** | Listing Exchange | [optional] [default to null]
**order_desc** | **String** | Order description | [optional] [default to null]
**order_id** | **String** | Order identifier | [optional] [default to null]
**order_type** | **String** | Order type | [optional] [default to null]
**order_ref** | **String** | Order reference | [optional] [default to null]
**orig_order_type** | **String** | Original order type | [optional] [default to null]
**price** | **f32** | Price of order | [optional] [default to null]
**remaining_quantity** | **f32** | Quantity remaining | [optional] [default to null]
**sec_type** | **String** | Asset class | [optional] [default to null]
**side** | **String** | The side of the market of the order.  * BUY: Buy contract near posted ask price  * SELL: Sell contract near posted bid price  * ASSN: Option Assignment, if BUYSELL&#x3D;BUY and OptionType&#x3D;PUT or BUYSELL&#x3D;SELL and OptionType&#x3D;CALL  * EXER: Option Exercise, if BUYSELL&#x3D;SELL and OptionType&#x3D;PUT or BUYSELL&#x3D;BUY and OptionType&#x3D;CALL  | [optional] [default to null]
**size_and_fills** | **f32** | Quantity outstanding and total quantity concatenated with forward slash separator | [optional] [default to null]
**status** | **String** | Status of the order | [optional] [default to null]
**supports_tax_opt** | **f32** | Supports Tax Optimization with 0 for no and 1 for yes | [optional] [default to null]
**ticker** | **String** | Underlying symbol | [optional] [default to null]
**time_in_force** | **String** | Time in force | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


