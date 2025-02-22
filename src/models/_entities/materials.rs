//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "materials")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    #[sea_orm(column_type = "Double")]
    pub density: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::material_planets::Entity")]
    MaterialPlanets,
    #[sea_orm(has_many = "super::material_stars::Entity")]
    MaterialStars,
}

impl Related<super::material_planets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MaterialPlanets.def()
    }
}

impl Related<super::material_stars::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MaterialStars.def()
    }
}

impl Related<super::planets::Entity> for Entity {
    fn to() -> RelationDef {
        super::material_planets::Relation::Planets.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::material_planets::Relation::Materials.def().rev())
    }
}

impl Related<super::stars::Entity> for Entity {
    fn to() -> RelationDef {
        super::material_stars::Relation::Stars.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::material_stars::Relation::Materials.def().rev())
    }
}
