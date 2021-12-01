# SecdefInner

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**all_exchanges** | **String** | List of exchanges and venues contract trades. | [optional] [default to null]
**asset_class** | **String** | Group of financial instruments which have similar financial characteristics and behave similar in the marketplace. | [optional] [default to null]
**chinese_name** | **String** | HTML encoded company description in Chinese. | [optional] [default to null]
**conid** | **i32** | IBKR contract identifier. | [optional] [default to null]
**cross_currency** | **bool** | Defines if a derivative contract has a different currency. | [optional] [default to null]
**currency** | **String** | Currency contract trades in. | [optional] [default to null]
**expiry** | **String** | Specific data contract expires. | [optional] [default to null]
**full_name** | **String** | Formatted company name with underlying symbol, expiration, strike, right. | [optional] [default to null]
**group** | **String** | Potential characteristic of each product. | [optional] [default to null]
**has_options** | **bool** | If contract has an option. | [optional] [default to null]
**increment_rules** | [***::models::SecdefInnerIncrementRules**](secdef_inner_incrementRules.md) |  | [optional] [default to null]
**is_us** | **bool** | If contract is a US contract. Currently supported for stocks, options and warrants. | [optional] [default to null]
**last_trading_day** | **String** | Final day derivative contract can be traded before delivery of the underlying asset or cash settlement. | [optional] [default to null]
**listing_exchange** | **String** | Main trading venue. | [optional] [default to null]
**multiplier** | **i32** | Multiplier for total premium paid or received for derivative contract. | [optional] [default to null]
**name** | **String** | Company Name. | [optional] [default to null]
**put_or_call** | **String** | Defines the right to buy or sell of the underlying security. | [optional] [default to null]
**sector** | **String** | The category of the economy. | [optional] [default to null]
**sector_group** | **String** | Stock Group contract belongs too. | [optional] [default to null]
**strike** | **f32** | Set price at which a derivative contract can be bought or sold. | [optional] [default to null]
**ticker** | **String** | Contract symbol. | [optional] [default to null]
**time** | **i32** |  | [optional] [default to null]
**_type** | **String** | Stock type. | [optional] [default to null]
**und_comp** | **String** | Company name for underlying contract. | [optional] [default to null]
**und_conid** | **i32** | Underlying contract identifier. | [optional] [default to null]
**und_sym** | **String** | IBKR Symbol for underlying contract. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


