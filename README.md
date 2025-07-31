# 🐙 Octopus Energy Prometheus Exporter

[![rust-clippy analyze](https://github.com/ishantanu/octopus-energy-exporter/actions/workflows/rust_clippy.yml/badge.svg)](https://github.com/ishantanu/octopus-energy-exporter/actions/workflows/rust_clippy.yml)

A lightweight prometheus exporter written in Rust that collects metrics from Octopus Energy using the [octopust](https://github.com/ishantanu/octopust) crate and exposes them for Prometheus scraping.

## 🚀 Features
* Collects electricity and gas usage from the Octopus Energy API
* Exposes usage metrics for Prometheus
* Efficient, asynchronous and fast
* Easy to configure using environment variables

## 📦 Installation
You can build the exporter using `Cargo`:

```
git clone https://github.com/ishantanu/octopus-energy-exporter
cd octopus-energy-exporter
cargo build --release
```

Or run it directly:
```
cargo run --release
```

## ⚙️ Configuration
The exporter expects four environment variables to be set.
* `OCTOPUS_API_KEY` - API key for calling Octopus Energy API
* `E_SERIAL_NUMBER` - Electricity meter serial number
* `G_SERIAL_NUMBER` - Gas meter serial number
* `MPAN` - Electricity Meter Point Administration Number
* `MPRN` - Gas Meter Point Reference Number

In addition the above environment variables, the CLI also supports two flags
```
Run the Octopus Energy Prometheus exporter

Usage: octopus-energy-exporter run [OPTIONS]

Options:
  -t, --timeout <TIMEOUT>    Timeout in seconds for the exporter [default: 0]
  -i, --interval <INTERVAL>  Interval in seconds between metric updates [default: 1800]
  -h, --help                 Print help
```

The `--interval` flag represents the frequency at which the API will be polled for data. This defaults to `1800s` or `30m`. This is based on the fact smart meter readings are available every half hour. One can set a different interval depending on the use case.

## 📊 Exposed Metrics
The following metrics are currently exposed on `http://localhost:9090/metrics`
* `octopus_electricity_usage_2w_kwh` - Total Octopus Energy electricity usage for last 2 weeks in kWh
* `octopus_electricity_usage_week_kwh` - Total Octopus Energy electricity usage on weekly basis in kWh
* `octopus_electricity_usage_4w_kwh` - Total Octopus Energy electricity usage for last 4 weeks in kWh
* `octopus_electricity_usage_two_days_kwh` - Total Octopus Energy electricity usage for two days in kWh
* `octopus_electricity_usage_current_month_kwh` - Total Octopus Energy electricity usage for current month in kWh
* `octopus_electricity_usage_last_2_months_kwh` - Total Octopus Energy electricity usage for the last two months in kWh
* `octopus_electricity_usage_last_3_months_kwh` - Total Octopus Energy electricity usage for the last three months in kWh
* `octopus_electricity_usage_last_6_months_kwh` - Total Octopus Energy electricity usage for the last six months in kWh
* `octopus_electricity_usage_last_1_year_kwh` - Total Octopus Energy electricity usage for the last 1 year in kWh
* `octopus_gas_usage_2w_kwh` - Total Octopus Energy gas usage for last 2 weeks in kWh
* `octopus_gas_usage_4w_kwh` - Total Octopus Energy gas usage for last 4 weeks in kWh
* `octopus_gas_usage_two_days_kwh` - Total Octopus Energy gas usage for two days in kWh
* `octopus_gas_usage_current_month_kwh` - Total Octopus Energy electricity usage for current month in kWh
* `octopus_gas_usage_last_2_months_kwh` - Total Octopus Energy electricity usage for the last two months in kWh
* `octopus_gas_usage_last_3_months_kwh` - Total Octopus Energy electricity usage for the last three months in kWh
* `octopus_gas_usage_last_6_months_kwh` - Total Octopus Energy electricity usage for the last six months in kWh
* `octopus_gas_usage_last_1_year_kwh` - Total Octopus Energy electricity usage for the last 1 year in kWh
* `octopus_gas_usage_week_kwh` - Total Octopus Energy electricity usage on weekly basis in kWh
* `octopus_energy_errors_total` - Total number of errors encountered in fetching the data

## 🛠️ Built With
* [Rust](https://www.rust-lang.org/)
* [octopust](https://github.com/ishantanu/octopust) - Octopus Energy API Client
* [tokio](https://tokio.rs/) - Async runtime

## Future improvements
* Calculate carbon footprint of energy usage using the [Carbon Intensity API](https://carbon-intensity.github.io/api-definitions/#carbon-intensity-api-v2-0-0).

## 🗒️ License

See [LICENSE](./LICENSE) for more details

## Contributions

Contributions, issues, and feature requests are welcome!