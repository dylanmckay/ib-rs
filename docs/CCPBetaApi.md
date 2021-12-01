# \CCPBetaApi

All URIs are relative to *https://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ccp_account_get**](CCPBetaApi.md#ccp_account_get) | **Get** /ccp/account | Brokerage Accounts
[**ccp_auth_init_post**](CCPBetaApi.md#ccp_auth_init_post) | **Post** /ccp/auth/init | Start CCP Session
[**ccp_auth_response_post**](CCPBetaApi.md#ccp_auth_response_post) | **Post** /ccp/auth/response | Complete CCP Session
[**ccp_order_delete**](CCPBetaApi.md#ccp_order_delete) | **Delete** /ccp/order | Delete Order
[**ccp_order_post**](CCPBetaApi.md#ccp_order_post) | **Post** /ccp/order | Submit Order
[**ccp_order_put**](CCPBetaApi.md#ccp_order_put) | **Put** /ccp/order | Update Order
[**ccp_orders_get**](CCPBetaApi.md#ccp_orders_get) | **Get** /ccp/orders | Order Status
[**ccp_positions_get**](CCPBetaApi.md#ccp_positions_get) | **Get** /ccp/positions | Positions
[**ccp_status_get**](CCPBetaApi.md#ccp_status_get) | **Get** /ccp/status | CCP Status
[**ccp_trades_get**](CCPBetaApi.md#ccp_trades_get) | **Get** /ccp/trades | Trades


# **ccp_account_get**
> ::models::InlineResponse200 ccp_account_get()
Brokerage Accounts

Provides the list of tradeable accounts

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse200**](inline_response_200.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ccp_auth_init_post**
> ::models::InlineResponse2001 ccp_auth_init_post(optional)
Start CCP Session

Initiate a brokerage session to CCP. Only one brokerage session type can run at a time. If an existing brokerage session to iServer is running then call the endpoint /logout first. Note at this time only order management is possible from CCP session, market data and scanner endpoints can't be used since they are only available from iServer session. Work is in progress to provide new CCP endpoints for market data and scanners.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **compete** | **bool**| Allow competing CCP session to run | 
 **locale** | **String**| Concatenate value for language and region, set to \&quot;en_US\&quot; | 
 **mac** | **String**| Local MAC Address | 
 **machine_id** | **String**| Local machine ID | 
 **username** | **String**| Login user, set to dash \&quot;-\&quot; | 

### Return type

[**::models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ccp_auth_response_post**
> ::models::InlineResponse2002 ccp_auth_response_post(optional)
Complete CCP Session

Session Token Authentication

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **auth** | [**Auth**](Auth.md)|  | 

### Return type

[**::models::InlineResponse2002**](inline_response_200_2.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ccp_order_delete**
> ::models::OrderData ccp_order_delete(acct, id)
Delete Order

Sends an Order cancellation request. The status of the order can be queried through /ccp/order. Passing arguments as GET is also supported (requires passing action=delete) (GET is meant for development only) 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **acct** | **String**| Account Number | 
  **id** | **f32**| Order Identifier of original submit order | 

### Return type

[**::models::OrderData**](order-data.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ccp_order_post**
> ::models::OrderData ccp_order_post(acct, conid, ccy, exchange, qty, optional)
Submit Order

Submits an Order. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **acct** | **String**| User Account | 
  **conid** | **f32**| Contract identifier from IBKR&#39;s database. | 
  **ccy** | **String**| Contract Currency | 
  **exchange** | **String**| Exchange | 
  **qty** | **f32**| Order Quantity | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **acct** | **String**| User Account | 
 **conid** | **f32**| Contract identifier from IBKR&#39;s database. | 
 **ccy** | **String**| Contract Currency | 
 **exchange** | **String**| Exchange | 
 **qty** | **f32**| Order Quantity | 
 **_type** | **String**| Order Price; required if order type is limit | 
 **side** | **String**| Side | 
 **price** | **f32**| Order Price; required if order type is limit | 
 **tif** | **String**| Time in Force | 

### Return type

[**::models::OrderData**](order-data.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ccp_order_put**
> ::models::OrderData ccp_order_put(acct, id)
Update Order

Updates an Order. Updating an order requires the same arguments as placing an order besides the conid. Note: The status of the order can be queried through GET /ccp/order. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **acct** | **String**| User Account | 
  **id** | **f32**| Order ID to be modified | 

### Return type

[**::models::OrderData**](order-data.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ccp_orders_get**
> ::models::InlineResponse2003 ccp_orders_get(acct, optional)
Order Status

Get status for all orders

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **acct** | **String**| User Account | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **acct** | **String**| User Account | 
 **cancelled** | **bool**| Return only Rejected or Cancelled orders since today midnight | 

### Return type

[**::models::InlineResponse2003**](inline_response_200_3.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ccp_positions_get**
> ::models::PositionData ccp_positions_get()
Positions

List of positions

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::PositionData**](position-data.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ccp_status_get**
> ::models::InlineResponse2004 ccp_status_get()
CCP Status

Provide the current CCP session status. When using the Gateway this endpoint will also initiate a brokerage session to CCP by sending /auth/init and response.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse2004**](inline_response_200_4.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ccp_trades_get**
> ::models::InlineResponse2003 ccp_trades_get(optional)
Trades

Get a list of Trades, by default, the list is from today midnight to Date.now(). 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **from** | **String**| From Date (YYYYMMDD-HH:mm:ss) or offset (-1,-2,-3..) | 
 **to** | **String**| To Date (YYYYMMDD-HH:mm:ss) or offset (-1,-2,-3..). To value should be bigger than from value.  | 

### Return type

[**::models::InlineResponse2003**](inline_response_200_3.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

