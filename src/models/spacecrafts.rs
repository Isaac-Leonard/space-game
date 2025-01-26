pub use super::_entities::spacecrafts::{ActiveModel, Entity, Model};
use super::{
    coordinates,
    objects::{self, ObjectTrait},
};
use loco_rs::model::ModelResult;
use sea_orm::{entity::prelude::*, ActiveValue, TransactionTrait};
pub type Spacecrafts = Entity;

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

pub struct NewShip<'a> {
    pub name: &'a str,
    pub user: i32,
    pub speed: f64,
    pub mass: f64,
    pub r#type: String,
}

// implement your read-oriented logic here
impl Model {
    pub async fn create_ship_for_user(
        db: &(impl ConnectionTrait + TransactionTrait),
        params: &NewShip<'_>,
    ) -> ModelResult<Self> {
        let txn = db.begin().await?;

        let object = objects::Model::create(
            objects::Location::Coordinates(coordinates::ActiveModel {
                x: ActiveValue::set(0.0),
                y: ActiveValue::set(0.0),
                z: ActiveValue::set(0.0),
                ..Default::default()
            }),
            &txn,
        )
        .await;

        let spacecraft = ActiveModel {
            mass: ActiveValue::set(params.mass),
            speed: ActiveValue::set(params.speed),
            owned_by: ActiveValue::set(params.user),
            r#type: ActiveValue::set(params.r#type.to_string()),
            name: ActiveValue::set(params.name.to_string()),
            object_id: ActiveValue::set(object.id),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        txn.commit().await?;

        Ok(spacecraft)
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
