# \AccountApi

All URIs are relative to *https://localhost:5000/v1/portal*

Method | HTTP request | Description
------------- | ------------- | -------------
[**iserver_account_post**](AccountApi.md#iserver_account_post) | **Post** /iserver/account | Updates currently selected account to the provided account
[**iserver_accounts_get**](AccountApi.md#iserver_accounts_get) | **Get** /iserver/accounts | Brokerage Accounts
[**portfolio_account_id_ledger_get**](AccountApi.md#portfolio_account_id_ledger_get) | **Get** /portfolio/{accountId}/ledger | Account Ledger
[**portfolio_account_id_meta_get**](AccountApi.md#portfolio_account_id_meta_get) | **Get** /portfolio/{accountId}/meta | Account Information
[**portfolio_account_id_summary_get**](AccountApi.md#portfolio_account_id_summary_get) | **Get** /portfolio/{accountId}/summary | Account Summary
[**portfolio_accounts_get**](AccountApi.md#portfolio_accounts_get) | **Get** /portfolio/accounts | Portfolio Accounts
[**portfolio_subaccounts_get**](AccountApi.md#portfolio_subaccounts_get) | **Get** /portfolio/subaccounts | List of Sub-Accounts


# **iserver_account_post**
> ::models::InlineResponse2001 iserver_account_post(body)
Updates currently selected account to the provided account

If an user has multiple accounts, and user wants to get orders, trades, etc. of an account other than currently selected account, then user can update the currently selected account using this API and then can fetch required information for the newly updated account.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**SetAccount**](SetAccount.md)| account id | 

### Return type

[**::models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **iserver_accounts_get**
> ::models::InlineResponse200 iserver_accounts_get()
Brokerage Accounts

Returns a list of accounts the user has trading access to, their respective aliases and the currently selected account.

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

# **portfolio_account_id_ledger_get**
> ::models::InlineResponse20013 portfolio_account_id_ledger_get(account_id)
Account Ledger

Information regarding settled cash, cash balances, etc. in the account's base currency and any other cash balances hold in other currencies. The list of supported currencies is available at https://www.interactivebrokers.com/en/index.php?f=3185.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| account id | 

### Return type

[**::models::InlineResponse20013**](inline_response_200_13.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **portfolio_account_id_meta_get**
> ::models::Accounts portfolio_account_id_meta_get(account_id)
Account Information

Account information related to account Id

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

# **portfolio_account_id_summary_get**
> ::models::InlineResponse20012 portfolio_account_id_summary_get(account_id)
Account Summary

Returns information about margin, cash balances and other information related to your account. See also /portfolio/{accountId}/ledger.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| account id | 

### Return type

[**::models::InlineResponse20012**](inline_response_200_12.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **portfolio_accounts_get**
> ::models::Accounts portfolio_accounts_get()
Portfolio Accounts

Returns a list of accounts for which the user has access to and can view portfolio positions and portfolio related information. For accounts the user has trading access to, see /iserver/accounts.

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

# **portfolio_subaccounts_get**
> ::models::Account portfolio_subaccounts_get()
List of Sub-Accounts

Returns a list of sub-accounts for which the user has access to and can view portfolio positions and portfolio related information. For accounts the user has trading access to, see /iserver/accounts.

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

