/* 
 * Client Portal Web API
 *
 * Client Poral Web API
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryresultBars {
  #[serde(rename = "close")]
  close: Option<f32>,
  #[serde(rename = "count")]
  count: Option<f32>,
  #[serde(rename = "endTime")]
  end_time: Option<String>,
  #[serde(rename = "high")]
  high: Option<f32>,
  #[serde(rename = "low")]
  low: Option<f32>,
  #[serde(rename = "open")]
  open: Option<f32>,
  #[serde(rename = "time")]
  time: Option<String>,
  #[serde(rename = "volume")]
  volume: Option<f32>,
  #[serde(rename = "weightedAvg")]
  weighted_avg: Option<f32>
}

impl HistoryresultBars {
  pub fn new() -> HistoryresultBars {
    HistoryresultBars {
      close: None,
      count: None,
      end_time: None,
      high: None,
      low: None,
      open: None,
      time: None,
      volume: None,
      weighted_avg: None
    }
  }

  pub fn set_close(&mut self, close: f32) {
    self.close = Some(close);
  }

  pub fn with_close(mut self, close: f32) -> HistoryresultBars {
    self.close = Some(close);
    self
  }

  pub fn close(&self) -> Option<&f32> {
    self.close.as_ref()
  }

  pub fn reset_close(&mut self) {
    self.close = None;
  }

  pub fn set_count(&mut self, count: f32) {
    self.count = Some(count);
  }

  pub fn with_count(mut self, count: f32) -> HistoryresultBars {
    self.count = Some(count);
    self
  }

  pub fn count(&self) -> Option<&f32> {
    self.count.as_ref()
  }

  pub fn reset_count(&mut self) {
    self.count = None;
  }

  pub fn set_end_time(&mut self, end_time: String) {
    self.end_time = Some(end_time);
  }

  pub fn with_end_time(mut self, end_time: String) -> HistoryresultBars {
    self.end_time = Some(end_time);
    self
  }

  pub fn end_time(&self) -> Option<&String> {
    self.end_time.as_ref()
  }

  pub fn reset_end_time(&mut self) {
    self.end_time = None;
  }

  pub fn set_high(&mut self, high: f32) {
    self.high = Some(high);
  }

  pub fn with_high(mut self, high: f32) -> HistoryresultBars {
    self.high = Some(high);
    self
  }

  pub fn high(&self) -> Option<&f32> {
    self.high.as_ref()
  }

  pub fn reset_high(&mut self) {
    self.high = None;
  }

  pub fn set_low(&mut self, low: f32) {
    self.low = Some(low);
  }

  pub fn with_low(mut self, low: f32) -> HistoryresultBars {
    self.low = Some(low);
    self
  }

  pub fn low(&self) -> Option<&f32> {
    self.low.as_ref()
  }

  pub fn reset_low(&mut self) {
    self.low = None;
  }

  pub fn set_open(&mut self, open: f32) {
    self.open = Some(open);
  }

  pub fn with_open(mut self, open: f32) -> HistoryresultBars {
    self.open = Some(open);
    self
  }

  pub fn open(&self) -> Option<&f32> {
    self.open.as_ref()
  }

  pub fn reset_open(&mut self) {
    self.open = None;
  }

  pub fn set_time(&mut self, time: String) {
    self.time = Some(time);
  }

  pub fn with_time(mut self, time: String) -> HistoryresultBars {
    self.time = Some(time);
    self
  }

  pub fn time(&self) -> Option<&String> {
    self.time.as_ref()
  }

  pub fn reset_time(&mut self) {
    self.time = None;
  }

  pub fn set_volume(&mut self, volume: f32) {
    self.volume = Some(volume);
  }

  pub fn with_volume(mut self, volume: f32) -> HistoryresultBars {
    self.volume = Some(volume);
    self
  }

  pub fn volume(&self) -> Option<&f32> {
    self.volume.as_ref()
  }

  pub fn reset_volume(&mut self) {
    self.volume = None;
  }

  pub fn set_weighted_avg(&mut self, weighted_avg: f32) {
    self.weighted_avg = Some(weighted_avg);
  }

  pub fn with_weighted_avg(mut self, weighted_avg: f32) -> HistoryresultBars {
    self.weighted_avg = Some(weighted_avg);
    self
  }

  pub fn weighted_avg(&self) -> Option<&f32> {
    self.weighted_avg.as_ref()
  }

  pub fn reset_weighted_avg(&mut self) {
    self.weighted_avg = None;
  }

}



