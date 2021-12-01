# ModifyOrder

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acct_id** | **String** |  | [optional] [default to null]
**aux_price** | **f32** |  | [optional] [default to null]
**conid** | **i32** |  | [optional] [default to null]
**deactivated** | **bool** | Set to true if you want to pause a working order. For details refer to the [TWS Users&#39; Guide:](https://guides.interactivebrokers.com/tws/twsguide.html#usersguidebook/getstarted/pause_execution.htm)  | [optional] [default to null]
**listing_exchange** | **String** | optional, not required | [optional] [default to null]
**order_type** | **String** | for example LMT | [optional] [default to null]
**outside_rth** | **bool** |  | [optional] [default to null]
**price** | **f32** |  | [optional] [default to null]
**quantity** | **f32** | usually integer, for some special cases can be float numbers | [optional] [default to null]
**side** | **String** | SELL or BUY | [optional] [default to null]
**ticker** | **String** | The ticker symbol of the original place order | [optional] [default to null]
**tif** | **String** | Specify a time in force to change how long your order will continue to work in the market | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


