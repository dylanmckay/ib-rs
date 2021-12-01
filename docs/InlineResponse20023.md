# InlineResponse20023

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_sell_long** | **bool** | Allowed to sell shares that you own | [optional] [default to null]
**cfi_code** | **String** | Classification of Financial Instrument codes | [optional] [default to null]
**classifier** | **String** |  | [optional] [default to null]
**company_name** | **String** | Contracts company name | [optional] [default to null]
**con_id** | **f32** | IBKRs contract identifier | [optional] [default to null]
**contract_month** | **String** | Month the contract must be satisfied by making or accepting delivery | [optional] [default to null]
**currency** | **String** | Currency contract trades in | [optional] [default to null]
**cusip** | **String** |  | [optional] [default to null]
**exchange** | **String** | Primary Exchange, Routing or Trading Venue | [optional] [default to null]
**expiry_full** | **f32** | Expiration Date in the format YYYYMMDD | [optional] [default to null]
**industry** | **String** | Specific group of companies or businesses. | [optional] [default to null]
**instrument_type** | **String** | Asset Class of the contract | [optional] [default to null]
**is_zero_commission_security** | **bool** | Supports zero commission trades | [optional] [default to null]
**local_symbol** | **String** | Contracts symbol from primary exchange. For options it is the OCC symbol. | [optional] [default to null]
**maturity_date** | **f32** | Date on which the underlying transaction settles if the option is exercised | [optional] [default to null]
**multiplier** | **String** | numerical value of each point of price movement | [optional] [default to null]
**r_t_h** | **bool** | Provides trading outside of Regular Trading Hours | [optional] [default to null]
**right** | **String** | Put or Call of the option | [optional] [default to null]
**rules** | [**Vec<::models::InlineResponse20023Rules>**](inline_response_200_23_rules.md) |  | [optional] [default to null]
**smart_available** | **bool** | Support IBKRs SMART routing | [optional] [default to null]
**strike** | **String** | fixed price at which the owner of the option buys or sells the underlying | [optional] [default to null]
**symbol** | **String** | Underlying symbol | [optional] [default to null]
**text** | **String** | Formatted contract parameters | [optional] [default to null]
**trading_class** | **String** | Designation of the contract | [optional] [default to null]
**underlying_con_id** | **f32** | IBKRs contract identifier for the underlying instrument | [optional] [default to null]
**underlying_issuer** | **String** | Legal entity for underlying contract | [optional] [default to null]
**valid_exchanges** | **String** | Comma separated list of exchanges or trading venues | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


