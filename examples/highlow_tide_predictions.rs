use chrono::NaiveDate;
use noaa_tides::{NoaaTideClient, PredictionsRequest, params};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = NoaaTideClient::new();

    let request = PredictionsRequest {
        station: "9414290".into(),
        date_range: params::DateRange {
            begin_date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2026, 1, 31).unwrap(),
        },
        datum: params::Datum::MLLW,
        time_zone: params::Timezone::LST_LDT,
        interval: params::Interval::HighLow,
        units: params::Units::English,
    };

    let data = client.fetch_predictions(&request).await?;
    println!("High/low tide predictions:");
    for p in data.predictions.iter() {
        println!(
            "{} - {:?} tide height: {}",
            p.datetime,
            p.tide_type.unwrap(),
            p.height
        );
    }
    Ok(())
}
