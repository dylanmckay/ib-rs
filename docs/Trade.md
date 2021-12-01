# Trade

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account** | **String** | accountCode | [optional] [default to null]
**acount_code** | **String** | Account Number | [optional] [default to null]
**clearing_id** | **String** | Firm which will settle the trade. For IBExecution customers only. | [optional] [default to null]
**clearing_name** | **String** | Specifies the true beneficiary of the order. For IBExecution customers only. | [optional] [default to null]
**commission** | **f32** | Commission of the order | [optional] [default to null]
**company_name** | **String** | Contracts company name | [optional] [default to null]
**conid** | **String** | IBKR&#39;s contract identifier | [optional] [default to null]
**conidex** | **String** | conid and exchange. Format supports conid or conid@exchange | [optional] [default to null]
**contract_description_1** | **String** | Format contract name | [optional] [default to null]
**exchange** | **String** | Exchange or venue of order | [optional] [default to null]
**execution_id** | **String** | execution identifier for the order | [optional] [default to null]
**liquidation_trade** | **f32** | If order adds liquidity to the market. | [optional] [default to null]
**net_amount** | **f32** | Net cost of the order, including contract multiplier and quantity. | [optional] [default to null]
**order_description** | **String** | Formatted description of the order \&quot;%side% %size% @ %price% on %exchange%\&quot;. | [optional] [default to null]
**order_ref** | **String** | User defined string used to identify the order. Value is set using \&quot;cOID\&quot; field while placing an order. | [optional] [default to null]
**position** | **String** | Total quantity owned for this contract | [optional] [default to null]
**price** | **String** | Average Price | [optional] [default to null]
**sec_type** | **String** | Asset class | [optional] [default to null]
**side** | **String** | The side of the market of the order.   * B - Buy contract near posted ask price   * S - Sell contract near posted bid price   * X - Option expired  | [optional] [default to null]
**size** | **String** | Quantity of the order | [optional] [default to null]
**submitter** | **String** | User that submitted order | [optional] [default to null]
**symbol** | **String** | Underlying Symbol | [optional] [default to null]
**trade_time** | **String** | Time of Status update in format \&quot;YYYYMMDD-hh:mm:ss\&quot;. | [optional] [default to null]
**trade_time_r** | **f32** | Time of status update in format unix time. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


