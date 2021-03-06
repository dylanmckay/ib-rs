# \OrderApi

All URIs are relative to *https://localhost:5000/v1/portal*

Method | HTTP request | Description
------------- | ------------- | -------------
[**iserver_account_account_id_order_orig_customer_order_id_delete**](OrderApi.md#iserver_account_account_id_order_orig_customer_order_id_delete) | **Delete** /iserver/account/{accountId}/order/{origCustomerOrderId} | Delete Order
[**iserver_account_account_id_order_orig_customer_order_id_post**](OrderApi.md#iserver_account_account_id_order_orig_customer_order_id_post) | **Post** /iserver/account/{accountId}/order/{origCustomerOrderId} | Modify Order
[**iserver_account_account_id_order_post**](OrderApi.md#iserver_account_account_id_order_post) | **Post** /iserver/account/{accountId}/order | Place Order
[**iserver_account_account_id_order_whatif_post**](OrderApi.md#iserver_account_account_id_order_whatif_post) | **Post** /iserver/account/{accountId}/order/whatif | Preview Order
[**iserver_account_account_id_orders_post**](OrderApi.md#iserver_account_account_id_orders_post) | **Post** /iserver/account/{accountId}/orders | Place Orders (Support bracket orders)
[**iserver_account_orders_get**](OrderApi.md#iserver_account_orders_get) | **Get** /iserver/account/orders | Live Orders
[**iserver_reply_replyid_post**](OrderApi.md#iserver_reply_replyid_post) | **Post** /iserver/reply/{replyid} | Place Order Reply


# **iserver_account_account_id_order_orig_customer_order_id_delete**
> Vec<::models::InlineResponse20011> iserver_account_account_id_order_orig_customer_order_id_delete(account_id, orig_customer_order_id)
Delete Order

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| account id | 
  **orig_customer_order_id** | **String**| Customer OrderId | 

### Return type

[**Vec<::models::InlineResponse20011>**](inline_response_200_11.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_account_account_id_order_orig_customer_order_id_post**
> Vec<::models::InlineResponse20011> iserver_account_account_id_order_orig_customer_order_id_post(account_id, orig_customer_order_id, body)
Modify Order

Modifies an open order. The /iserver/accounts endpoint must first be called.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| account id | 
  **orig_customer_order_id** | **String**| Customer OrderId | 
  **body** | [**ModifyOrder**](ModifyOrder.md)| modify-order request | 

### Return type

[**Vec<::models::InlineResponse20011>**](inline_response_200_11.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_account_account_id_order_post**
> Vec<::models::InlineResponse2009> iserver_account_account_id_order_post(account_id, body)
Place Order

Please note here, sometimes this end-point alone can't make sure you submit the order successfully, you could receive some questions in the response, you have to to answer them in order to submit the order successfully. You can use \"/iserver/reply/{replyid}\" end-point to answer questions 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| account id | 
  **body** | [**OrderRequest**](OrderRequest.md)| order request info | 

### Return type

[**Vec<::models::InlineResponse2009>**](inline_response_200_9.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_account_account_id_order_whatif_post**
> ::models::InlineResponse20010 iserver_account_account_id_order_whatif_post(account_id, body)
Preview Order

This end-point allows you to preview order without actually submitting the order and you can get commission information in the response. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| account id | 
  **body** | [**OrderRequest**](OrderRequest.md)| order info | 

### Return type

[**::models::InlineResponse20010**](inline_response_200_10.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_account_account_id_orders_post**
> Vec<::models::InlineResponse2009> iserver_account_account_id_orders_post(account_id, body)
Place Orders (Support bracket orders)

You can pass a list of orders here 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| account id | 
  **body** | [**Body2**](Body2.md)| order request info | 

### Return type

[**Vec<::models::InlineResponse2009>**](inline_response_200_9.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_account_orders_get**
> ::models::InlineResponse2007 iserver_account_orders_get()
Live Orders

The end-point is meant to be used in polling mode, e.g. requesting every x seconds. The response will contain two objects, one is notification, the other is orders.  Orders is the list of orders (cancelled, filled, submitted) with activity in the current day.  Notifications contains information about execute orders as they happen, see status field. 

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse2007**](inline_response_200_7.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_reply_replyid_post**
> Vec<::models::InlineResponse20011> iserver_reply_replyid_post(replyid, body)
Place Order Reply

Reply to questions when placing orders and submit orders

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **replyid** | **String**| Please use the \&quot;id\&quot; from the response of \&quot;Place Order\&quot; end-point | 
  **body** | [**Body3**](Body3.md)| Answer to question | 

### Return type

[**Vec<::models::InlineResponse20011>**](inline_response_200_11.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

