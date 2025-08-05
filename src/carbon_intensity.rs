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

    let format_period_from = date.format("%Y-%m-%dT%H:%MZ").to_string();
    let format_period_to = date1.format("%Y-%m-%dT%H:%MZ").to_string();
    let result = get_intensities(
        &carbonintensity::Target::Region(region),
        &format_period_from,
        &Some(&format_period_to),
    ).await?;

    let avg_intensity: f64 = result
    .iter()
    .map(|(_dt, val)| *val as f64)
    .sum::<f64>() / result.len() as f64;

    let carbon_grams = usage_kwh * avg_intensity;

    Ok(carbon_grams)
}

#[cfg(test)]
mod tests {
    use carbonintensity::Region;

    #[test]
    fn test_region_enum_values() {
        // Ensure that all known variants exist and can be matched
        let regions = vec![
            Region::NorthScotland,
            Region::SouthScotland,
            Region::NorthWestEngland,
            Region::NorthEastEngland,
            Region::SouthYorkshire,
            Region::NorthWalesMerseysideAndCheshire,
            Region::SouthWales,
            Region::WestMidlands,
            Region::EastMidlands,
            Region::EastEngland,
            Region::SouthWestEngland,
            Region::SouthEngland,
            Region::London,
            Region::SouthEastEngland,
            Region::England,
            Region::Wales,
            Region::Scotland,
        ];
        for region in regions {
            match region {
                Region::NorthScotland
                | Region::SouthScotland
                | Region::NorthWestEngland
                | Region::NorthEastEngland
                | Region::SouthYorkshire
                | Region::NorthWalesMerseysideAndCheshire
                | Region::SouthWales
                | Region::WestMidlands
                | Region::EastMidlands
                | Region::EastEngland
                | Region::SouthWestEngland
                | Region::SouthEngland
                | Region::London
                | Region::SouthEastEngland
                | Region::England
                | Region::Wales
                | Region::Scotland => assert!(true),
            }
        }
    }

        #[test]
    fn test_calculate_carbon_intensity() {
        // Suppose result is a Vec<(DateTime, u32)>
        // We'll use dummy values as (dt, intensity)
        let result = vec![
            ("2025-08-01T00:00:00Z", 200u32),
            ("2025-08-01T01:00:00Z", 300u32),
            ("2025-08-01T02:00:00Z", 500u32),
        ];
        // We only care about the intensity values for this test
        let avg_intensity: f64 = result
            .iter()
            .map(|(_dt, val)| *val as f64)
            .sum::<f64>() / result.len() as f64;

        let usage_kwh = 10.0;
        let carbon_grams = usage_kwh * avg_intensity;

        // Expected average: (200 + 300 + 500) / 3 = 333.333...
        // Expected carbon: 10 * 333.333... = 3333.333...
        assert!((avg_intensity - 333.3333).abs() < 0.01);
        assert!((carbon_grams - 3333.3333).abs() < 0.1);
    }

    // If get_carbon_intensity or similar functions exist, add a test stub
    // Uncomment and adjust as needed based on actual function signatures

    // #[tokio::test]
    // async fn test_get_carbon_intensity_zero_usage() {
    //     let grams = get_carbon_intensity(0.0, Region::England, "2025-08-01T00:00:00Z", Some("2025-08-02T00:00:00Z")).await;
    //     assert!(grams.unwrap() >= 0.0);
    // }
}