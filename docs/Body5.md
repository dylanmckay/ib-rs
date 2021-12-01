# Body5

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**orders** | [**Vec<::models::OrderRequest>**](order-request.md) | Notes for bracket orders: 1. Children orders will not have its own \&quot;cOID\&quot;, so please donot pass \&quot;cOID\&quot; parameter in child order.Instead, they will have a \&quot;parentId\&quot; which must be equal to \&quot;cOID\&quot; of parent. 2. When you cancel a parent order, it will cancel all bracket orders, when you cancel one child order, it will also cancel its sibling order.  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


