# \FYIApi

All URIs are relative to *https://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fyi_deliveryoptions_device_id_delete**](FYIApi.md#fyi_deliveryoptions_device_id_delete) | **Delete** /fyi/deliveryoptions/{deviceId} | Delete a device
[**fyi_deliveryoptions_device_post**](FYIApi.md#fyi_deliveryoptions_device_post) | **Post** /fyi/deliveryoptions/device | Enable/Disable device option
[**fyi_deliveryoptions_email_put**](FYIApi.md#fyi_deliveryoptions_email_put) | **Put** /fyi/deliveryoptions/email | Enable/Disable email option
[**fyi_deliveryoptions_get**](FYIApi.md#fyi_deliveryoptions_get) | **Get** /fyi/deliveryoptions | Get delivery options
[**fyi_disclaimer_typecode_get**](FYIApi.md#fyi_disclaimer_typecode_get) | **Get** /fyi/disclaimer/{typecode} | Get disclaimer for a certain kind of fyi
[**fyi_disclaimer_typecode_put**](FYIApi.md#fyi_disclaimer_typecode_put) | **Put** /fyi/disclaimer/{typecode} | Mark disclaimer read
[**fyi_notifications_get**](FYIApi.md#fyi_notifications_get) | **Get** /fyi/notifications | Get a list of notifications
[**fyi_notifications_more_get**](FYIApi.md#fyi_notifications_more_get) | **Get** /fyi/notifications/more | Get more notifications based on a certain one
[**fyi_notifications_notification_id_put**](FYIApi.md#fyi_notifications_notification_id_put) | **Put** /fyi/notifications/{notificationId} | Get a list of notifications
[**fyi_settings_get**](FYIApi.md#fyi_settings_get) | **Get** /fyi/settings | Get a list of subscriptions
[**fyi_settings_typecode_post**](FYIApi.md#fyi_settings_typecode_post) | **Post** /fyi/settings/{typecode} | Enable/Disable certain subscription
[**fyi_unreadnumber_get**](FYIApi.md#fyi_unreadnumber_get) | **Get** /fyi/unreadnumber | Get unread number of fyis. The HTTP method POST is also supported.


# **fyi_deliveryoptions_device_id_delete**
> Value fyi_deliveryoptions_device_id_delete(device_id)
Delete a device

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **device_id** | **String**| device ID | 

### Return type

[**Value**](Value.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **fyi_deliveryoptions_device_post**
> ::models::InlineResponse2006 fyi_deliveryoptions_device_post(body)
Enable/Disable device option

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**Body**](Body.md)| device info | 

### Return type

[**::models::InlineResponse2006**](inline_response_200_6.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **fyi_deliveryoptions_email_put**
> ::models::InlineResponse2006 fyi_deliveryoptions_email_put(enabled)
Enable/Disable email option

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **enabled** | **String**| true/false | 

### Return type

[**::models::InlineResponse2006**](inline_response_200_6.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **fyi_deliveryoptions_get**
> ::models::InlineResponse2005 fyi_deliveryoptions_get()
Get delivery options

options for sending fyis to email and other devices 

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse2005**](inline_response_200_5.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **fyi_disclaimer_typecode_get**
> ::models::InlineResponse2007 fyi_disclaimer_typecode_get(typecode)
Get disclaimer for a certain kind of fyi

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **typecode** | **String**| fyi code, for example --M8, EA | 

### Return type

[**::models::InlineResponse2007**](inline_response_200_7.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **fyi_disclaimer_typecode_put**
> ::models::InlineResponse2006 fyi_disclaimer_typecode_put(typecode)
Mark disclaimer read

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **typecode** | **String**| fyi code, for example --M8, EA | 

### Return type

[**::models::InlineResponse2006**](inline_response_200_6.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **fyi_notifications_get**
> ::models::Notifications fyi_notifications_get(max, optional)
Get a list of notifications

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **max** | **String**| max number of fyis in response | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **max** | **String**| max number of fyis in response | 
 **exclude** | **String**| if set, don&#39;t set include | 
 **include** | **String**| if set, don&#39;t set exclude | 

### Return type

[**::models::Notifications**](notifications.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **fyi_notifications_more_get**
> ::models::Notifications fyi_notifications_more_get(id)
Get more notifications based on a certain one

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| id of last notification in the list | 

### Return type

[**::models::Notifications**](notifications.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **fyi_notifications_notification_id_put**
> Value fyi_notifications_notification_id_put(notification_id)
Get a list of notifications

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **notification_id** | **String**| mark a notification read | 

### Return type

[**Value**](Value.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **fyi_settings_get**
> Vec<::models::InlineResponse2008> fyi_settings_get()
Get a list of subscriptions

Return the current choices of subscriptions, we can toggle the option 

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**Vec<::models::InlineResponse2008>**](inline_response_200_8.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **fyi_settings_typecode_post**
> Value fyi_settings_typecode_post(typecode, body)
Enable/Disable certain subscription

Configure which typecode you would like to enable/disable. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **typecode** | **String**| fyi code | 
  **body** | [**Body1**](Body1.md)|  | 

### Return type

[**Value**](Value.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **fyi_unreadnumber_get**
> ::models::InlineResponse2009 fyi_unreadnumber_get()
Get unread number of fyis. The HTTP method POST is also supported.

Returns the total number of unread fyis 

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse2009**](inline_response_200_9.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

