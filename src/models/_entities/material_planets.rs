//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "material_planets")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key, auto_increment = false)]
    pub planet_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub material_id: i32,
    #[sea_orm(column_type = "Double")]
    pub mass: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::materials::Entity",
        from = "Column::MaterialId",
        to = "super::materials::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Materials,
    #[sea_orm(
        belongs_to = "super::planets::Entity",
        from = "Column::PlanetId",
        to = "super::planets::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Planets,
}

impl Related<super::materials::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Materials.def()
    }
}

impl Related<super::planets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Planets.def()
    }
}
