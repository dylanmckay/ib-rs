# OrderRequest

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acct_id** | **String** | acctId is optional. It should be one of the accounts returned by /iserver/accounts. If not passed, the first one in the list is selected.  | [optional] [default to null]
**allocation_method** | **String** | Set the allocation method when placing an order using an FA account for a group Possible allocation methods are \&quot;NetLiquidity\&quot;, \&quot;AvailableEquity\&quot;, \&quot;EqualQuantity\&quot; and \&quot;PctChange\&quot;.  | [optional] [default to null]
**aux_price** | [***Value**](Value.md) | optional if order is STOP_LIMIT, this is the stop price. You must specify both price and auxPrice for STOP_LIMIT orders.  | [optional] [default to null]
**c_oid** | **String** | Customer Order ID. An arbitrary string that can be used to identify the order, e.g \&quot;my-fb-order\&quot;. The value must be unique for a 24h span. Please do not set this value for child orders when placing a bracket order.  | [optional] [default to null]
**conid** | **i32** | conid is the identifier of the security you want to trade, you can find the conid with /iserver/secdef/search.  | [optional] [default to null]
**fx_qty** | **f32** | double number, this is the cash quantity field which can only be used for FX conversion order. When using &#39;fxQty&#39; you don&#39;t need to specify &#39;quantity&#39;.  | [optional] [default to null]
**is_ccy_conv** | **bool** | set to true if the order is a FX conversion order  | [optional] [default to null]
**is_single_group** | **bool** | set to true if you want to place a single group orders(OCA)  | [optional] [default to null]
**listing_exchange** | **String** | listingExchange is optional. By default we use \&quot;SMART\&quot; routing. Possible values are available via this end point: /v1/portal/iserver/contract/{conid}/info, see valid_exchange: e.g: SMART,AMEX,NYSE, CBOE,ISE,CHX,ARCA,ISLAND,DRCTEDGE,BEX,BATS,EDGEA,CSFBALGO,JE FFALGO,BYX,IEX,FOXRIVER,TPLUS1,NYSENAT,PSX  | [optional] [default to null]
**order_type** | **String** | The order-type determines what type of order you want to send. LMT - A limit order is an order to buy or sell at the specified price or better. MKT - A market order is an order to buy or sell at the markets current NBBO. STP - A stop order becomes a market order once the specified stop price is attained or penetrated. STOP_LIMIT - A stop limit order becomes a limit order once the specified stop price is attained or penetrated. MIDPRICE - A MidPrice order attempts to fill at the current midpoint of the NBBO or better.  | [optional] [default to null]
**outside_rth** | **bool** | set to true if the order can be executed outside regular trading hours.  | [optional] [default to null]
**parent_id** | **String** | Only specify for child orders when placing bracket orders. The parentId for the child order(s) must be equal to the cOId (customer order id) of the parent.  | [optional] [default to null]
**price** | **f32** | optional if order is LMT, or STOP_LIMIT, this is the limit price. For STP this is the stop price. For MIDPRICE this is the option price cap.  | [optional] [default to null]
**quantity** | **f32** | usually integer, for some special cases can be float numbers | [optional] [default to null]
**referrer** | **String** | Custom order reference  | [optional] [default to null]
**sec_type** | **String** | The contract-identifier (conid) and security type (type) specified as a concatenated value, conid:type | [optional] [default to null]
**side** | **String** | SELL or BUY | [optional] [default to null]
**strategy** | **String** | Specify which IB Algo algorithm to use for this order.  | [optional] [default to null]
**strategy_parameters** | [***Value**](Value.md) | The IB Algo parameters for the specified algorithm.  | [optional] [default to null]
**ticker** | **String** | This is the  underlying symbol for the contract.  | [optional] [default to null]
**tif** | **String** | The Time-In-Force determines how long the order remains active on the market.   * GTC - use Good-Till-Cancel for orders to remain active until it executes or cancelled.   * OPG - use Open-Price-Guarantee for Limit-On-Open (LOO) or Market-On-Open (MOO) orders.   * DAY - if not executed a Day order will automatically cancel at the end of the markets regular trading hours.   * IOC - any portion of an Immediate-or-Cancel order that is not filled as soon as it becomes available in the market is cancelled.  | [optional] [default to null]
**use_adaptive** | **bool** | If true, the system will use the Price Management Algo to submit the order. https://www.interactivebrokers.com/en/index.php?f&#x3D;43423  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


