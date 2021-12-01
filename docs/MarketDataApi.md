# \MarketDataApi

All URIs are relative to *https://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**iserver_marketdata_conid_unsubscribe_get**](MarketDataApi.md#iserver_marketdata_conid_unsubscribe_get) | **Get** /iserver/marketdata/{conid}/unsubscribe | Market Data Cancel (Single)
[**iserver_marketdata_history_get**](MarketDataApi.md#iserver_marketdata_history_get) | **Get** /iserver/marketdata/history | Market Data History
[**iserver_marketdata_snapshot_get**](MarketDataApi.md#iserver_marketdata_snapshot_get) | **Get** /iserver/marketdata/snapshot | Market Data
[**iserver_marketdata_unsubscribeall_get**](MarketDataApi.md#iserver_marketdata_unsubscribeall_get) | **Get** /iserver/marketdata/unsubscribeall | Market Data Cancel (All)


# **iserver_marketdata_conid_unsubscribe_get**
> ::models::InlineResponse20026 iserver_marketdata_conid_unsubscribe_get(conid)
Market Data Cancel (Single)

Cancel market data for given conid. To cancel all market data request(s), see /iserver/marketdata/unsubscribeall. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **conid** | **String**| contract id | 

### Return type

[**::models::InlineResponse20026**](inline_response_200_26.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_marketdata_history_get**
> ::models::HistoryData iserver_marketdata_history_get(conid, period, optional)
Market Data History

Get historical market Data for given conid, length of data is controlled by 'period' and 'bar'. Formatted as: min=minute, h=hour, d=day, w=week, m=month, y=year e.g. period =1y with bar =1w returns 52 data points (Max of 1000 data points supported). **Note**: There's a limit of 5 concurrent requests. Excessive requests will return a 'Too many requests' status 429 response. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **conid** | **String**| contract id | 
  **period** | **String**| available time period-- {1-30}min, {1-8}h, {1-1000}d, {1-792}w, {1-182}m, {1-15}y | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **conid** | **String**| contract id | 
 **period** | **String**| available time period-- {1-30}min, {1-8}h, {1-1000}d, {1-792}w, {1-182}m, {1-15}y | 
 **exchange** | **String**| Exchange of the conid (e.g. ISLAND, NYSE, etc.). Default value is empty which corresponds to primary exchange of the conid. | 
 **bar** | **String**| possible value-- 1min, 2min, 3min, 5min, 10min, 15min, 30min, 1h, 2h, 3h, 4h, 8h, 1d, 1w, 1m | 
 **outside_rth** | **bool**| For contracts that support it, will determine if historical data includes outside of regular trading hours. | 

### Return type

[**::models::HistoryData**](history-data.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_marketdata_snapshot_get**
> Vec<::models::InlineResponse20024> iserver_marketdata_snapshot_get(conids, optional)
Market Data

Get Market Data for the given conid(s). The endpoint will return by default bid, ask, last, change, change pct, close, listing exchange. See response fields for a list of available fields that can be request via fields argument. The endpoint /iserver/accounts must be called prior to /iserver/marketdata/snapshot. For derivative contracts the endpoint /iserver/secdef/search must be called first. First /snapshot endpoint call for given conid will initiate the market data request.  To receive all available fields the /snapshot endpoint will need to be called several times. To receive streaming market data the endpoint /ws can be used. Refer to [Streaming WebSocket Data](https://interactivebrokers.github.io/cpwebapi/RealtimeSubscription.html) for details. 

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

[**Vec<::models::InlineResponse20024>**](inline_response_200_24.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_marketdata_unsubscribeall_get**
> ::models::InlineResponse20025 iserver_marketdata_unsubscribeall_get()
Market Data Cancel (All)

Cancel all market data request(s). To cancel market data for given conid, see /iserver/marketdata/{conid}/unsubscribe. 

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20025**](inline_response_200_25.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

