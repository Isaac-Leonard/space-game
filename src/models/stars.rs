pub use super::_entities::stars::{ActiveModel, Entity, Model};
use super::{material_stars, objects::ObjectTrait, planets};
use futures::future::join_all;
use sea_orm::entity::prelude::*;
pub type Stars = Entity;

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

// implement your read-oriented logic here
impl Model {
    pub async fn get_mass<C>(&self, db: &C) -> f64
    where
        C: ConnectionTrait,
    {
        let materials = self
            .find_related(material_stars::Entity)
            .all(db)
            .await
            .unwrap();
        materials.into_iter().fold(0.0, |a, b| a + b.mass)
    }

    pub async fn get_planets(&self, db: &(impl ConnectionTrait + Send)) -> Vec<planets::Model> {
        let objects = self.get_contained_objects(db).await.unwrap();
        let planets = join_all(objects.into_iter().map(|o| async move {
            planets::Entity::find()
                .filter(planets::Column::Id.eq(o.id))
                .one(db)
                .await
                .unwrap()
                .unwrap()
        }))
        .await;
        planets
    }
}

impl ObjectTrait for Model {
    fn get_id(&self) -> i32 {
        self.id
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
