# \OrderApi

All URIs are relative to *https://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**iserver_account_account_id_order_order_id_delete**](OrderApi.md#iserver_account_account_id_order_order_id_delete) | **Delete** /iserver/account/{accountId}/order/{orderId} | Cancel Order
[**iserver_account_account_id_order_order_id_post**](OrderApi.md#iserver_account_account_id_order_order_id_post) | **Post** /iserver/account/{accountId}/order/{orderId} | Modify Order
[**iserver_account_account_id_order_post**](OrderApi.md#iserver_account_account_id_order_post) | **Post** /iserver/account/{accountId}/order | Place Order (Deprecated)
[**iserver_account_account_id_order_whatif_post**](OrderApi.md#iserver_account_account_id_order_whatif_post) | **Post** /iserver/account/{accountId}/order/whatif | Preview Order (Deprecated)
[**iserver_account_account_id_orders_post**](OrderApi.md#iserver_account_account_id_orders_post) | **Post** /iserver/account/{accountId}/orders | Place Orders
[**iserver_account_account_id_orders_whatif_post**](OrderApi.md#iserver_account_account_id_orders_whatif_post) | **Post** /iserver/account/{accountId}/orders/whatif | Preview Orders
[**iserver_account_order_status_order_id_get**](OrderApi.md#iserver_account_order_status_order_id_get) | **Get** /iserver/account/order/status/{orderId} | Order Status
[**iserver_account_orders_fa_group_post**](OrderApi.md#iserver_account_orders_fa_group_post) | **Post** /iserver/account/orders/{faGroup} | Place Orders for FA
[**iserver_account_orders_get**](OrderApi.md#iserver_account_orders_get) | **Get** /iserver/account/orders | Live Orders
[**iserver_reply_replyid_post**](OrderApi.md#iserver_reply_replyid_post) | **Post** /iserver/reply/{replyid} | Place Order Reply


# **iserver_account_account_id_order_order_id_delete**
> ::models::InlineResponse20020 iserver_account_account_id_order_order_id_delete(account_id, order_id)
Cancel Order

Cancels an open order. Must call /iserver/accounts endpoint prior to cancelling an order. Use /iservers/account/orders endpoint to review open-order(s) and get latest order status.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| account id, or fa group if deleting a group order | 
  **order_id** | **String**| Customer order id, use /iservers/account/orders endpoint to query orderId. | 

### Return type

[**::models::InlineResponse20020**](inline_response_200_20.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_account_account_id_order_order_id_post**
> Vec<::models::InlineResponse20019> iserver_account_account_id_order_order_id_post(account_id, order_id, body)
Modify Order

Modifies an open order. Must call /iserver/accounts endpoint prior to modifying an order. Use /iservers/account/orders endpoint to review open-order(s).

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| account id, or fa group if modifying a group order | 
  **order_id** | **String**| Customer order id, use /iservers/account/orders endpoint to query orderId. | 
  **body** | [**ModifyOrder**](ModifyOrder.md)| modify-order request | 

### Return type

[**Vec<::models::InlineResponse20019>**](inline_response_200_19.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_account_account_id_order_post**
> Vec<::models::InlineResponse20015> iserver_account_account_id_order_post(account_id, body)
Place Order (Deprecated)

This end-point is going to be deprecated, you can use /iserver/account/{accountId}/orders, just pass one order in the array, the order structure will be same. Please note here, sometimes this endpoint alone can't make sure you submit the order successfully, you could receive some questions in the response, you have to to answer them in order to submit the order successfully. You can use \"/iserver/reply/{replyid}\" endpoint to answer questions 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| account id | 
  **body** | [**OrderRequest**](OrderRequest.md)| order request info | 

### Return type

[**Vec<::models::InlineResponse20015>**](inline_response_200_15.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_account_account_id_order_whatif_post**
> ::models::InlineResponse20018 iserver_account_account_id_order_whatif_post(account_id, body)
Preview Order (Deprecated)

This end-point is going to be deprecated, you can use /iserver/account/{accountId}/orders/whatif, just pass one order in the array, the order structure will be same. This endpoint allows you to preview order without actually submitting the order and you can get commission information in the response. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| account id | 
  **body** | [**OrderRequest**](OrderRequest.md)| order info | 

### Return type

[**::models::InlineResponse20018**](inline_response_200_18.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_account_account_id_orders_post**
> Vec<::models::InlineResponse20015> iserver_account_account_id_orders_post(account_id, body)
Place Orders

You can pass a list of orders here such as bracket and OCA orders. Notes for OCA(one cancel all orders) orders: 1. if one order in the group is filled/cancelled, all the others in the same group will be cancelled. 2. To create OCA orders, please set isSingleGroup to true in each order 3. All orders in the same group will have same oca_group_id, you can get oca_group_id from /iserver/account/order/status/{orderId} end-point after orders are placed successfully 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| account id | 
  **body** | [**Body4**](Body4.md)| order request info | 

### Return type

[**Vec<::models::InlineResponse20015>**](inline_response_200_15.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_account_account_id_orders_whatif_post**
> ::models::InlineResponse20018 iserver_account_account_id_orders_whatif_post(account_id, body)
Preview Orders

This endpoint allows you to preview order without actually submitting the order and you can get commission information in the response. Also supports bracket orders. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| account id | 
  **body** | [**Body5**](Body5.md)| order info | 

### Return type

[**::models::InlineResponse20018**](inline_response_200_18.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_account_order_status_order_id_get**
> ::models::OrderStatus iserver_account_order_status_order_id_get(order_id)
Order Status

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **order_id** | **String**| Customer order id, use /iservers/account/orders endpoint to query orderId. | 

### Return type

[**::models::OrderStatus**](order-status.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_account_orders_fa_group_post**
> Vec<::models::InlineResponse20015> iserver_account_orders_fa_group_post(fa_group, body)
Place Orders for FA

Financial Advisors can use this endpoint to place an order for a specified group. To place an order for a specified account use the endpoint /iserver/account/{accountId}/order. More information about groups can be found in the [TWS Users' Guide:](https://guides.interactivebrokers.com/twsguide/twsguide.htm#usersguidebook/financialadvisors/create_an_account_group_for_share_allocation.htm). 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **fa_group** | **String**| financial advisor group | 
  **body** | [**OrderRequest**](OrderRequest.md)| order request info | 

### Return type

[**Vec<::models::InlineResponse20015>**](inline_response_200_15.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_account_orders_get**
> ::models::InlineResponse20014 iserver_account_orders_get(optional)
Live Orders

The endpoint is meant to be used in polling mode, e.g. requesting every x seconds. The response will contain two objects, one is notification, the other is orders. Orders is the list of live orders (cancelled, filled, submitted). Notifications contains information about execute orders as they happen, see status field. To receive streaming live orders the endpoint /ws can be used. Refer to [Streaming WebSocket Data](https://interactivebrokers.github.io/cpwebapi/RealtimeSubscription.html) for details. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | [**Body3**](Body3.md)| an array of filters | 

### Return type

[**::models::InlineResponse20014**](inline_response_200_14.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_reply_replyid_post**
> Vec<::models::InlineResponse20019> iserver_reply_replyid_post(replyid, body)
Place Order Reply

Reply to questions when placing orders and submit orders

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **replyid** | **String**| Please use the \&quot;id\&quot; from the response of \&quot;Place Order\&quot; endpoint | 
  **body** | [**Body6**](Body6.md)| Answer to question | 

### Return type

[**Vec<::models::InlineResponse20019>**](inline_response_200_19.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

