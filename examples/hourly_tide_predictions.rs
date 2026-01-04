use chrono::NaiveDate;
use noaa_tides::{DateRange, Datum, Interval, NoaaTideClient, PredictionsRequest, Timezone, Units};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = NoaaTideClient::new();
    let request = PredictionsRequest {
        station: "9414290".into(),
        date_range: DateRange {
            begin_date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2026, 1, 31).unwrap(),
        },
        datum: Datum::MLLW,
        time_zone: Timezone::LST_LDT,
        interval: Interval::Hourly,
        units: Units::English,
    };
    let data = client.fetch(&request).await?;
    for prediction in data.predictions.iter() {
        println!("{}: {:+.2} ft", prediction.datetime, prediction.height);
    }
    Ok(())
}
