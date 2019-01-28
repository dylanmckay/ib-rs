# OrderRequest

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acct_id** | **String** | acctId is optional. It should be one of the accounts returned by /iserver/accounts. If not passed, the first one in the list is selected.  | [optional] [default to null]
**conid** | **i32** | conid is the identifier of the security you want to trade, you can find the conid with /iserver/secdef/search.  | [optional] [default to null]
**sec_type** | **String** | conid:type for example 265598:STK | [optional] [default to null]
**c_oid** | **String** | cOID: Is the customer identifier. Can be some arbitrary string. e.g \&quot;my-fb-order\&quot;.  | [optional] [default to null]
**order_type** | **String** | orderType can be one of MKT (Market), LMT (Limit), STP (Stop) or STP_LIMIT (stop limit)  | [optional] [default to null]
**listing_exchange** | **String** | listingExchange is optional. By default we use \&quot;SMART\&quot; routing. Possible values are available via this end point: /v1/portal/iserver/contract/{{conid}}/info, see valid_exchange: e.g: SMART,AMEX,NYSE, CBOE,ISE,CHX,ARCA,ISLAND,DRCTEDGE,BEX,BATS,EDGEA,CSFBALGO,JE FFALGO,BYX,IEX,FOXRIVER,TPLUS1,NYSENAT,PSX  | [optional] [default to null]
**outside_rth** | **bool** | set to true if the order can be executed outside regular trading hours.  | [optional] [default to null]
**price** | **f32** | optional if order is MKT, for LMT, this is the limit price. For STP this is the stop price.  | [optional] [default to null]
**side** | **String** | SELL or BUY | [optional] [default to null]
**ticker** | **String** |  | [optional] [default to null]
**tif** | **String** | GTC (Good Till Cancel) or DAY. DAY orders are automatically cancelled at the end of the Day or Trading hours.  | [optional] [default to null]
**referrer** | **String** | for example QuickTrade | [optional] [default to null]
**quantity** | **f32** | usually integer, for some special cases can be float numbers | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


