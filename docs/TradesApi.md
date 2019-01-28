# \TradesApi

All URIs are relative to *https://localhost:5000/v1/portal*

Method | HTTP request | Description
------------- | ------------- | -------------
[**iserver_account_trades_get**](TradesApi.md#iserver_account_trades_get) | **Get** /iserver/account/trades | List of Trades for the selected account


# **iserver_account_trades_get**
> Vec<::models::Trade> iserver_account_trades_get()
List of Trades for the selected account

Returns a list of trades for the currently selected account for the current day.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**Vec<::models::Trade>**](trade.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

