# AlertRequest

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alert_message** | **String** | The message you want to receive via email or text message | [optional] [default to null]
**alert_name** | **String** | name of alert | [optional] [default to null]
**alert_repeatable** | **i32** | whether alert is repeatable or not, so value can only be 0 or 1, this has to be 1 for MTA alert | [optional] [default to null]
**conditions** | [**Vec<::models::AlertrequestConditions>**](alertrequest_conditions.md) |  | [optional] [default to null]
**email** | **String** | email address to receive alert | [optional] [default to null]
**expire_time** | **String** | format, YYYYMMDD-HH:mm:ss, please NOTE this will only work when tif is GTD  | [optional] [default to null]
**i_tws_orders_only** | **i32** | value can only be 0 or 1, set to 1 to enable the alert only in IBKR mobile  | [optional] [default to null]
**order_id** | **i32** | orderId is required when modifying alert. You can get it from /iserver/account/:accountId/alerts  | [optional] [default to null]
**outside_rth** | **i32** | value can only be 0 or 1, set to 1 if the alert can be triggered outside regular trading hours.  | [optional] [default to null]
**play_audio** | **String** | audio message to play when alert is triggered | [optional] [default to null]
**send_message** | **i32** | whether allowing to send email or not, so value can only be 0 or 1,  | [optional] [default to null]
**show_popup** | **i32** | value can only be 0 or 1, set to 1 to allow to show alert in pop-ups | [optional] [default to null]
**tif** | **String** | time in force, can only be GTC or GTD | [optional] [default to null]
**tool_id** | **i32** | for MTA alert only, each user has a unique toolId and it will stay the same, do not send for normal alert  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


