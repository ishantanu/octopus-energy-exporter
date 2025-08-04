use std::error::Error;

use carbonintensity::{get_intensities};
use chrono::{DateTime, Utc};

// Example function to fetch usage and compute carbon
pub async fn get_carbon_intensity(
    usage_kwh: f64,
    region: carbonintensity::Region,
    period_from: &str,
    period_to:  Option<&str>,
) -> Result<f64, Box<dyn Error>> {
    let date = DateTime::parse_from_rfc3339(period_from)
        .unwrap()
        .with_timezone(&Utc);
    let pt = period_to.unwrap();
    
    let date1 = DateTime::parse_from_rfc3339(pt)
        .unwrap()
        .with_timezone(&Utc);

    // Convert back to ISO 8601 string
    let iso_string = date.format("%Y-%m-%dT%H:%MZ").to_string();
    let iso_string1 = date1.format("%Y-%m-%dT%H:%MZ").to_string();
    println!("Inside getci: {iso_string}, {:?}", iso_string1);
    let result = get_intensities(
        &carbonintensity::Target::Region(region),
        &iso_string,
        &Some(&iso_string1),
    ).await?;

    let avg_intensity: f64 = result
    .iter()
    .map(|(_dt, val)| *val as f64)
    .sum::<f64>() / result.len() as f64;

    let carbon_grams = usage_kwh * avg_intensity;
    println!("{avg_intensity}, {carbon_grams}");


    Ok(carbon_grams.into())
}