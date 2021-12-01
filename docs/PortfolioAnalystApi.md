# \PortfolioAnalystApi

All URIs are relative to *https://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**pa_performance_post**](PortfolioAnalystApi.md#pa_performance_post) | **Post** /pa/performance | Account Performance
[**pa_summary_post**](PortfolioAnalystApi.md#pa_summary_post) | **Post** /pa/summary | Account Balance&#39;s Summary
[**pa_transactions_post**](PortfolioAnalystApi.md#pa_transactions_post) | **Post** /pa/transactions | Position&#39;s Transaction History


# **pa_performance_post**
> ::models::Performance pa_performance_post(body)
Account Performance

Returns the performance (MTM) for the given accounts, if more than one account is passed, the result is consolidated.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**Body7**](Body7.md)| an array of account ids | 

### Return type

[**::models::Performance**](performance.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **pa_summary_post**
> ::models::Summary pa_summary_post(body)
Account Balance's Summary

Returns a summary of all account balances for the given accounts, if more than one account is passed, the result is consolidated.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**Body8**](Body8.md)| an array of account ids | 

### Return type

[**::models::Summary**](summary.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **pa_transactions_post**
> ::models::Transactions pa_transactions_post(body)
Position's Transaction History

transaction history for a given number of conids and accounts. Types of transactions include dividend payments, buy and sell transactions, transfers. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**Body9**](Body9.md)|  | 

### Return type

[**::models::Transactions**](transactions.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

