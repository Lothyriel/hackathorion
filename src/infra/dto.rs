use mongodb::bson::oid::ObjectId;
use mongodb::bson::serde_helpers::serialize_object_id_as_hex_string;
use serde::{Deserialize, Serialize};

use crate::app::poi::Coordinates;

#[derive(Deserialize, Serialize)]
pub struct ComercialPoiDto {
    #[serde(rename = "_id")]
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    id: ObjectId,
    name: String,
    description: String,
    image: String,
    coords: Coordinates,
    tags: Vec<String>,
    instagram: String,
    approved: bool,
}

#[derive(Deserialize, Serialize)]
pub struct TouristPoiDto {
    #[serde(rename = "_id")]
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    id: ObjectId,
    name: String,
    description: String,
    image: String,
    coords: Coordinates,
    tags: Vec<String>,
    approved: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PoiDto {
    Comercial(ComercialPoiDto),
    Tourist(TouristPoiDto),
}

#[derive(Serialize)]
pub struct RouteVm {
    #[serde(rename = "_id")]
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub waypoints: Vec<PoiDto>,
    pub image: String,
    pub title: String,
}

#[derive(Deserialize)]
pub struct RouteDto {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub waypoints: Vec<ObjectId>,
    pub image: String,
    pub title: String,
}
