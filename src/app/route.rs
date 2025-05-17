use axum::extract::State;
use serde_json::json;
use urlencoding::encode;

use super::{ApiResult, AppState, Json};

pub async fn calculate(
    State(state): State<AppState>,
    Json(params): Json<RouteParams>,
) -> ApiResult<()> {
    let res = openroute_calculate(state, params).await?;

    todo!()
}

async fn openroute_calculate(
    state: AppState,
    params: RouteParams,
) -> Result<RouteResponse, anyhow::Error> {
    let coordinates: Vec<[f32; 2]> = params.waypoints.iter().map(|p| [p.0, p.1]).collect();

    let body = json!({
        "coordinates": coordinates
    });

    let res = state
        .http_client
        .post("https://api.openrouteservice.org/v2/directions/driving-car")
        .header("Authorization", state.openroute_key)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    let response = res.json().await?;

    Ok(response)
}

fn export_to_maps_url(params: RouteParams) {
    let center_lat = 1.2357379;
    let center_lng = -36.0811227;
    let zoom = 4;
    let mode = "3e0";

    let encoded_waypoints: Vec<String> = params
        .waypoints
        .iter()
        .map(|point| encode(&format!("{}, {}", point.0, point.1)).to_string())
        .collect();

    let url = format!(
        "https://www.google.com/maps/dir/{}/@{},{},{}z/data=!{}",
        encoded_waypoints.join("/"),
        center_lat,
        center_lng,
        zoom,
        mode,
    );
}

pub async fn get_suggested(State(state): State<AppState>) -> Json<Vec<()>> {
    todo!()
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RouteResponse {
    pub routes: Vec<CandidateRoute>,
}

#[derive(Deserialize)]
pub struct CandidateRoute {
    pub summary: Summary,
    pub geometry: String,
    // You can add more fields here if needed, e.g. segments, way_points, etc.
}

#[derive(Deserialize)]
pub struct Summary {
    pub distance: f64,
    pub duration: f64,
}

#[derive(Deserialize)]
pub struct RouteParams {
    waypoints: Vec<(f32, f32)>,
}

#[derive(Serialize)]
pub struct Route {
    waypoints: Vec<(f32, f32)>,
}
