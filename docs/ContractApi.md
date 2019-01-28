# \ContractApi

All URIs are relative to *https://localhost:5000/v1/portal*

Method | HTTP request | Description
------------- | ------------- | -------------
[**iserver_contract_conid_info_get**](ContractApi.md#iserver_contract_conid_info_get) | **Get** /iserver/contract/{conid}/info | Contract Info
[**iserver_secdef_search_post**](ContractApi.md#iserver_secdef_search_post) | **Post** /iserver/secdef/search | Search by symbol or name
[**trsrv_futures_get**](ContractApi.md#trsrv_futures_get) | **Get** /trsrv/futures | Security Futures by Symbol
[**trsrv_secdef_post**](ContractApi.md#trsrv_secdef_post) | **Post** /trsrv/secdef | Secdef by Conid


# **iserver_contract_conid_info_get**
> ::models::Contract iserver_contract_conid_info_get(conid)
Contract Info

get contract details, you can use this to prefill your order before you submit an order

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **conid** | **String**| contract id | 

### Return type

[**::models::Contract**](contract.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_secdef_search_post**
> Vec<::models::InlineResponse2008> iserver_secdef_search_post(symbol)
Search by symbol or name

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **symbol** | [**Symbol**](Symbol.md)| symbol or name to be searched | 

### Return type

[**Vec<::models::InlineResponse2008>**](inline_response_200_8.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **trsrv_futures_get**
> ::models::InlineResponse20015 trsrv_futures_get(symbols)
Security Futures by Symbol

Returns a list of non-expired future contracts for given symbol(s)

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **symbols** | **String**| list of case-sensitive symbols separated by comma | 

### Return type

[**::models::InlineResponse20015**](inline_response_200_15.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **trsrv_secdef_post**
> ::models::Secdef trsrv_secdef_post(body)
Secdef by Conid

Returns a list of security definitions for the given conids

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**Body4**](Body4.md)| request body | 

### Return type

[**::models::Secdef**](secdef.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

