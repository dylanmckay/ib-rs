/* 
 * Client Portal Web API
 *
 * Production version of the Client Portal Web API
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryData {
  #[serde(rename = "barLength")]
  bar_length: Option<i32>,
  #[serde(rename = "data")]
  data: Option<Vec<::models::HistorydataData>>,
  #[serde(rename = "delay")]
  delay: Option<i32>,
  /// price/volume/...
  #[serde(rename = "high")]
  high: Option<String>,
  /// price/volume/...
  #[serde(rename = "low")]
  low: Option<String>,
  /// Market Data Availability. The field may contain two chars. The first char is the primary code: R = Realtime, D = Delayed, Z = Frozen, Y = Frozen Delayed. The second char is the secondary code: P = Snapshot Available, p = Consolidated. 
  #[serde(rename = "mdAvailability")]
  md_availability: Option<String>,
  /// total number of points
  #[serde(rename = "points")]
  points: Option<f32>,
  /// start date time
  #[serde(rename = "start")]
  start: Option<String>,
  #[serde(rename = "symbol")]
  symbol: Option<String>,
  #[serde(rename = "text")]
  text: Option<String>,
  #[serde(rename = "tickNum")]
  tick_num: Option<String>,
  #[serde(rename = "timePeriod")]
  time_period: Option<String>,
  #[serde(rename = "travelTime")]
  travel_time: Option<f32>
}

impl HistoryData {
  pub fn new() -> HistoryData {
    HistoryData {
      bar_length: None,
      data: None,
      delay: None,
      high: None,
      low: None,
      md_availability: None,
      points: None,
      start: None,
      symbol: None,
      text: None,
      tick_num: None,
      time_period: None,
      travel_time: None
    }
  }

  pub fn set_bar_length(&mut self, bar_length: i32) {
    self.bar_length = Some(bar_length);
  }

  pub fn with_bar_length(mut self, bar_length: i32) -> HistoryData {
    self.bar_length = Some(bar_length);
    self
  }

  pub fn bar_length(&self) -> Option<&i32> {
    self.bar_length.as_ref()
  }

  pub fn reset_bar_length(&mut self) {
    self.bar_length = None;
  }

  pub fn set_data(&mut self, data: Vec<::models::HistorydataData>) {
    self.data = Some(data);
  }

  pub fn with_data(mut self, data: Vec<::models::HistorydataData>) -> HistoryData {
    self.data = Some(data);
    self
  }

  pub fn data(&self) -> Option<&Vec<::models::HistorydataData>> {
    self.data.as_ref()
  }

  pub fn reset_data(&mut self) {
    self.data = None;
  }

  pub fn set_delay(&mut self, delay: i32) {
    self.delay = Some(delay);
  }

  pub fn with_delay(mut self, delay: i32) -> HistoryData {
    self.delay = Some(delay);
    self
  }

  pub fn delay(&self) -> Option<&i32> {
    self.delay.as_ref()
  }

  pub fn reset_delay(&mut self) {
    self.delay = None;
  }

  pub fn set_high(&mut self, high: String) {
    self.high = Some(high);
  }

  pub fn with_high(mut self, high: String) -> HistoryData {
    self.high = Some(high);
    self
  }

  pub fn high(&self) -> Option<&String> {
    self.high.as_ref()
  }

  pub fn reset_high(&mut self) {
    self.high = None;
  }

  pub fn set_low(&mut self, low: String) {
    self.low = Some(low);
  }

  pub fn with_low(mut self, low: String) -> HistoryData {
    self.low = Some(low);
    self
  }

  pub fn low(&self) -> Option<&String> {
    self.low.as_ref()
  }

  pub fn reset_low(&mut self) {
    self.low = None;
  }

  pub fn set_md_availability(&mut self, md_availability: String) {
    self.md_availability = Some(md_availability);
  }

  pub fn with_md_availability(mut self, md_availability: String) -> HistoryData {
    self.md_availability = Some(md_availability);
    self
  }

  pub fn md_availability(&self) -> Option<&String> {
    self.md_availability.as_ref()
  }

  pub fn reset_md_availability(&mut self) {
    self.md_availability = None;
  }

  pub fn set_points(&mut self, points: f32) {
    self.points = Some(points);
  }

  pub fn with_points(mut self, points: f32) -> HistoryData {
    self.points = Some(points);
    self
  }

  pub fn points(&self) -> Option<&f32> {
    self.points.as_ref()
  }

  pub fn reset_points(&mut self) {
    self.points = None;
  }

  pub fn set_start(&mut self, start: String) {
    self.start = Some(start);
  }

  pub fn with_start(mut self, start: String) -> HistoryData {
    self.start = Some(start);
    self
  }

  pub fn start(&self) -> Option<&String> {
    self.start.as_ref()
  }

  pub fn reset_start(&mut self) {
    self.start = None;
  }

  pub fn set_symbol(&mut self, symbol: String) {
    self.symbol = Some(symbol);
  }

  pub fn with_symbol(mut self, symbol: String) -> HistoryData {
    self.symbol = Some(symbol);
    self
  }

  pub fn symbol(&self) -> Option<&String> {
    self.symbol.as_ref()
  }

  pub fn reset_symbol(&mut self) {
    self.symbol = None;
  }

  pub fn set_text(&mut self, text: String) {
    self.text = Some(text);
  }

  pub fn with_text(mut self, text: String) -> HistoryData {
    self.text = Some(text);
    self
  }

  pub fn text(&self) -> Option<&String> {
    self.text.as_ref()
  }

  pub fn reset_text(&mut self) {
    self.text = None;
  }

  pub fn set_tick_num(&mut self, tick_num: String) {
    self.tick_num = Some(tick_num);
  }

  pub fn with_tick_num(mut self, tick_num: String) -> HistoryData {
    self.tick_num = Some(tick_num);
    self
  }

  pub fn tick_num(&self) -> Option<&String> {
    self.tick_num.as_ref()
  }

  pub fn reset_tick_num(&mut self) {
    self.tick_num = None;
  }

  pub fn set_time_period(&mut self, time_period: String) {
    self.time_period = Some(time_period);
  }

  pub fn with_time_period(mut self, time_period: String) -> HistoryData {
    self.time_period = Some(time_period);
    self
  }

  pub fn time_period(&self) -> Option<&String> {
    self.time_period.as_ref()
  }

  pub fn reset_time_period(&mut self) {
    self.time_period = None;
  }

  pub fn set_travel_time(&mut self, travel_time: f32) {
    self.travel_time = Some(travel_time);
  }

  pub fn with_travel_time(mut self, travel_time: f32) -> HistoryData {
    self.travel_time = Some(travel_time);
    self
  }

  pub fn travel_time(&self) -> Option<&f32> {
    self.travel_time.as_ref()
  }

  pub fn reset_travel_time(&mut self) {
    self.travel_time = None;
  }

}



