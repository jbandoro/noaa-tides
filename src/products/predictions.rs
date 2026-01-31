use super::{de_string_to_f32, de_string_to_native_datetime};
use crate::params::{DateRange, Datum, Interval, Timezone, Units};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Request parameters for tide predictions
#[derive(Debug, Serialize)]
pub struct PredictionsRequest {
    pub station: String,
    pub datum: Datum,
    pub time_zone: Timezone,
    pub interval: Interval,
    pub units: Units,

    #[serde(flatten)]
    pub date_range: DateRange,
}

#[derive(Debug, Deserialize)]
pub struct PredictionsResponse {
    pub predictions: Vec<Prediction>,
}

#[derive(Debug, Deserialize)]
pub struct Prediction {
    #[serde(rename = "t", deserialize_with = "de_string_to_native_datetime")]
    pub datetime: NaiveDateTime,

    #[serde(rename = "v", deserialize_with = "de_string_to_f32")]
    pub height: f32,

    #[serde(rename = "type")]
    pub tide_type: Option<TideType>,
}

/// Variants of all possible tide types in prediction responses
#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
pub enum TideType {
    #[serde(rename = "H")]
    High,
    #[serde(rename = "L")]
    Low,
    #[serde(rename = "HH")]
    HigherHigh,
    #[serde(rename = "LL")]
    LowerLow,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use serde_urlencoded;

    #[test]
    fn request_query() {
        let request = PredictionsRequest {
            station: "1234567".to_string(),
            date_range: DateRange {
                begin_date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
                end_date: NaiveDate::from_ymd_opt(2026, 1, 31).unwrap(),
            },
            datum: Datum::MLLW,
            time_zone: Timezone::LST_LDT,
            interval: Interval::HighLow,
            units: Units::English,
        };

        let query = serde_urlencoded::to_string(&request).unwrap();

        let expected = "station=1234567&datum=MLLW&time_zone=lst_ldt&interval=hilo&\
            units=english&begin_date=20260101&end_date=20260131";
        assert_eq!(query, expected);
    }

    #[test]
    fn response_deserialization_success() {
        let data = r#"
        {
            "predictions": [{
                "t": "2026-01-01 12:34",
                "v": "3.5",
                "type": "H"
            }]
        }   
        "#;
        let predictions_resp = serde_json::from_str::<PredictionsResponse>(data).unwrap();

        let expected_datetime = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            chrono::NaiveTime::from_hms_opt(12, 34, 0).unwrap(),
        );
        let expected_height: f32 = 3.5;
        let expected_tide_type = Some(TideType::High);

        let predictions = predictions_resp.predictions;
        assert_eq!(predictions.len(), 1);
        let prediction = &predictions[0];
        assert_eq!(prediction.datetime, expected_datetime);
        assert_eq!(prediction.height, expected_height);
        assert_eq!(prediction.tide_type, expected_tide_type);
    }
}
