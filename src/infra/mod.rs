mod dto;

use anyhow::Error;
use futures::stream::TryStreamExt;
use mongodb::{Database, bson::doc};

use crate::app::poi::Poi;

type DbResult<T> = Result<T, Error>;

pub trait PoiRepository {
    async fn add(&self, poi: Poi) -> DbResult<()>;
    async fn get(&self, tags: Vec<String>) -> DbResult<Vec<Poi>>;
}

impl PoiRepository for Database {
    async fn add(&self, poi: Poi) -> DbResult<()> {
        self.collection("poi").insert_one(poi).await?;

        Ok(())
    }

    async fn get(&self, tags: Vec<String>) -> DbResult<Vec<Poi>> {
        let filter = doc! {
            "tags": { "$in": tags }
        };

        let cursor = self.collection("poi").find(filter).await?;

        let results = cursor.try_collect().await?;

        Ok(results)
    }
}
