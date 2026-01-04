use noaa_tides::{DateRange, Datum, Interval, NoaaTideClient, PredictionsRequest, Timezone, Units};

use chrono::NaiveDate;

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
        interval: Interval::HighLow,
        units: Units::English,
    };

    let data = client.fetch(&request).await?;
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
