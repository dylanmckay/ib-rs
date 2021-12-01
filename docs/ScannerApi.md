# \ScannerApi

All URIs are relative to *https://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**iserver_scanner_params_get**](ScannerApi.md#iserver_scanner_params_get) | **Get** /iserver/scanner/params | Scanner Parameters
[**iserver_scanner_run_post**](ScannerApi.md#iserver_scanner_run_post) | **Post** /iserver/scanner/run | run scanner to get a list of contracts


# **iserver_scanner_params_get**
> ::models::InlineResponse20027 iserver_scanner_params_get()
Scanner Parameters

Returns an object contains four lists contain all parameters for scanners

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20027**](inline_response_200_27.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_scanner_run_post**
> Vec<::models::InlineResponse20028> iserver_scanner_run_post(body)
run scanner to get a list of contracts

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**ScannerParams**](ScannerParams.md)| scanner-params request | 

### Return type

[**Vec<::models::InlineResponse20028>**](inline_response_200_28.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

