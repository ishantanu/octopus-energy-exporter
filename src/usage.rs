use std::{collections::HashMap, env};
use chrono::{DateTime, Utc};
use octopust::{models::{ListElectrictyConsumptionQuery, ListGasConsumptionQuery}, Client};


pub struct Summary {
    pub e_usage_kwh_two_days: f64,
    pub e_usage_kwh_week: f64,
    pub e_usage_kwh_two_weeks: f64,
    pub e_usage_kwh_four_weeks: f64,
    pub e_usage_kwh_month: f64,
    pub e_usage_kwh_two_months: f64,
    pub e_usage_kwh_three_months: f64,
    pub e_usage_kwh_six_months: f64,
    pub e_usage_kwh_year: f64,
    pub g_usage_kwh_two_days: f64,
    pub g_usage_kwh_week: f64,
    pub g_usage_kwh_two_weeks: f64,
    pub g_usage_kwh_four_weeks: f64,
    pub g_usage_kwh_month: f64,
    pub g_usage_kwh_two_months: f64,
    pub g_usage_kwh_three_months: f64,
    pub g_usage_kwh_six_months: f64,
    pub g_usage_kwh_year: f64,
}

pub async fn fetch_electricity_and_gas_consumption(
    client: &Client,
    period_to: &str,
    periods: &HashMap<String, DateTime<Utc>>,
    group_by_opts: &HashMap<String, &str>,
) -> Result<Summary, Box<dyn std::error::Error>> {
    let mpan = env::var("MPAN").expect("MPAN env variable not set");
    let e_serial_number = env::var("E_SERIAL_NO").expect("SERIAL_NO env variable not set");
    let mprn = env::var("MPRN").expect("MPRN env variable not set");
    let g_serial_number = env::var("G_SERIAL_NO").expect("GAS_SERIAL_NO env variable not set");

    let mut e_usage_kwh_two_days = 0.0;
    let mut e_usage_kwh_week = 0.0;
    let mut e_usage_kwh_two_weeks = 0.0;
    let mut e_usage_kwh_four_weeks = 0.0;
    let mut e_usage_kwh_month = 0.0;
    let mut e_usage_kwh_two_months = 0.0;
    let mut e_usage_kwh_three_months = 0.0;
    let mut e_usage_kwh_six_months = 0.0;
    let mut e_usage_kwh_year = 0.0;

    let mut g_usage_kwh_two_days = 0.0;
    let mut g_usage_kwh_week = 0.0;
    let mut g_usage_kwh_two_weeks = 0.0;
    let mut g_usage_kwh_four_weeks = 0.0;
    let mut g_usage_kwh_month = 0.0;
    let mut g_usage_kwh_two_months = 0.0;
    let mut g_usage_kwh_three_months = 0.0;
    let mut g_usage_kwh_six_months = 0.0;
    let mut g_usage_kwh_year = 0.0;
    
    // 1. Loop through both key and value
    for value in group_by_opts.values() {
        if *value == "hour" {
            println!("Inside hour");
            for (key, value) in periods {
                println!("Key: {}, Value: {}", key, value);
                 let e_readings = client.list_electricity_consumption(ListElectrictyConsumptionQuery { 
                    mpan: &mpan, 
                    group_by: Some("hour"), 
                    serial_number: &e_serial_number, 
                    period_from:Some(&value.format("%Y-%m-%dT%H:%M:%SZ").to_string()) , 
                    period_to: Some(period_to), 
                    page_size: Some(10000),
                    ..Default::default()
                 }).await;

                 let g_readings = client.list_gas_consumption(ListGasConsumptionQuery{
                    mprn: &mprn,
                    serial_number: &g_serial_number,
                    group_by: Some("hour"),
                    period_to: Some(period_to),
                    period_from: Some(&value.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                    page_size: Some(10000),
                    ..Default::default()
                }).await;


                 match key.as_str() {
                     "2d" => {
                         e_usage_kwh_two_days = e_readings
                        .unwrap().results
                        .iter()
                        .map(|reading| reading.consumption)
                        .sum();

                        g_usage_kwh_two_days = g_readings
                        .unwrap().results
                        .iter()
                        .map(|reading| reading.consumption)
                        .sum();
                    
                     }

                     "1w" => {
                        e_usage_kwh_week = e_readings
                        .unwrap().results
                        .iter()
                        .map(|reading| reading.consumption)
                        .sum();

                        g_usage_kwh_week = g_readings
                        .unwrap().results
                        .iter()
                        .map(|reading| reading.consumption)
                        .sum();
                     }

                     "2w" => {
                        e_usage_kwh_two_weeks = e_readings
                        .unwrap().results
                        .iter()
                        .map(|reading| reading.consumption)
                        .sum();

                        g_usage_kwh_two_weeks = g_readings
                        .unwrap().results
                        .iter()
                        .map(|reading| reading.consumption)
                        .sum();
                     }

                     "4w" => {
                        e_usage_kwh_four_weeks = e_readings
                        .unwrap().results
                        .iter()
                        .map(|reading| reading.consumption)
                        .sum();

                        g_usage_kwh_four_weeks = g_readings
                        .unwrap().results
                        .iter()
                        .map(|reading| reading.consumption)
                        .sum();
                     }

                     "1m" => {
                        e_usage_kwh_month = e_readings
                        .unwrap().results
                        .iter()
                        .map(|reading| reading.consumption)
                        .sum();

                        g_usage_kwh_month = g_readings
                        .unwrap().results
                        .iter()
                        .map(|reading| reading.consumption)
                        .sum();
                     }

                     "2m" => {
                        e_usage_kwh_two_months = e_readings
                        .unwrap().results
                        .iter()
                        .map(|reading| reading.consumption)
                        .sum();

                        g_usage_kwh_two_months = g_readings
                        .unwrap().results
                        .iter()
                        .map(|reading| reading.consumption)
                        .sum();
                     }

                     "3m" => {
                        e_usage_kwh_three_months = e_readings
                        .unwrap().results
                        .iter()
                        .map(|reading| reading.consumption)
                        .sum();

                        g_usage_kwh_three_months = g_readings
                        .unwrap().results
                        .iter()
                        .map(|reading| reading.consumption)
                        .sum();
                     }

                     "6m" => {
                        e_usage_kwh_six_months = e_readings
                        .unwrap().results
                        .iter()
                        .map(|reading| reading.consumption)
                        .sum();

                        g_usage_kwh_six_months = g_readings
                        .unwrap().results
                        .iter()
                        .map(|reading| reading.consumption)
                        .sum();
                     }

                     "1y" => {
                        e_usage_kwh_year = e_readings
                        .unwrap().results
                        .iter()
                        .map(|reading| reading.consumption)
                        .sum();

                        g_usage_kwh_year = g_readings
                        .unwrap().results
                        .iter()
                        .map(|reading| reading.consumption)
                        .sum();
                     }
                     _ => { println!("test");}
                 }

            }

        }
    }

    Ok(Summary {
        e_usage_kwh_two_days,
        e_usage_kwh_week,
        e_usage_kwh_two_weeks,
        e_usage_kwh_four_weeks,
        e_usage_kwh_month,
        e_usage_kwh_two_months,
        e_usage_kwh_three_months,
        e_usage_kwh_six_months,
        e_usage_kwh_year,
        g_usage_kwh_two_days,
        g_usage_kwh_week,
        g_usage_kwh_two_weeks,
        g_usage_kwh_four_weeks,
        g_usage_kwh_month,
        g_usage_kwh_two_months,
        g_usage_kwh_three_months,
        g_usage_kwh_six_months,
        g_usage_kwh_year,
    })
}