# TransactionsTransactions

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acctid** | **String** |  | [optional] [default to null]
**amt** | **f32** | Raw value, no formatting. Transaction amount. For trades does not include commission. In asset currency | [optional] [default to null]
**conid** | **f32** |  | [optional] [default to null]
**cur** | **String** | currency code | [optional] [default to null]
**date** | **String** | Date of transaction.  Epoch time, GMT | [optional] [default to null]
**desc** | **String** | Transaction description | [optional] [default to null]
**fx_rate** | **f32** | Conversion rate from asset currency to response currency | [optional] [default to null]
**pr** | **f32** | In asset currency. Not be applicable for all transaction types. | [optional] [default to null]
**qty** | **f32** | Not applicable for all transaction types | [optional] [default to null]
**_type** | **String** | Transaction Type Name: Examples: \&quot;Sell\&quot;, \&quot;Buy\&quot;, \&quot;Corporate Action\&quot;, \&quot;Dividend Payment\&quot;, \&quot;Transfer\&quot;, \&quot;Payment in Lieu\&quot; Dividends and Transfers do not have price and quantity in response  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


