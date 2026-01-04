// Parameters used in NOAA CO-OPS API requests

use chrono::NaiveDate;
use serde::Serialize;

/// Represents datum options for requests, see documentation:
/// <https://api.tidesandcurrents.noaa.gov/api/prod/#datum>
#[derive(Debug, Serialize)]
pub enum Datum {
    /// Mean Higher High Water
    MHHW,
    /// Mean High Water
    MHW,
    /// Mean Tide Level
    MTL,
    /// Mean Sea Level
    MSL,
    /// Mean Low Water
    MLW,
    /// Mean Lower Low Water (Nautical Chart Datum for all U.S. coastal waters)
    MLLW,
    /// Columbia River Datum. Only available for certain stations on the Columbia River, Washington/Oregon
    CRD,
    /// International Great Lakes Datum. Only available for Great Lakes stations.
    IGLD,
    /// Great Lakes Low Water Datum (Nautical Chart Datum for the Great Lakes). Only available for Great Lakes Stations
    LWD,
    /// North American Vertical Datum Note! This datum is not available for all stations.
    NAVD,
    /// Station Datum - original reference that all data is collected to, uniquely defined for each station.
    STND,
}

/// Represents timezone options for requests, see documentation:
/// <https://api.tidesandcurrents.noaa.gov/api/prod/#timezone>
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
#[allow(non_camel_case_types)]
pub enum Timezone {
    /// Greenwich Mean Time
    GMT,
    /// Local Standard Time, not corrected for Daylight Saving Time, local to the requested station.
    LST,
    /// Local Standard Time, corrected for Daylight Saving Time when appropriate, local to the requested station
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
    /// Metric units (Celsius, meters, cm/s appropriate for the data)
    /// Visibility data is kilometers (km), Currents data is in cm/s.
    Metric,
    /// English units (fahrenheit, feet, knots appropriate for the data)
    /// Visibility data is Nautical Miles (nm), Currents data is in knots
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
