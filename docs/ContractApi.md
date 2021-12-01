# \ContractApi

All URIs are relative to *https://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**iserver_contract_conid_algos_get**](ContractApi.md#iserver_contract_conid_algos_get) | **Get** /iserver/contract/{conid}/algos | IB Algo Params
[**iserver_contract_conid_info_and_rules_get**](ContractApi.md#iserver_contract_conid_info_and_rules_get) | **Get** /iserver/contract/{conid}/info-and-rules | Info and Rules
[**iserver_contract_conid_info_get**](ContractApi.md#iserver_contract_conid_info_get) | **Get** /iserver/contract/{conid}/info | Contract Details
[**iserver_secdef_info_get**](ContractApi.md#iserver_secdef_info_get) | **Get** /iserver/secdef/info | Secdef Info
[**iserver_secdef_search_post**](ContractApi.md#iserver_secdef_search_post) | **Post** /iserver/secdef/search | Search by Symbol or Name
[**iserver_secdef_strikes_get**](ContractApi.md#iserver_secdef_strikes_get) | **Get** /iserver/secdef/strikes | Search Strikes
[**trsrv_futures_get**](ContractApi.md#trsrv_futures_get) | **Get** /trsrv/futures | Security Futures by Symbol
[**trsrv_secdef_post**](ContractApi.md#trsrv_secdef_post) | **Post** /trsrv/secdef | Secdef by Conid
[**trsrv_secdef_schedule_get**](ContractApi.md#trsrv_secdef_schedule_get) | **Get** /trsrv/secdef/schedule | Get trading schedule for symbol
[**trsrv_stocks_get**](ContractApi.md#trsrv_stocks_get) | **Get** /trsrv/stocks | Security Stocks by Symbol


# **iserver_contract_conid_algos_get**
> Vec<::models::InlineResponse20022> iserver_contract_conid_algos_get(conid, optional)
IB Algo Params

Returns supported IB Algos for contract. Must be called a second time to query the list of available parameters.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **conid** | **String**| IBKR contract identifier | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **conid** | **String**| IBKR contract identifier | 
 **algos** | **String**| List of algo ids delimited by \&quot;;\&quot; to filter by. Max of 8 algos ids can be specified. | 
 **add_description** | **String**| Whether or not to add algo descriptions to response. Set to 1 for yes, 0 for no. | 
 **add_params** | **String**| Whether or not to show algo parameters.  Set to 1 for yes, 0 for no. | 

### Return type

[**Vec<::models::InlineResponse20022>**](inline_response_200_22.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_contract_conid_info_and_rules_get**
> ::models::InlineResponse20023 iserver_contract_conid_info_and_rules_get(conid, is_buy)
Info and Rules

Returns trading related rules and info for contract

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **conid** | **String**| IBKR contract identifier | 
  **is_buy** | **bool**| Side of the market rules apply too. Set to true for Buy Orders, set to false for Sell Orders | 

### Return type

[**::models::InlineResponse20023**](inline_response_200_23.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_contract_conid_info_get**
> ::models::Contract iserver_contract_conid_info_get(conid)
Contract Details

Using the Contract Identifier get contract info. You can use this to prefill your order before you submit an order

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

# **iserver_secdef_info_get**
> Value iserver_secdef_info_get(conid, sectype, optional)
Secdef Info

Provides Contract Details of Futures, Options, Warrants, Cash and CFDs based on conid. To get the strike price for Options/Warrants use \"/iserver/secdef/strikes\" endpoint. Must call /secdef/search for the underlying contract first.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **conid** | **String**| underlying contract id | 
  **sectype** | **String**| FUT/OPT/WAR/CASH/CFD | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **conid** | **String**| underlying contract id | 
 **sectype** | **String**| FUT/OPT/WAR/CASH/CFD | 
 **month** | **String**| contract month, only required for FUT/OPT/WAR in the format MMMYY, example JAN00 | 
 **exchange** | **String**| optional, default is SMART | 
 **strike** | **String**| optional, only required for OPT/WAR | 
 **right** | **String**| C for call, P for put | 

### Return type

[**Value**](Value.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_secdef_search_post**
> Vec<::models::InlineResponse20029> iserver_secdef_search_post(symbol)
Search by Symbol or Name

Search by underlying symbol or company name. Relays back what derivative contract(s) it has. This endpoint must be called before using /secdef/info. If company name is specified will only receive limited response: conid, companyName, companyHeader and symbol. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **symbol** | [**Symbol**](Symbol.md)| Symbol or Company Name to be searched | 

### Return type

[**Vec<::models::InlineResponse20029>**](inline_response_200_29.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_secdef_strikes_get**
> ::models::InlineResponse20030 iserver_secdef_strikes_get(conid, sectype, month, optional)
Search Strikes

Query strikes for Options/Warrants. For the conid of the underlying contract, available contract months and exchanges use \"/iserver/secdef/search\"

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **conid** | **String**| contract id of the underlying contract | 
  **sectype** | **String**| OPT/WAR | 
  **month** | **String**| contract month | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **conid** | **String**| contract id of the underlying contract | 
 **sectype** | **String**| OPT/WAR | 
 **month** | **String**| contract month | 
 **exchange** | **String**| optional, default is SMART | 

### Return type

[**::models::InlineResponse20030**](inline_response_200_30.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **trsrv_futures_get**
> ::models::InlineResponse20036 trsrv_futures_get(symbols)
Security Futures by Symbol

Returns a list of non-expired future contracts for given symbol(s)

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **symbols** | **String**| list of case-sensitive symbols separated by comma | 

### Return type

[**::models::InlineResponse20036**](inline_response_200_36.md)

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
  **body** | [**Body11**](Body11.md)| request body | 

### Return type

[**::models::Secdef**](secdef.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **trsrv_secdef_schedule_get**
> ::models::InlineResponse20037 trsrv_secdef_schedule_get(asset_class, symbol, optional)
Get trading schedule for symbol

Returns the trading schedule up to a month for the requested contract

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **asset_class** | **String**| specify the asset class of the contract. Available values-- Stock: STK, Option: OPT, Future: FUT, Contract For Difference: CFD, Warrant: WAR, Forex: SWP, Mutual Fund: FND, Bond: BND, Inter-Commodity Spreads: ICS  | 
  **symbol** | **String**| Underlying Symbol for specified contract, for example &#39;AAPL&#39; for US Stock - Apple Inc. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **asset_class** | **String**| specify the asset class of the contract. Available values-- Stock: STK, Option: OPT, Future: FUT, Contract For Difference: CFD, Warrant: WAR, Forex: SWP, Mutual Fund: FND, Bond: BND, Inter-Commodity Spreads: ICS  | 
 **symbol** | **String**| Underlying Symbol for specified contract, for example &#39;AAPL&#39; for US Stock - Apple Inc. | 
 **exchange** | **String**| Native exchange for contract, for example &#39;NASDAQ&#39; for US Stock - Apple Inc. | 
 **exchange_filter** | **String**| Response only returns trading schedule for specified exchange | 

### Return type

[**::models::InlineResponse20037**](inline_response_200_37.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **trsrv_stocks_get**
> ::models::InlineResponse20038 trsrv_stocks_get(symbols)
Security Stocks by Symbol

Returns an object contains all stock contracts for given symbol(s)

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **symbols** | **String**| list of upper-sensitive symbols separated by comma | 

### Return type

[**::models::InlineResponse20038**](inline_response_200_38.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

