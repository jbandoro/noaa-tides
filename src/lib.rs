//! noaa-tides
//!
//! Library to fetch NOAA tide and currents data from their [CO-OPS API](https://api.tidesandcurrents.noaa.gov/api/prod/).
//!
//! The CO-OPS API is a single endpoint with multiple products that can be requested with different combinations of
//! query parameters. This library was built to provide a type-safe interface for building requests and deserializing responses into
//! dedicated structs. This library currently supports the "predictions" product, which includes predicted tide heights for
//! specified stations and date ranges.
//!
//! Contributions to support additional products are welcome!
//!
//! No API keys are required since the NOAA CO-OPS API does not require authentication, please be mindful with usage.
//!
//! # Example
//!
//! Below is an example to fetch high/low tide predictions for the San Francisco Golden Gate station for January 2026
//! ```no_run
//! use noaa_tides::{DateRange, Datum, Interval, NoaaTideClient, PredictionsRequest, Timezone, Units};
//!
//! use chrono::NaiveDate;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = NoaaTideClient::new();
//!
//!     let request = PredictionsRequest {
//!         station: "9414290".into(),
//!         date_range: DateRange {
//!             begin_date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
//!             end_date: NaiveDate::from_ymd_opt(2026, 1, 31).unwrap(),
//!         },
//!         datum: Datum::MLLW,
//!         time_zone: Timezone::LST_LDT,
//!         interval: Interval::HighLow,
//!         units: Units::English,
//!     };
//!
//!     let data = client.fetch(&request).await?;
//!     println!("High/low tide predictions:");
//!     for p in data.predictions.iter() {
//!         println!(
//!             "{} - {:?} tide height: {}",
//!             p.datetime,
//!             p.tide_type.unwrap(),
//!             p.height
//!         );
//!     }
//!     Ok(())
//! }

//!
mod parameters;
mod products;

pub use crate::parameters::{DateRange, Datum, Interval, Timezone, Units};
pub use crate::products::predictions::{PredictionsRequest, TideType};

use crate::products::NoaaTideProduct;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const BASE_URL: &str = "https://api.tidesandcurrents.noaa.gov/api/prod/datagetter";

/// Client to get data from the NOAA Tides and Currents API
pub struct NoaaTideClient {
    http: Client,
    base_url: String,
}

impl NoaaTideClient {
    pub fn new() -> Self {
        Self {
            http: Client::new(),
            base_url: BASE_URL.to_string(),
        }
    }

    /// Fetch data from the NOAA Tides and Currents API for a product request
    pub async fn fetch<P>(&self, params: &P) -> Result<P::Response, NoaaTideError>
    where
        P: NoaaTideProduct + Serialize,
        P::Response: serde::de::DeserializeOwned,
    {
        let response = self
            .http
            .get(&self.base_url)
            .query(&params)
            .query(&[("product", params.product_name()), ("format", "json")])
            .send()
            .await?
            .json::<NoaaResponse<P::Response>>()
            .await?;
        match response {
            NoaaResponse::Success(data) => Ok(data),
            NoaaResponse::Failure(wrapper) => Err(NoaaTideError::ApiError(wrapper.error.message)),
        }
    }
}

impl Default for NoaaTideClient {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum NoaaResponse<T> {
    Success(T),
    Failure(ErrorWrapper),
}

/// Represents an error with its message returned by the NOAA API
#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub message: String,
}

/// Wrapper for NOAA API error responses
#[derive(Debug, Deserialize)]
pub struct ErrorWrapper {
    pub error: ApiError,
}

/// Possible errors when fetching data from the NOAA API
#[derive(Error, Debug)]
pub enum NoaaTideError {
    #[error("Network error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("NOAA API returned an error: {0}")]
    ApiError(String),

    #[error("Unknown error occurred")]
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito;

    #[derive(Debug, Serialize)]
    struct MockProductRequest {
        station: String,
    }

    #[derive(Debug, Deserialize)]
    struct MockProductResponse {
        value: i32,
    }

    impl NoaaTideProduct for MockProductRequest {
        type Response = MockProductResponse;

        fn product_name(&self) -> &'static str {
            "some_product"
        }
    }

    #[tokio::test]
    async fn verify_query_parameters() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![mockito::Matcher::UrlEncoded(
                "station".into(),
                "1234567".into(),
            )]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"value": 10}"#)
            .create_async()
            .await;

        let client = NoaaTideClient {
            http: Client::new(),
            base_url: server.url(),
        };

        let request = MockProductRequest {
            station: "1234567".to_string(),
        };

        let result = client.fetch(&request).await;
        assert!(result.is_ok());
        mock.assert_async().await;
        assert_eq!(result.unwrap().value, 10);
    }
}
