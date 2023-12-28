use serde::Deserialize;
use reqwest::Error;

#[derive(Deserialize, Debug)]
pub struct FloodMonitoringStationLatestReading {
    value: f32
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct FloodMonitoringStationData {
    latestReading: FloodMonitoringStationLatestReading
}

#[derive(Deserialize, Debug)]
pub struct FloodMonitoringApiResponse {
    items: Vec<FloodMonitoringStationData>
}

#[tokio::main]
pub async fn get_river_levels(monitoring_station_id: i32)-> Result<FloodMonitoringApiResponse, Error> {
    let request_url = format!("https://environment.data.gov.uk/flood-monitoring/id/stations/{:?}/measures", monitoring_station_id);
    let response = reqwest::get(request_url)
                    .await?
                    .json::<FloodMonitoringApiResponse>()
                    .await?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(get_river_levels(2642).unwrap().items.len(), 1);
    }
}
