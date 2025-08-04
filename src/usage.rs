use std::{collections::HashMap, env};
use chrono::{DateTime, Utc};
use octopust::{models::{ListElectrictyConsumptionQuery, ListGasConsumptionQuery}, Client};
use carbonintensity::Region;

use crate::carbon_intensity;


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
    pub carbon_intensity_two_days: f64,
    pub carbon_intensity_week: f64,
    pub carbon_intensity_two_weeks: f64,
    pub carbon_intensity_four_weeks: f64,
    pub carbon_intensity_month: f64,
    pub carbon_intensity_two_months: f64,
    pub carbon_intensity_three_months: f64,
    pub carbon_intensity_six_months: f64,
    pub carbon_intensity_year: f64,
}

pub async fn fetch_electricity_and_gas_consumption(
    client: &Client,
    period_to: &str,
    periods: &HashMap<String, DateTime<Utc>>,
    group_by_opts: &HashMap<String, &str>,
    region: &str,
) -> Result<Summary, Box<dyn std::error::Error>> {
    let mpan = env::var("MPAN").expect("MPAN env variable not set");
    let e_serial_number = env::var("E_SERIAL_NO").expect("E_SERIAL_NO env variable not set");
    let mprn = env::var("MPRN").expect("MPRN env variable not set");
    let g_serial_number = env::var("G_SERIAL_NO").expect("G_SERIAL_NO env variable not set");

    let mut carbon_region= Region::England;
    let mut e_usage_kwh_two_days = 0.0;
    let mut e_usage_kwh_week = 0.0;
    let mut e_usage_kwh_two_weeks = 0.0;
    let mut e_usage_kwh_four_weeks = 0.0;
    let mut e_usage_kwh_month = 0.0;
    let mut e_usage_kwh_two_months = 0.0;
    let mut e_usage_kwh_three_months = 0.0;
    let mut e_usage_kwh_six_months = 0.0;
    let mut e_usage_kwh_year = 0.0;

    let mut carbon_intensity_two_days: f64 = 0.0;
    let mut carbon_intensity_week: f64 = 0.0;
    let mut carbon_intensity_two_weeks: f64 = 0.0;
    let mut carbon_intensity_four_weeks: f64 = 0.0;
    let mut carbon_intensity_month: f64 = 0.0;
    let mut carbon_intensity_two_months: f64 = 0.0;
    let mut carbon_intensity_three_months: f64 = 0.0;
    let mut carbon_intensity_six_months: f64 = 0.0;
    let mut carbon_intensity_year: f64 = 0.0;

    let mut g_usage_kwh_two_days = 0.0;
    let mut g_usage_kwh_week = 0.0;
    let mut g_usage_kwh_two_weeks = 0.0;
    let mut g_usage_kwh_four_weeks = 0.0;
    let mut g_usage_kwh_month = 0.0;
    let mut g_usage_kwh_two_months = 0.0;
    let mut g_usage_kwh_three_months = 0.0;
    let mut g_usage_kwh_six_months = 0.0;
    let mut g_usage_kwh_year = 0.0;
    
    match region {
      "North Scotland" => carbon_region = Region::NorthScotland,
      "South Scotland" => carbon_region = Region::SouthScotland,
      "North West England" => carbon_region = Region::NorthWestEngland,
      "North East England" => carbon_region = Region::NorthEastEngland,
      "South Yorkshire" => carbon_region = Region::SouthYorkshire,
      "North Wales, Merseyside and Cheshire" => carbon_region = Region::NorthWalesMerseysideAndCheshire,
      "South Wales" => carbon_region = Region::SouthWales,
      "West Midlands" => carbon_region = Region::WestMidlands,
      "East Midlands" => carbon_region = Region::EastMidlands,
      "East England" => carbon_region = Region::EastEngland,
      "South West England" => carbon_region = Region::SouthWestEngland,
      "South England" => carbon_region = Region::SouthEngland,
      "London" => carbon_region = Region::London,
      "South East England" => carbon_region = Region::SouthEastEngland,
      "England" => carbon_region = Region::England,
      "Wales" => carbon_region = Region::Wales,
      "Scotland" => carbon_region = Region::Scotland,
      _ => { eprintln!("Warning: Unknown region '{region}' encountered.");}
    }
    // 1. Loop through both key and value
    for value in group_by_opts.values() {
        if *value == "hour" {
            for (key, value) in periods {
                println!("Key: {key}, Value: {value}");
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

                        println!("region {carbon_region}");

                        let period_from = &value.format("%Y-%m-%dT%H:%M:%SZ").to_string();
                        let ci = carbon_intensity::get_carbon_intensity(e_usage_kwh_two_days, carbon_region, period_from, Some(period_to));
                        carbon_intensity_two_days = ci.await.unwrap();
                        println!("carbon intensity {carbon_intensity_two_days}");
                        
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

                        let period_from = &value.format("%Y-%m-%dT%H:%M:%SZ").to_string();
                        let ci = carbon_intensity::get_carbon_intensity(e_usage_kwh_week, carbon_region, period_from, Some(period_to));
                        carbon_intensity_week = ci.await.unwrap();
                        println!("carbon intensity {carbon_intensity_week}");

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

                        let period_from = &value.format("%Y-%m-%dT%H:%M:%SZ").to_string();
                        let ci = carbon_intensity::get_carbon_intensity(e_usage_kwh_two_weeks, carbon_region, period_from, Some(period_to));
                        carbon_intensity_two_weeks = ci.await.unwrap();
                        println!("carbon intensity {carbon_intensity_two_weeks}");

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

                        let period_from = &value.format("%Y-%m-%dT%H:%M:%SZ").to_string();
                        let ci = carbon_intensity::get_carbon_intensity(e_usage_kwh_four_weeks, carbon_region, period_from, Some(period_to));
                        carbon_intensity_four_weeks = ci.await.unwrap();
                        println!("carbon intensity {carbon_intensity_four_weeks}");

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
                     
                        let period_from = &value.format("%Y-%m-%dT%H:%M:%SZ").to_string();
                        let ci = carbon_intensity::get_carbon_intensity(e_usage_kwh_month, carbon_region, period_from, Some(period_to));
                        carbon_intensity_month = ci.await.unwrap();
                        println!("carbon intensity {carbon_intensity_month}");

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

                        let period_from = &value.format("%Y-%m-%dT%H:%M:%SZ").to_string();
                        let ci = carbon_intensity::get_carbon_intensity(e_usage_kwh_two_months, carbon_region, period_from, Some(period_to));
                        carbon_intensity_two_months = ci.await.unwrap();
                        println!("carbon intensity {carbon_intensity_two_months}");

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

                     
                        let period_from = &value.format("%Y-%m-%dT%H:%M:%SZ").to_string();
                        let ci = carbon_intensity::get_carbon_intensity(e_usage_kwh_three_months, carbon_region, period_from, Some(period_to));
                        carbon_intensity_three_months = ci.await.unwrap();
                        println!("carbon intensity {carbon_intensity_three_months}");

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

                        let period_from = &value.format("%Y-%m-%dT%H:%M:%SZ").to_string();
                        let ci = carbon_intensity::get_carbon_intensity(e_usage_kwh_six_months, carbon_region, period_from, Some(period_to));
                        carbon_intensity_six_months = ci.await.unwrap();
                        println!("carbon intensity {carbon_intensity_six_months}");

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

                        let period_from = &value.format("%Y-%m-%dT%H:%M:%SZ").to_string();
                        let ci = carbon_intensity::get_carbon_intensity(e_usage_kwh_year, carbon_region, period_from, Some(period_to));
                        carbon_intensity_year = ci.await.unwrap();
                        println!("carbon intensity {carbon_intensity_year}");

                        g_usage_kwh_year = g_readings
                        .unwrap().results
                        .iter()
                        .map(|reading| reading.consumption)
                        .sum();
                     }
                     _ => { eprintln!("Warning: Unknown period key '{key}' encountered.");}
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
        carbon_intensity_two_days,
        carbon_intensity_week,
        carbon_intensity_two_weeks,
        carbon_intensity_four_weeks,
        carbon_intensity_month,
        carbon_intensity_two_months,
        carbon_intensity_three_months,
        carbon_intensity_six_months,
        carbon_intensity_year,
    })
}
