# AlertResponse

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account** | **String** | account id | [optional] [default to null]
**alert_active** | **i32** | whether alert is active or not, so value can only be 0 or 1 | [optional] [default to null]
**alert_default_type** | **String** | MTA alert only | [optional] [default to null]
**alert_email** | **String** | email address to receive alert | [optional] [default to null]
**alert_message** | **String** | The message you want to receive via email or text message | [optional] [default to null]
**alert_mta_currency** | **String** | MTA alert only | [optional] [default to null]
**alert_mta_defaults** | **String** | MTA alert only | [optional] [default to null]
**alert_name** | **String** | name of alert | [optional] [default to null]
**alert_play_audio** | **String** | audio message to play when alert is triggered | [optional] [default to null]
**alert_repeatable** | **i32** | whether alert is repeatable or not, so value can only be 0 or 1 | [optional] [default to null]
**alert_send_message** | **i32** | whether allowing to send email or not, so value can only be 0 or 1,  | [optional] [default to null]
**alert_show_popup** | **i32** | value can only be 0 or 1, set to 1 to allow to show alert in pop-ups | [optional] [default to null]
**alert_triggered** | **bool** | whether the alert has been triggered | [optional] [default to null]
**condition_outside_rth** | **i32** | whether allowing the condition can be triggered outside of regular trading hours, 1 means allow | [optional] [default to null]
**condition_size** | **i32** | size of conditions array | [optional] [default to null]
**conditions** | [**Vec<::models::AlertresponseConditions>**](alertresponse_conditions.md) |  | [optional] [default to null]
**expire_time** | **String** | format, YYYYMMDD-HH:mm:ss  | [optional] [default to null]
**itws_orders_only** | **i32** | value can only be 0 or 1, set to 1 to enable the alert only in IBKR mobile  | [optional] [default to null]
**order_id** | **i32** |  | [optional] [default to null]
**order_not_editable** | **bool** | whether the alert can be edited | [optional] [default to null]
**order_status** | **String** | status of alert | [optional] [default to null]
**outside_rth** | **i32** | value can only be 0 or 1, set to 1 if the alert can be triggered outside regular trading hours.  | [optional] [default to null]
**tif** | **String** | time in force, can only be GTC or GTD | [optional] [default to null]
**time_zone** | **String** | MTA alert only | [optional] [default to null]
**tool_id** | **i32** | for MTA alert only, each user has a unique toolId and it will stay the same, do not send for normal alert  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


