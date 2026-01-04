use chrono::Local;
use noaa_tides::NoaaTideClient;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let client = NoaaClient::new();
//     let today = Local::now().date_naive();

//     // SF Golden Gate Station
//     let predictions = client.get_predictions("9414290", today, today).await?;

//     println!("Predictions for today:");
//     for p in predictions.iter() {
//         println!("Tide Prediction: {:?}", p);
//     }

//     Ok(())
// }

use chrono::NaiveDate;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = NoaaTideClient::new();
    let begin_date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2026, 1, 31).unwrap();
    let station_id = "9414290"; // SF Golden Gate Station

    let predictions = client
        .get_predictions(station_id, begin_date, end_date)
        .await?;
    println!("Tide Predictions:");
    for p in predictions.iter() {
        println!("{} - {:?} tide height: {}", p.time, p.tide_type, p.height);
    }
    Ok(())
}
