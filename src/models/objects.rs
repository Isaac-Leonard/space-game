use std::future::Future;

use futures::{future::BoxFuture, FutureExt};

pub use super::_entities::objects::{ActiveModel, Column, Entity, Model};
use super::coordinates;
use sea_orm::{entity::prelude::*, ActiveValue};
pub type Objects = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

pub enum Location<'a> {
    Coordinates(coordinates::ActiveModel),
    Contained(&'a Model),
}

// implement your read-oriented logic here
impl Model {
    pub async fn create(location: Location<'_>, db: &impl ConnectionTrait) -> Self {
        match location {
            Location::Contained(container) => ActiveModel {
                contained_by: ActiveValue::set(Some(container.id)),
                ..Default::default()
            }
            .insert(db)
            .await
            .unwrap(),
            Location::Coordinates(mut coords) => {
                let object = ActiveModel {
                    ..Default::default()
                }
                .insert(db)
                .await
                .unwrap();
                coords.object_id = ActiveValue::set(object.id);
                coords.insert(db).await.unwrap();
                object
            }
        }
    }
}

impl ObjectTrait for Model {
    fn get_id(&self) -> i32 {
        self.id
    }
    async fn get_object(&self, _db: &impl ConnectionTrait) -> Model {
        // Not happy with this clone but can't return a reference because the trait normally has to return new data from the database
        self.clone()
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}

pub trait ObjectTrait: ModelTrait + Send + Sync {
    fn get_id(&self) -> i32;

    fn get_object(&self, db: &impl ConnectionTrait) -> impl Future<Output = Model> + Send {
        async {
            Entity::find()
                .filter(Column::Id.eq(self.get_id()))
                .one(db)
                .await
                .unwrap()
                .unwrap()
        }
    }

    fn get_coords<'a, C>(&'a self, db: &'a C) -> BoxFuture<'a, coordinates::Model>
    where
        C: ConnectionTrait + Send + Sync,
    {
        async {
            let object = self.get_object(db).await;
            let container = object.get_container(db).await;
            if let Some(container) = container {
                container.get_coords(db).await
            } else {
                coordinates::Entity::find()
                    .filter(Column::Id.eq(object.id))
                    .one(db)
                    .await
                    .unwrap()
                    .unwrap()
            }
        }
        .boxed()
    }

    fn has_coordinates(
        &self,
        db: &(impl ConnectionTrait + Send),
    ) -> impl std::future::Future<Output = bool> + Send {
        async {
            let object = self.get_object(db).await;
            object.contained_by == None
        }
    }

    fn get_container(
        &self,
        db: &(impl ConnectionTrait + Send),
    ) -> impl Future<Output = Option<Model>> + Send {
        async {
            let object = self.get_object(db).await;
            if !object.has_coordinates(db).await {
                let container: Option<Model> = Entity::find()
                    .filter(Column::Id.eq(object.contained_by))
                    .one(db)
                    .await
                    .unwrap();
                // This should not be None so we unwrap to just add an extra layer of error checking without doing it properly
                Some(container.unwrap())
            } else {
                None
            }
        }
    }

    fn get_contained_objects(
        &self,
        db: &(impl ConnectionTrait + Send),
    ) -> impl Future<Output = Result<Vec<Model>, DbErr>> + Send {
        async {
            Entity::find()
                .filter(Column::ContainedBy.eq(self.get_id()))
                .all(db)
                .await
        }
    }
}
