# \AlertApi

All URIs are relative to *https://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**iserver_account_account_id_alert_post**](AlertApi.md#iserver_account_account_id_alert_post) | **Post** /iserver/account/{accountId}/alert | Create or modify alert
[**iserver_account_alertid_get**](AlertApi.md#iserver_account_alertid_get) | **Get** /iserver/account/alert/:id | Get details of an alert
[**iserver_account_mta_get**](AlertApi.md#iserver_account_mta_get) | **Get** /iserver/account/mta | Get MTA alert
[**iserver_accountaccount_id_alert_activate_post**](AlertApi.md#iserver_accountaccount_id_alert_activate_post) | **Post** /iserver/account/:accountId/alert/activate | Activate or deactivate an alert
[**iserver_accountaccount_id_alertalert_id_delete**](AlertApi.md#iserver_accountaccount_id_alertalert_id_delete) | **Delete** /iserver/account/:accountId/alert/:alertId | Delete an alert
[**iserver_accountaccount_id_alerts_get**](AlertApi.md#iserver_accountaccount_id_alerts_get) | **Get** /iserver/account/:accountId/alerts | Get a list of available alerts


# **iserver_account_account_id_alert_post**
> ::models::InlineResponse20017 iserver_account_account_id_alert_post(account_id, body)
Create or modify alert

Please note here, DO NOT pass orderId when creating a new alert, toolId is only required for MTA alert 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| account id | 
  **body** | [**AlertRequest**](AlertRequest.md)| alert info | 

### Return type

[**::models::InlineResponse20017**](inline_response_200_17.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_account_alertid_get**
> ::models::AlertResponse iserver_account_alertid_get(id)
Get details of an alert

Use the endpoint /iserver/account/:accountId/alerts to receive the alert id. The order_id in the response is the alert id. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| alert id | 

### Return type

[**::models::AlertResponse**](alert-response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_account_mta_get**
> ::models::AlertResponse iserver_account_mta_get()
Get MTA alert

Each login user only has one mobile trading assistant (MTA) alert with it's own unique tool id. The tool id cannot be changed. When modified a new order Id is generated. MTA alerts can not be created or deleted. If you call delete /iserver/account/:accountId/alert/:alertId, it will reset MTA to default. See [here](https://www.interactivebrokers.com/en/software/mobileiphonemobile/mobileiphone.htm#monitor/trading-assistant.htm) for more information on MTA alerts. 

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::AlertResponse**](alert-response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_accountaccount_id_alert_activate_post**
> ::models::InlineResponse20012 iserver_accountaccount_id_alert_activate_post(account_id, body)
Activate or deactivate an alert

Please note, if alertId is 0, it will activate/deactivate all alerts

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| account id | 
  **body** | [**Body2**](Body2.md)| order request info | 

### Return type

[**::models::InlineResponse20012**](inline_response_200_12.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_accountaccount_id_alertalert_id_delete**
> ::models::InlineResponse20012 iserver_accountaccount_id_alertalert_id_delete(account_id, alert_id)
Delete an alert

Please be careful, if alertId is 0, it will delete all alerts

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| account id | 
  **alert_id** | **String**| alert id | 

### Return type

[**::models::InlineResponse20012**](inline_response_200_12.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_accountaccount_id_alerts_get**
> Vec<::models::InlineResponse20013> iserver_accountaccount_id_alerts_get(account_id)
Get a list of available alerts

The response will contain both active and inactive alerts, but it won't have MTA alert

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| account id | 

### Return type

[**Vec<::models::InlineResponse20013>**](inline_response_200_13.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

