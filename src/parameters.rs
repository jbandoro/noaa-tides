// Parameters used in NOAA CO-OPS API requests

use chrono::NaiveDate;
use serde::Serialize;

/// Represents datum options for requests, see documentation:
/// <https://api.tidesandcurrents.noaa.gov/api/prod/#datum>
#[derive(Debug, Serialize)]
pub enum Datum {
    MHHW,
    MHW,
    MTL,
    MLW,
    MLLW,
}

/// Represents timezone options for requests, see documentation:
/// <https://api.tidesandcurrents.noaa.gov/api/prod/#timezone>
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Timezone {
    GMT,
    LST,
    LST_LDT,
}

/// Represents interval options for requests, see documentation:
/// <https://api.tidesandcurrents.noaa.gov/api/prod/#interval>
#[derive(Debug, Serialize)]
pub enum Interval {
    #[serde(rename = "h")]
    Hourly,
    #[serde(rename = "hilo")]
    HighLow,
    #[serde(rename = "1")]
    OneMinute,
    #[serde(rename = "5")]
    FiveMinutes,
    #[serde(rename = "6")]
    SixMinutes,
    #[serde(rename = "10")]
    TenMinutes,
    #[serde(rename = "15")]
    FifteenMinutes,
    #[serde(rename = "30")]
    ThirtyMinutes,
    #[serde(rename = "60")]
    SixtyMinutes,
}

/// Represents units options for requests, see documentation:
/// <https://api.tidesandcurrents.noaa.gov/api/prod/#units>
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Units {
    Metric,
    English,
}

/// Represents date range parameters for requests, see documentation:
/// <https://api.tidesandcurrents.noaa.gov/api/prod/#timerange>
#[derive(Debug, Serialize)]
pub struct DateRange {
    #[serde(serialize_with = "yyyymmdd::serialize")]
    pub begin_date: NaiveDate,

    #[serde(serialize_with = "yyyymmdd::serialize")]
    pub end_date: NaiveDate,
}

mod yyyymmdd {
    use chrono::NaiveDate;
    use serde::Serializer;

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.format("%Y%m%d").to_string();
        serializer.serialize_str(&s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn date_range_serialization() {
        let date_range = DateRange {
            begin_date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2026, 1, 31).unwrap(),
        };

        let actual = serde_json::to_string(&date_range).unwrap();
        let expected = r#"{"begin_date":"20260101","end_date":"20260131"}"#;

        assert_eq!(actual, expected);
    }
}
