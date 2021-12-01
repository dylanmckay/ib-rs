# HistoryData

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bar_length** | **i32** | The number of seconds in a bar | [optional] [default to null]
**data** | [**Vec<::models::HistorydataData>**](historydata_data.md) |  | [optional] [default to null]
**high** | **String** | High value during this time series with format %h/%v/%t. %h is the high price (scaled by priceFactor), %v is volume (volume factor will always be 100 (reported volume &#x3D; actual volume/100)) and %t is minutes from start time of the chart  | [optional] [default to null]
**low** | **String** | Low value during this time series with format %l/%v/%t. %l is the low price (scaled by priceFactor), %v is volume (volume factor will always be 100 (reported volume &#x3D; actual volume/100)) and %t is minutes from start time of the chart  | [optional] [default to null]
**md_availability** | **String** | Market Data Availability. The field may contain two chars. The first char is the primary code: S &#x3D; Streaming, R &#x3D; Realtime, D &#x3D; Delayed, Z &#x3D; Frozen, Y &#x3D; Frozen Delayed. The second char is the secondary code: P &#x3D; Snapshot Available, p &#x3D; Consolidated.  | [optional] [default to null]
**message_version** | **i32** |  | [optional] [default to null]
**mkt_data_delay** | **i32** | The time it takes, in milliseconds, to process the historical data request | [optional] [default to null]
**negative_capable** | **bool** |  | [optional] [default to null]
**outside_rth** | **bool** | The historical data returned includes outside of regular trading hours  | [optional] [default to null]
**points** | **i32** | total number of points | [optional] [default to null]
**price_display_rule** | **i32** |  | [optional] [default to null]
**price_display_value** | **String** |  | [optional] [default to null]
**price_factor** | **i32** | priceFactor is price increment obtained from display rule | [optional] [default to null]
**start_time** | **String** | start date time in the format YYYYMMDD-HH:mm:ss | [optional] [default to null]
**symbol** | **String** | Underlying symbol | [optional] [default to null]
**text** | **String** | companyName | [optional] [default to null]
**time_period** | **String** | The duration for the historical data request | [optional] [default to null]
**trading_day_duration** | **i32** | The number of seconds in the trading day | [optional] [default to null]
**travel_time** | **i32** |  | [optional] [default to null]
**volume_factor** | **i32** |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


