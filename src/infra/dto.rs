use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ComercialPoiDto {
    id: ObjectId,
    name: String,
    description: String,
    image: String,
    coords: (f32, f32),
    tags: Vec<String>,
    instagram: String,
    approved: bool,
}

#[derive(Deserialize, Serialize)]
pub struct TouristPoiDto {
    id: ObjectId,
    name: String,
    description: String,
    image: String,
    coords: (f32, f32),
    tags: Vec<String>,
    approved: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum PoiDto {
    Comercial(ComercialPoiDto),
    Tourist(TouristPoiDto),
}
