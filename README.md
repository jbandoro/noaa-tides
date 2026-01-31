# noaa-tides
Library to fetch NOAA tide and currents data from their [CO-OPS API](https://api.tidesandcurrents.noaa.gov/api/prod/).

The CO-OPS API is a single endpoint that supports multiple products with different response formats
that can be requested with different combinations of query parameters. This library was built to 
provide a type-safe interface for building requests and deserializing responses into
dedicated structs for each product.

## Currently Supported Products
This library currently supports the following products:

- **predictions**: includes predicted tide heights for specified stations and date ranges. Supports
various options for datum, time zones, intervals and units. Returns a vector of tide predictions with
dateime, height an optional tide type if the `Interval::HighLow` parameter is used.


## Example Usage

Below is an example using the `predictions` product to fetch hourly tide predictions for the San
Francisco Golden Gate station for the month of January 2026. More examples can be found in the
`examples/` directory.

```rust
use noaa_tides::{params, NoaaTideClient, PredictionsRequest};
use chrono::NaiveDate;

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
        interval: params::Interval::Hourly,
        units: params::Units::English,
    };
    let data = client.fetch_predictions(&request).await?;
    for prediction in data.predictions.iter() {
        println!("{}: {:+.2} ft", prediction.datetime, prediction.height);
    }
    Ok(())
}
```
