# noaa-tides
Library to fetch NOAA tide and currents data from their [CO-OPS API](https://api.tidesandcurrents.noaa.gov/api/prod/).

The CO-OPS API is a single endpoint with multiple products with different responses that can be requested with different combinations of
query parameters. This library was built to provide a type-safe interface for building requests and deserializing responses into
dedicated structs.

## Currently Supported Products
This library currently supports the the "predictions" product, which includes predicted tide heights for specified stations and date ranges. Contributions
to support additional products are welcome!
