#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use std::collections::HashMap;

use axum::debug_handler;
use loco_rs::prelude::*;
use migration::{ExprTrait, IntoColumnRef};
use schemars::JsonSchema;
use sea_orm::{Condition, QueryOrder, QuerySelect};
use serde::Serialize;

use crate::models::{
    coordinates, material_stars, materials,
    objects::{self, ObjectTrait},
    spacecrafts, stars, users,
};

#[derive(Clone, Debug, PartialEq, Serialize, JsonSchema)]
pub struct StarDescriptor {
    id: i32,
    x: f64,
    y: f64,
    radius: f64,
    mass: f64,
    r#type: String,
}

#[debug_handler]
pub async fn near_by(
    auth: auth::JWTWithUser<users::Model>,
    State(ctx): State<AppContext>,
) -> Json<Vec<StarDescriptor>> {
    let ship = auth
        .user
        .find_related(spacecrafts::Entity)
        .one(&ctx.db)
        .await
        .unwrap()
        .unwrap();
    let coords = ship.get_coords(&ctx.db).await;
    let x_distance = coordinates::Column::X.into_column_ref().sub(coords.x);
    let y_distance = coordinates::Column::Y.into_column_ref().sub(coords.y);
    let distance_from_ship =
        (x_distance.clone().mul(x_distance)).add(y_distance.clone().mul(y_distance));

    let stars = coordinates::Entity::find()
        .filter(Condition::any().add(distance_from_ship.clone().lt((9.6e15 * 10.0_f64).powi(2))))
        .order_by_asc(distance_from_ship)
        .find_also_related(objects::Entity)
        .limit(20)
        .all(&ctx.db)
        .await
        .unwrap();

    let mut star_descriptors = vec![];
    for (star_coords, object) in stars {
        let star = stars::Entity::find()
            .filter(stars::Column::Id.eq(object.unwrap().id))
            .one(&ctx.db)
            .await
            .unwrap();
        let Some(star) = star else { continue };
        star_descriptors.push(StarDescriptor {
            id: star.id,
            x: star_coords.x,
            y: star_coords.y,
            radius: star.radius,
            mass: star.get_mass(&ctx.db).await,
            r#type: star.r#type.clone(),
        })
    }
    Json(star_descriptors)
}

#[derive(Clone, Debug, PartialEq, Serialize, JsonSchema)]
pub struct SystemDetails {
    pub id: i32,
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub composition: HashMap<String, f64>,
    pub planets: Vec<PlanetDescriptor>,
}

#[derive(Clone, Debug, PartialEq, Serialize, JsonSchema)]
pub struct PlanetDescriptor {
    r#type: String,
}

#[debug_handler]
pub async fn system_details(
    auth: auth::JWTWithUser<users::Model>,
    State(ctx): State<AppContext>,
) -> Json<SystemDetails> {
    let ship = auth
        .user
        .find_related(spacecrafts::Entity)
        .one(&ctx.db)
        .await
        .unwrap()
        .unwrap();

    let coords = ship.get_coords(&ctx.db).await;

    let x_distance = coordinates::Column::X.into_column_ref().sub(coords.x);
    let y_distance = coordinates::Column::Y.into_column_ref().sub(coords.y);
    let distance_from_ship =
        (x_distance.clone().mul(x_distance)).add(y_distance.clone().mul(y_distance));

    let star = stars::Entity::find()
        .filter(Condition::any().add(distance_from_ship.clone().lt((9.6e15 * 10.0_f64).powi(2))))
        .order_by_asc(distance_from_ship)
        .limit(1)
        .one(&ctx.db)
        .await
        .unwrap()
        .unwrap();
    let planets = star.get_planets(&ctx.db).await;
    let composition = star
        .find_related(material_stars::Entity)
        .find_also_related(materials::Entity)
        .all(&ctx.db)
        .await
        .unwrap();
    let composition = composition
        .into_iter()
        .map(|(amount, material)| (material.unwrap().name.clone(), amount.mass))
        .fold(HashMap::new(), |mut map, (name, mass)| {
            map.insert(name, mass);
            map
        });
    let star_coords = star.get_coords(&ctx.db).await;
    Json(SystemDetails {
        id: star.id,
        x: star_coords.x,
        y: star_coords.y,
        radius: star.radius,
        composition,
        planets: planets
            .into_iter()
            .map(|p| PlanetDescriptor { r#type: p.r#type })
            .collect(),
    })
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/systems")
        .add("/nearby", get(near_by))
        .add("/current_system", get(system_details))
}
