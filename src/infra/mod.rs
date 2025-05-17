mod dto;

use anyhow::Error;
use futures::stream::TryStreamExt;
use mongodb::{
    Database,
    bson::{doc, oid::ObjectId},
};

use crate::app::poi::{Poi, PoiFilter};

type DbResult<T> = Result<T, Error>;

pub trait PoiRepository {
    async fn add(&self, poi: Poi) -> DbResult<()>;
    async fn get(&self, tags: PoiFilter) -> DbResult<Vec<Poi>>;
    async fn put(&self, id: ObjectId, poi: Poi) -> DbResult<()>;
}

impl PoiRepository for Database {
    async fn add(&self, poi: Poi) -> DbResult<()> {
        self.collection("poi").insert_one(poi).await?;

        Ok(())
    }

    async fn get(&self, filter: PoiFilter) -> DbResult<Vec<Poi>> {
        let mut doc_filter = doc! {
            "approved": filter.approved
        };

        if !filter.tags.is_empty() {
            doc_filter.insert("tags", doc! { "$in": filter.approved });
        }

        let cursor = self.collection("poi").find(doc_filter).await?;

        let results = cursor.try_collect().await?;

        Ok(results)
    }

    async fn put(&self, id: ObjectId, poi: Poi) -> DbResult<()> {
        let filter = doc! { "_id": id };

        self.collection("poi").replace_one(filter, poi).await?;

        Ok(())
    }
}
