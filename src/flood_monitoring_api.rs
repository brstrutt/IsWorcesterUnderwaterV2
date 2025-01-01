use gloo_net::http::Request;
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Clone, Copy, Debug)]
pub struct LatestReading {
    pub value: f32
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Data {
    pub latestReading: LatestReading
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub items: Vec<Data>
}

pub async fn get_river_levels(monitoring_station_id: i32)-> Result<Response, gloo_net::Error> {
    let request_url: String = format!("https://environment.data.gov.uk/flood-monitoring/id/stations/{:?}/measures", monitoring_station_id);

    let response_data = Request::get(&request_url)
        .send()
        .await?
        .json()
        .await?;

    Ok(response_data)
}

pub async fn get_latest_river_level(monitoring_station_id: i32)-> Result<LatestReading, String> {
    let data = get_river_levels(monitoring_station_id)
        .await;
    
    match data {
        Err(error) => Err(error.to_string()),
        Ok(data) => {
            match data.items.first() {
                Some(first_data) => Ok(first_data.latestReading),
                _ => Err(String::from("Returned object didn't contain any data")),
            }
        }
    }
}
