# AuthStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authenticated** | **bool** | Brokerage session is authenticated | [optional] [default to null]
**competing** | **bool** | Brokerage session is competing, e.g. user is logged in to IBKR Mobile, WebTrader, TWS or other trading platforms. | [optional] [default to null]
**connected** | **bool** | Connected to backend | [optional] [default to null]
**fail** | **String** | Authentication failed, why. | [optional] [default to null]
**message** | **String** | System messages that may affect trading | [optional] [default to null]
**prompts** | **Vec<String>** | Prompt messages that may affect trading or the account | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


