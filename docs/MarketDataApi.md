# \MarketDataApi

All URIs are relative to *https://localhost:5000/v1/portal*

Method | HTTP request | Description
------------- | ------------- | -------------
[**iserver_marketdata_history_get**](MarketDataApi.md#iserver_marketdata_history_get) | **Get** /iserver/marketdata/history | Market Data History
[**iserver_marketdata_snapshot_get**](MarketDataApi.md#iserver_marketdata_snapshot_get) | **Get** /iserver/marketdata/snapshot | Market Data


# **iserver_marketdata_history_get**
> ::models::HistoryData iserver_marketdata_history_get(conid, period, optional)
Market Data History

Get history of market Data for the given conid, length of data is controlled by period and bar. e.g. 1y period with bar =1w returns 52 data points

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **conid** | **String**| contract id | 
  **period** | **String**| time period-- 1d,1w,1m,1y | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **conid** | **String**| contract id | 
 **period** | **String**| time period-- 1d,1w,1m,1y | 
 **bar** | **String**| possible value-- 5min,1h,1w | 

### Return type

[**::models::HistoryData**](history-data.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_marketdata_snapshot_get**
> Vec<::models::InlineResponse20013> iserver_marketdata_snapshot_get(conids, optional)
Market Data

Get Market Data for the given conid(s). The end-point will return by default bid, ask, last, change, change pct, close, listing exchange. See response fields for a list of available fields that can be request via fields argument. The endpoint /iserver/accounts should be called prior to /iserver/marketdata/snapshot. To receive all available fields the /snapshot endpoint will need to be called several times. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **conids** | **String**| list of conids separated by comma | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **conids** | **String**| list of conids separated by comma | 
 **since** | **i32**| time period since which updates are required. uses epoch time with milliseconds. | 
 **fields** | **String**| list of fields separated by comma | 

### Return type

[**Vec<::models::InlineResponse20013>**](inline_response_200_13.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

