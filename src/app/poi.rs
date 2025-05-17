use axum::extract::{Query, State};
use serde::{Deserialize, Serialize};

use crate::infra::PoiRepository;

use super::{ApiResult, AppState, Json, MessageResponse};

pub async fn get(
    State(state): State<AppState>,
    Query(query): Query<TagsQuery>,
) -> ApiResult<Vec<Poi>> {
    let points = state.db().get(query.tags).await?;

    Ok(Json(points))
}

pub async fn add(
    State(state): State<AppState>,
    Json(params): Json<Poi>,
) -> ApiResult<MessageResponse> {
    state.db().add(params).await?;

    Ok(MessageResponse::new("resource added"))
}

#[derive(Deserialize)]
pub struct TagsQuery {
    tags: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ComercialPoi {
    name: String,
    description: String,
    image: String,
    cord: (f32, f32),
    tags: Vec<String>,
    instagram: String,
}

#[derive(Deserialize, Serialize)]
pub struct TouristPoi {
    name: String,
    description: String,
    image: String,
    cord: (f32, f32),
    tags: Vec<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Poi {
    Comercial(ComercialPoi),
    Tourist(TouristPoi),
}
