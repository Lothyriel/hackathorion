pub mod dto;

use anyhow::Error;
use dto::{PoiDto, RouteDto, RouteVm};
use futures::stream::TryStreamExt;
use mongodb::{
    Database,
    bson::{doc, oid::ObjectId},
};

use crate::app::poi::{Poi, PoiFilter};

type DbResult<T> = Result<T, Error>;

pub trait PoiRepository {
    async fn add(&self, poi: Poi) -> DbResult<()>;
    async fn get(&self, tags: PoiFilter) -> DbResult<Vec<PoiDto>>;
    async fn get_by_id(&self, id: ObjectId) -> DbResult<Option<PoiDto>>;
    async fn put(&self, id: ObjectId, poi: Poi) -> DbResult<()>;
}

pub trait RouteRepository {
    async fn get(&self) -> DbResult<Vec<RouteVm>>;
}

impl RouteRepository for Database {
    async fn get(&self) -> DbResult<Vec<RouteVm>> {
        let routes = self.collection("route").find(doc! {}).await?;

        let routes: Vec<RouteDto> = routes.try_collect().await?;

        let mut output = vec![];

        for route in routes {
            let mut waypoints = vec![];

            for w in route.waypoints {
                let waypoint = self.get_by_id(w).await?.unwrap();
                waypoints.push(waypoint);
            }

            output.push(RouteVm {
                waypoints,
                id: route.id,
                image: route.image,
                title: route.title,
            });
        }

        Ok(output)
    }
}

impl PoiRepository for Database {
    async fn add(&self, poi: Poi) -> DbResult<()> {
        self.collection("poi").insert_one(poi).await?;

        Ok(())
    }

    async fn get(&self, filter: PoiFilter) -> DbResult<Vec<PoiDto>> {
        let mut doc_filter = doc! {};

        if let Some(approved) = filter.approved {
            doc_filter.insert("approved", approved);
        }

        if !filter.tags.is_empty() {
            doc_filter.insert("tags", doc! { "$in": filter.tags });
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

    async fn get_by_id(&self, id: ObjectId) -> DbResult<Option<PoiDto>> {
        let filter = doc! { "_id": id };

        let result = self.collection("poi").find_one(filter).await?;

        Ok(result)
    }
}
