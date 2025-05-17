use axum::extract::State;
use urlencoding::encode;

use super::{AppState, Json};

#[derive(serde::Deserialize)]
pub struct RouteParams {
    waypoints: Vec<String>,
}

#[derive(serde::Serialize)]
pub struct Route {
    waypoints: Vec<String>,
}

pub async fn calculate(
    State(state): State<AppState>,
    Json(params): Json<RouteParams>,
) -> Json<Route> {
    let center_lat = 1.2357379;
    let center_lng = -36.0811227;
    let zoom = 4;
    let mode = "3e0";

    let encoded_waypoints: Vec<String> = params
        .waypoints
        .iter()
        .map(|point| encode(point).to_string())
        .collect();

    let url = format!(
        "https://www.google.com/maps/dir/{}/@{},{},{}z/data=!{}",
        encoded_waypoints.join("/"),
        center_lat,
        center_lng,
        zoom,
        mode,
    );

    todo!()
}

pub async fn get_suggested(State(state): State<AppState>) -> Json<Vec<()>> {
    todo!()
}
