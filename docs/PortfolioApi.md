# \PortfolioApi

All URIs are relative to *https://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**portfolio_account_id_allocation_get**](PortfolioApi.md#portfolio_account_id_allocation_get) | **Get** /portfolio/{accountId}/allocation | Account Allocation
[**portfolio_account_id_ledger_get**](PortfolioApi.md#portfolio_account_id_ledger_get) | **Get** /portfolio/{accountId}/ledger | Account Ledger
[**portfolio_account_id_meta_get**](PortfolioApi.md#portfolio_account_id_meta_get) | **Get** /portfolio/{accountId}/meta | Account Information
[**portfolio_account_id_position_conid_get**](PortfolioApi.md#portfolio_account_id_position_conid_get) | **Get** /portfolio/{accountId}/position/{conid} | Position by Conid
[**portfolio_account_id_positions_invalidate_post**](PortfolioApi.md#portfolio_account_id_positions_invalidate_post) | **Post** /portfolio/{accountId}/positions/invalidate | Invalidates the backend cache of the Portfolio
[**portfolio_account_id_positions_page_id_get**](PortfolioApi.md#portfolio_account_id_positions_page_id_get) | **Get** /portfolio/{accountId}/positions/{pageId} | Portfolio Positions
[**portfolio_account_id_summary_get**](PortfolioApi.md#portfolio_account_id_summary_get) | **Get** /portfolio/{accountId}/summary | Account Summary
[**portfolio_accounts_get**](PortfolioApi.md#portfolio_accounts_get) | **Get** /portfolio/accounts | Portfolio Accounts
[**portfolio_allocation_post**](PortfolioApi.md#portfolio_allocation_post) | **Post** /portfolio/allocation | Account Alloction (All Accounts)
[**portfolio_positions_conid_get**](PortfolioApi.md#portfolio_positions_conid_get) | **Get** /portfolio/positions/{conid} | Positions by Conid
[**portfolio_subaccounts_get**](PortfolioApi.md#portfolio_subaccounts_get) | **Get** /portfolio/subaccounts | List of Sub-Accounts


# **portfolio_account_id_allocation_get**
> ::models::Allocation portfolio_account_id_allocation_get(account_id)
Account Allocation

Information about the account's portfolio allocation by Asset Class, Industry and Category.  /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| account id | 

### Return type

[**::models::Allocation**](allocation.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **portfolio_account_id_ledger_get**
> ::models::InlineResponse20033 portfolio_account_id_ledger_get(account_id)
Account Ledger

Information regarding settled cash, cash balances, etc. in the account's base currency and any other cash balances hold in other currencies.  /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint. The list of supported currencies is available at https://www.interactivebrokers.com/en/index.php?f=3185.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| account id | 

### Return type

[**::models::InlineResponse20033**](inline_response_200_33.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **portfolio_account_id_meta_get**
> ::models::Accounts portfolio_account_id_meta_get(account_id)
Account Information

Account information related to account Id /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| account id | 

### Return type

[**::models::Accounts**](accounts.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **portfolio_account_id_position_conid_get**
> ::models::Position portfolio_account_id_position_conid_get(account_id, conid)
Position by Conid

Returns a list of all positions matching the conid. For portfolio models the conid could be in more than one model, returning an array with the name of the model it belongs to.  /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| account id | 
  **conid** | **i32**| contract id | 

### Return type

[**::models::Position**](position.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **portfolio_account_id_positions_invalidate_post**
> Value portfolio_account_id_positions_invalidate_post(account_id)
Invalidates the backend cache of the Portfolio

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| account id | 

### Return type

[**Value**](Value.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **portfolio_account_id_positions_page_id_get**
> ::models::Position portfolio_account_id_positions_page_id_get(account_id, page_id, optional)
Portfolio Positions

Returns a list of positions for the given account. The endpoint supports paging, page's default size is 30 positions. /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| account id | 
  **page_id** | **String**| page id | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **account_id** | **String**| account id | 
 **page_id** | **String**| page id | 
 **model** | **String**| optional | 
 **sort** | **String**| declare the table to be sorted by which column | 
 **direction** | **String**| in which order, a means ascending - d means descending | 
 **period** | **String**| period for pnl column, can be 1D, 7D, 1M... | 

### Return type

[**::models::Position**](position.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **portfolio_account_id_summary_get**
> ::models::InlineResponse20034 portfolio_account_id_summary_get(account_id)
Account Summary

Returns information about margin, cash balances and other information related to specified account. See also /portfolio/{accountId}/ledger. /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| account id | 

### Return type

[**::models::InlineResponse20034**](inline_response_200_34.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **portfolio_accounts_get**
> ::models::Accounts portfolio_accounts_get()
Portfolio Accounts

In non-tiered account structures, returns a list of accounts for which the user can view position and account information. This endpoint must be called prior  to calling other /portfolio endpoints for those accounts. For querying a list of accounts  which the user can trade, see /iserver/accounts. For a list of subaccounts in tiered  account structures (e.g. financial advisor or ibroker accounts) see /portfolio/subaccounts.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::Accounts**](accounts.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **portfolio_allocation_post**
> ::models::Allocation portfolio_allocation_post(body)
Account Alloction (All Accounts)

Similar to /portfolio/{accountId}/allocation but returns a consolidated view of of all the accounts returned by /portfolio/accounts. /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**Body10**](Body10.md)| accounts info | 

### Return type

[**::models::Allocation**](allocation.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **portfolio_positions_conid_get**
> ::models::InlineResponse20032 portfolio_positions_conid_get(conid)
Positions by Conid

Returns an object of all positions matching the conid for all the selected accounts. For portfolio models the conid could be in more than one model, returning an array with the name of the model it belongs to. /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **conid** | **i32**| contract id | 

### Return type

[**::models::InlineResponse20032**](inline_response_200_32.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **portfolio_subaccounts_get**
> ::models::Account portfolio_subaccounts_get()
List of Sub-Accounts

Used in tiered account structures (such as financial advisor and ibroker accounts)  to return a list of sub-accounts for which the user can view position and  account-related information. This endpoint must be called prior to calling other  /portfolio endpoints for those subaccounts.  To query a list of accounts the user can trade, see /iserver/accounts.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::Account**](account.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

