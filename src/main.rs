use prometheus::{Encoder, TextEncoder, IntCounter, Gauge, Registry};
use std::{env, time::Duration};
use std::sync::Arc;
use tokio::time;
use warp::Filter;
use chrono::{DateTime, Datelike, Duration as ChronoDuration, Timelike, Utc};
use clap::{Parser, Subcommand};
use std::collections::HashMap;

mod usage;
mod carbon_intensity;
use octopust::Client;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run the Octopus Energy Prometheus exporter
    Run {
        /// Timeout in seconds for the exporter
        #[arg(short, long, default_value = "0")]
        timeout: u64,

        /// Interval in seconds between metric updates
        #[arg(short, long, default_value = "1800")]
        interval: u64,

        /// Region to get carbon intensity data from
        #[arg(short, long, default_value = "England")]
        region: String,
    }
}

// Using years directly
fn get_year_range(years_back: i32) -> (DateTime<Utc>, DateTime<Utc>) {
    let end = Utc::now();
    
    let start = end
        .with_year(end.year() - years_back).unwrap()
        .with_day(1).unwrap()
        .with_hour(0).unwrap()
        .with_minute(0).unwrap()
        .with_second(0).unwrap()
        .with_nanosecond(0).unwrap();

    (start, end)
}

fn get_month_range(months_back: i32) -> (DateTime<Utc>, DateTime<Utc>) {
    let end = Utc::now();
    
    if months_back == 0 {
        // Current month
        let start = end
            .with_day(1).unwrap()
            .with_hour(0).unwrap()
            .with_minute(0).unwrap()
            .with_second(0).unwrap()
            .with_nanosecond(0).unwrap();
        return (start, end);
    }
    
    // Previous months
    let mut year = end.year();
    let mut month = end.month() as i32 - months_back;
    
    while month <= 0 {
        year -= 1;
        month += 12;
    }
    
    let start = end
        .with_year(year).unwrap()
        .with_month(month as u32).unwrap()
        .with_day(1).unwrap()
        .with_hour(0).unwrap()
        .with_minute(0).unwrap()
        .with_second(0).unwrap()
        .with_nanosecond(0).unwrap();

    (start, end)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match args.command {
        Commands::Run { timeout, interval, region } => {
            println!("Starting Octopus Energy Prometheus exporter with timeout: {timeout} seconds");
            
            // Verify API key is set
            if env::var("OCTOPUS_API_KEY").is_err() {
                eprintln!("Error: OCTOPUS_API_KEY environment variable must be set");
                std::process::exit(1);
            }

            let mut group_by_opts = HashMap::new();
            let mut periods: HashMap<String, DateTime<Utc>> = HashMap::new();
            let now = Utc::now();

            // Current month
            let (start_current, end_current) = get_month_range(0);

            // Last 2 months
            let (start_2m, end_2m) = get_month_range(2);

            // Last 3 months
            let (start_3m, end_3m) = get_month_range(3);

            // Last 6 months
            let (start_6m, end_6m) = get_month_range(6);

            // Last 1 year
            let (start_1y_direct, end_1y_direct) = get_year_range(1);

            group_by_opts.insert(String::from("hour"), "hour");

            periods.insert("2d".to_string(), now - ChronoDuration::days(2));
            periods.insert("1w".to_string(), now - ChronoDuration::weeks(1));
            periods.insert("2w".to_string(), now - ChronoDuration::weeks(2));
            periods.insert("4w".to_string(), now - ChronoDuration::weeks(4));
            periods.insert("1m".to_string(), now - ChronoDuration::days(end_current.signed_duration_since(start_current).num_days()));
            periods.insert("2m".to_string(), now - ChronoDuration::days(end_2m.signed_duration_since(start_2m).num_days()));
            periods.insert("3m".to_string(), now - ChronoDuration::days(end_3m.signed_duration_since(start_3m).num_days()));
            periods.insert("6m".to_string(), now - ChronoDuration::days(end_6m.signed_duration_since(start_6m).num_days()));
            periods.insert("1y".to_string(), now - ChronoDuration::days(end_1y_direct.signed_duration_since(start_1y_direct).num_days()));

            // Create Prometheus registry and metrics
            let registry = Registry::new();

            let electricity_usage_gauge_two_weeks = Gauge::new("octopus_electricity_usage_2w_kwh", "Total Octopus Energy electricity usage for last 2 weeks in kWh").unwrap();
            registry.register(Box::new(electricity_usage_gauge_two_weeks.clone())).unwrap();

            let electricity_usage_gauge_four_weeks = Gauge::new("octopus_electricity_usage_4w_kwh", "Total Octopus Energy electricity usage for last 4 weeks in kWh").unwrap();
            registry.register(Box::new(electricity_usage_gauge_four_weeks.clone())).unwrap();

            let electricity_usage_gauge_two_days = Gauge::new("octopus_electricity_usage_two_days_kwh", "Total Octopus Energy electricity usage for two days in kWh").unwrap();
            registry.register(Box::new(electricity_usage_gauge_two_days.clone())).unwrap();

            let electricity_usage_gauge_current_month = Gauge::new("octopus_electricity_usage_current_month_kwh", "Total Octopus Energy electricity usage for current month in kWh").unwrap();
            registry.register(Box::new(electricity_usage_gauge_current_month.clone())).unwrap();

            let electricity_usage_gauge_last_2_months = Gauge::new("octopus_electricity_usage_last_2_months_kwh", "Total Octopus Energy electricity usage for the last two months in kWh").unwrap();
            registry.register(Box::new(electricity_usage_gauge_last_2_months.clone())).unwrap();

            let electricity_usage_gauge_last_3_months = Gauge::new("octopus_electricity_usage_last_3_months_kwh", "Total Octopus Energy electricity usage for the last three months in kWh").unwrap();
            registry.register(Box::new(electricity_usage_gauge_last_3_months.clone())).unwrap();
            
            let electricity_usage_gauge_last_6_months = Gauge::new("octopus_electricity_usage_last_6_months_kwh", "Total Octopus Energy electricity usage for the last six months in kWh").unwrap();
            registry.register(Box::new(electricity_usage_gauge_last_6_months.clone())).unwrap();
            
            let electricity_usage_gauge_last_1_year = Gauge::new("octopus_electricity_usage_last_1_year_kwh", "Total Octopus Energy electricity usage for the last 1 year in kWh").unwrap();
            registry.register(Box::new(electricity_usage_gauge_last_1_year.clone())).unwrap();

            let electricity_usage_gauge_week = Gauge::new("octopus_electricity_usage_week_kwh", "Total Octopus Energy electricity usage on weekly basis in kWh").unwrap();
            registry.register(Box::new(electricity_usage_gauge_week.clone())).unwrap();

            // gas usage 
            let gas_usage_gauge_two_weeks = Gauge::new("octopus_gas_usage_2w_kwh", "Total Octopus Energy gas usage for last 2 weeks in kWh").unwrap();
            registry.register(Box::new(gas_usage_gauge_two_weeks.clone())).unwrap();

            let gas_usage_gauge_four_weeks = Gauge::new("octopus_gas_usage_4w_kwh", "Total Octopus Energy gas usage for last 4 weeks in kWh").unwrap();
            registry.register(Box::new(gas_usage_gauge_four_weeks.clone())).unwrap();

            let gas_usage_gauge_two_days = Gauge::new("octopus_gas_usage_two_days_kwh", "Total Octopus Energy gas usage for two days in kWh").unwrap();
            registry.register(Box::new(gas_usage_gauge_two_days.clone())).unwrap();

            let gas_usage_gauge_current_month = Gauge::new("octopus_gas_usage_current_month_kwh", "Total Octopus Energy gas usage for current month in kWh").unwrap();
            registry.register(Box::new(gas_usage_gauge_current_month.clone())).unwrap();

            let gas_usage_gauge_last_2_months = Gauge::new("octopus_gas_usage_last_2_months_kwh", "Total Octopus Energy gas usage for the last two months in kWh").unwrap();
            registry.register(Box::new(gas_usage_gauge_last_2_months.clone())).unwrap();

            let gas_usage_gauge_last_3_months = Gauge::new("octopus_gas_usage_last_3_months_kwh", "Total Octopus Energy gas usage for the last three months in kWh").unwrap();
            registry.register(Box::new(gas_usage_gauge_last_3_months.clone())).unwrap();
            
            let gas_usage_gauge_last_6_months = Gauge::new("octopus_gas_usage_last_6_months_kwh", "Total Octopus Energy gas usage for the last six months in kWh").unwrap();
            registry.register(Box::new(gas_usage_gauge_last_6_months.clone())).unwrap();
            
            let gas_usage_gauge_last_1_year = Gauge::new("octopus_gas_usage_last_1_year_kwh", "Total Octopus Energy gas usage for the last 1 year in kWh").unwrap();
            registry.register(Box::new(gas_usage_gauge_last_1_year.clone())).unwrap();

            let gas_usage_gauge_week = Gauge::new("octopus_gas_usage_week_kwh", "Total Octopus Energy gas usage on weekly basis in kWh").unwrap();
            registry.register(Box::new(gas_usage_gauge_week.clone())).unwrap();

            // carbon intensity
            let carbon_intensity_gauge_week = Gauge::new("octopus_energy_carbon_emissions_week_grams", "Total carbon emissions on weekly basis in grams").unwrap();
            registry.register(Box::new(carbon_intensity_gauge_week.clone())).unwrap();

            let carbon_intensity_gauge_two_weeks = Gauge::new("octopus_energy_carbon_emissions_2w_grams", "Total carbon emissions for last two weeks in grams").unwrap();
            registry.register(Box::new(carbon_intensity_gauge_two_weeks.clone())).unwrap();

            let carbon_intensity_gauge_four_weeks = Gauge::new("octopus_energy_carbon_emissions_4w_grams", "Total carbon emissions for last four weeks in grams").unwrap();
            registry.register(Box::new(carbon_intensity_gauge_four_weeks.clone())).unwrap();

            let carbon_intensity_gauge_two_days = Gauge::new("octopus_energy_carbon_emissions_2d_grams", "Total carbon emissions for last two days in grams").unwrap();
            registry.register(Box::new(carbon_intensity_gauge_two_days.clone())).unwrap();

            let carbon_intensity_gauge_current_month = Gauge::new("octopus_energy_carbon_emissions_current_month_grams", "Total carbon emissions for current month in grams").unwrap();
            registry.register(Box::new(carbon_intensity_gauge_current_month.clone())).unwrap();

            let carbon_intensity_gauge_last_2_months = Gauge::new("octopus_energy_carbon_emissions_last_2_months_grams", "Total Octopus Energy gas usage for the last two months in kWh").unwrap();
            registry.register(Box::new(carbon_intensity_gauge_last_2_months.clone())).unwrap();
            
            let carbon_intensity_gauge_last_3_months = Gauge::new("octopus_energy_carbon_emissions_last_3_months_grams", "Total Octopus Energy gas usage for the last three months in kWh").unwrap();
            registry.register(Box::new(carbon_intensity_gauge_last_3_months.clone())).unwrap();
            
            let carbon_intensity_gauge_last_6_months = Gauge::new("octopus_energy_carbon_emissions_last_6_months_grams", "Total Octopus Energy gas usage for the last six months in kWh").unwrap();
            registry.register(Box::new(carbon_intensity_gauge_last_6_months.clone())).unwrap();
            
            let carbon_intensity_gauge_last_1_year = Gauge::new("octopus_energy_carbon_emissions_last_1_months_grams", "Total Octopus Energy gas usage for the last 1 year in kWh").unwrap();
            registry.register(Box::new(carbon_intensity_gauge_last_1_year.clone())).unwrap();
            
            let error_counter = IntCounter::new("octopus_energy_errors_total", "Total number of errors encountered").unwrap();
            registry.register(Box::new(error_counter.clone())).unwrap();

            let registry = Arc::new(registry);

            // Create a future that will complete after the timeout (if timeout > 0)
            let timeout_future = if timeout > 0 {
                Some(time::sleep(Duration::from_secs(timeout)))
            } else {
                None
            };

            // Polling task for updating metrics
            {
                let electricity_usage_gauge_two_weeks = electricity_usage_gauge_two_weeks.clone();
                let electricity_usage_gauge_four_weeks = electricity_usage_gauge_four_weeks.clone();
                let electricity_usage_gauge_two_days = electricity_usage_gauge_two_days.clone();
                let electricity_usage_gauge_week = electricity_usage_gauge_week.clone();
                let electricity_usage_gauge_current_month = electricity_usage_gauge_current_month.clone();
                let electricity_usage_gauge_last_2_months = electricity_usage_gauge_last_2_months.clone();
                let electricity_usage_gauge_last_3_months = electricity_usage_gauge_last_3_months.clone();
                let electricity_usage_gauge_last_6_months = electricity_usage_gauge_last_6_months.clone();
                let electricity_usage_gauge_last_1_year = electricity_usage_gauge_last_1_year.clone();

                let gas_usage_gauge_two_weeks = gas_usage_gauge_two_weeks.clone();
                let gas_usage_gauge_four_weeks = gas_usage_gauge_four_weeks.clone();
                let gas_usage_gauge_two_days = gas_usage_gauge_two_days.clone();
                let gas_usage_gauge_week = gas_usage_gauge_week.clone();
                let gas_usage_gauge_current_month = gas_usage_gauge_current_month.clone();
                let gas_usage_gauge_last_2_months = gas_usage_gauge_last_2_months.clone();
                let gas_usage_gauge_last_3_months = gas_usage_gauge_last_3_months.clone();
                let gas_usage_gauge_last_6_months = gas_usage_gauge_last_6_months.clone();
                let gas_usage_gauge_last_1_year = gas_usage_gauge_last_1_year.clone();

                let carbon_intensity_gauge_week = carbon_intensity_gauge_week.clone();
                let carbon_intensity_gauge_two_weeks = carbon_intensity_gauge_two_weeks.clone();
                let carbon_intensity_gauge_four_weeks = carbon_intensity_gauge_four_weeks.clone();
                let carbon_intensity_gauge_two_days = carbon_intensity_gauge_two_days.clone();
                let carbon_intensity_gauge_current_month = carbon_intensity_gauge_current_month.clone();
                let carbon_intensity_gauge_last_2_months = carbon_intensity_gauge_last_2_months.clone();
                let carbon_intensity_gauge_last_3_months = carbon_intensity_gauge_last_3_months.clone();
                let carbon_intensity_gauge_last_6_months = carbon_intensity_gauge_last_6_months.clone();
                let carbon_intensity_gauge_last_1_year = carbon_intensity_gauge_last_1_year.clone();

                let error_counter = error_counter.clone();

                tokio::spawn(async move {
                    let api_key = env::var("OCTOPUS_API_KEY").expect("OCTOPUS_API_KEY not set");
                    let client = Client::new(api_key);

                    loop {
                        let now = Utc::now();
                        
                        match usage::fetch_electricity_and_gas_consumption(&client, &now.format("%Y-%m-%dT%H:%M:%SZ").to_string(), &periods, &group_by_opts, region.as_str()).await {
                            Ok(summary) => {
                                electricity_usage_gauge_two_days.set(summary.e_usage_kwh_two_days);
                                electricity_usage_gauge_week.set(summary.e_usage_kwh_week);
                                electricity_usage_gauge_two_weeks.set(summary.e_usage_kwh_two_weeks);
                                electricity_usage_gauge_four_weeks.set(summary.e_usage_kwh_four_weeks);
                                electricity_usage_gauge_current_month.set(summary.e_usage_kwh_month);
                                electricity_usage_gauge_last_2_months.set(summary.e_usage_kwh_two_months);
                                electricity_usage_gauge_last_3_months.set(summary.e_usage_kwh_three_months);
                                electricity_usage_gauge_last_6_months.set(summary.e_usage_kwh_six_months);
                                electricity_usage_gauge_last_1_year.set(summary.e_usage_kwh_year);

                                gas_usage_gauge_two_days.set(summary.g_usage_kwh_two_days);
                                gas_usage_gauge_week.set(summary.g_usage_kwh_week);
                                gas_usage_gauge_two_weeks.set(summary.g_usage_kwh_two_weeks);
                                gas_usage_gauge_four_weeks.set(summary.g_usage_kwh_four_weeks);
                                gas_usage_gauge_current_month.set(summary.g_usage_kwh_month);
                                gas_usage_gauge_last_2_months.set(summary.g_usage_kwh_two_months);
                                gas_usage_gauge_last_3_months.set(summary.g_usage_kwh_three_months);
                                gas_usage_gauge_last_6_months.set(summary.g_usage_kwh_six_months);
                                gas_usage_gauge_last_1_year.set(summary.g_usage_kwh_year);

                                carbon_intensity_gauge_two_days.set(summary.carbon_intensity_two_days);
                                carbon_intensity_gauge_current_month.set(summary.carbon_intensity_month);
                                carbon_intensity_gauge_four_weeks.set(summary.carbon_intensity_four_weeks);
                                carbon_intensity_gauge_week.set(summary.carbon_intensity_week);
                                carbon_intensity_gauge_last_2_months.set(summary.carbon_intensity_two_months);
                                carbon_intensity_gauge_last_3_months.set(summary.carbon_intensity_three_months);
                                carbon_intensity_gauge_last_6_months.set(summary.carbon_intensity_six_months);
                                carbon_intensity_gauge_two_weeks.set(summary.carbon_intensity_two_weeks);
                                carbon_intensity_gauge_last_1_year.set(summary.carbon_intensity_year);
                                
                                println!(
                                    "[DEBUG] Electricity Usage Summary: usage_kwh = 2d : {:.3}, current week: {:.3}, 2 weeks: {:.3}, 4 weeks: {:.3}, current month: {:.3}, 2 months: {:.3}, 3 months: {:.3}, 6 months: {:.3}, 1 year: {:.3}",
                                    summary.e_usage_kwh_two_days,
                                    summary.e_usage_kwh_week,
                                    summary.e_usage_kwh_two_weeks,
                                    summary.e_usage_kwh_four_weeks,
                                    summary.e_usage_kwh_month,
                                    summary.e_usage_kwh_two_months,
                                    summary.e_usage_kwh_three_months,
                                    summary.e_usage_kwh_six_months,
                                    summary.e_usage_kwh_year,
                                );

                                println!(
                                    "[DEBUG] Gas Usage Summary: usage_kwh = 2d : {:.3}, current week: {:.3}, 2 weeks: {:.3}, 4 weeks: {:.3}, current month: {:.3}, 2 months: {:.3}, 3 months: {:.3}, 6 months: {:.3}, 1 year: {:.3}",
                                    summary.g_usage_kwh_two_days,
                                    summary.g_usage_kwh_week,
                                    summary.g_usage_kwh_two_weeks,
                                    summary.g_usage_kwh_four_weeks,
                                    summary.g_usage_kwh_month,
                                    summary.g_usage_kwh_two_months,
                                    summary.g_usage_kwh_three_months,
                                    summary.g_usage_kwh_six_months,
                                    summary.g_usage_kwh_year,
                                );

                                println!(
                                    "[DEBUG] Carbon Usage Summary: usage_grams = 2d : {:.3}, current week: {:.3}, 2 weeks: {:.3}, 4 weeks: {:.3}, current month: {:.3}, 2 months: {:.3}, 3 months: {:.3}, 6 months: {:.3}, 1 year: {:.3}",
                                    summary.carbon_intensity_two_days,
                                    summary.carbon_intensity_week,
                                    summary.carbon_intensity_two_weeks,
                                    summary.carbon_intensity_four_weeks,
                                    summary.carbon_intensity_month,
                                    summary.carbon_intensity_two_months,
                                    summary.carbon_intensity_three_months,
                                    summary.carbon_intensity_six_months,
                                    summary.carbon_intensity_year,
                                );
                            }
                            Err(e) => {
                                error_counter.inc();
                                eprintln!("[DEBUG] Error fetching  usage: {e}");
                            }
                        }
                        println!("[DEBUG] Sleeping for {interval} seconds before next metrics push.");
                        time::sleep(Duration::from_secs(interval)).await;
                    }
                });
            }

            // Set up web server routes
            let metrics_route = {
                let registry = Arc::clone(&registry);
                warp::path!("metrics").map(move || {
                    let encoder = TextEncoder::new();
                    let metric_families = registry.gather();
                    let mut buffer = Vec::new();
                    encoder.encode(&metric_families, &mut buffer).unwrap();
                    let metrics = String::from_utf8(buffer).unwrap();

                    warp::http::Response::builder()
                        .header("Content-Type", "text/plain; version=0.0.4")
                        .body(metrics)
                })
            };

            let health_route = warp::path!("health").map(|| "OK");
            let routes = metrics_route.or(health_route);

            println!("Starting server on http://localhost:9090");
            println!("Metrics endpoint: http://localhost:9090/metrics");
            println!("Health endpoint: http://localhost:9090/health");

            // Run the server with optional timeout
            if let Some(timeout_future) = timeout_future {
                tokio::select! {
                    _ = warp::serve(routes).run(([127, 0, 0, 1], 9090)) => {},
                    _ = timeout_future => {
                        println!("Timeout reached after {timeout} seconds, shutting down...");
                    }
                }
            } else {
                warp::serve(routes)
                    .run(([127, 0, 0, 1], 9090))
                    .await;
            }
        }
    }

    Ok(())
}
