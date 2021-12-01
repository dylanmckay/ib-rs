# Transactions

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**currency** | **String** | same as request | [optional] [default to null]
**from** | **f32** | Period start date. Epoch time, GMT | [optional] [default to null]
**id** | **String** | will always be getTransactions | [optional] [default to null]
**includes_real_time** | **bool** | Indicates whether current day and realtime data is included in the result | [optional] [default to null]
**to** | **f32** | Period end date. Epoch time, GMT | [optional] [default to null]
**transactions** | [**Vec<::models::TransactionsTransactions>**](transactions_transactions.md) | Sorted by date descending | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


