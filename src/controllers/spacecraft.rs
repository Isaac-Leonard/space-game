#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use axum::debug_handler;
use futures::StreamExt;
use loco_rs::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::models::{_entities::spacecrafts, coordinates, objects::ObjectTrait, users};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Coordinates {
    x: f64,
    y: f64,
    z: f64,
}

impl From<coordinates::Model> for Coordinates {
    fn from(value: coordinates::Model) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SpacecraftDescriptor {
    name: String,
    mass: f64,
    speed: f64,
    id: i32,
    coordinates: Coordinates,
}

impl From<(spacecrafts::Model, coordinates::Model)> for SpacecraftDescriptor {
    fn from((craft, coords): (spacecrafts::Model, coordinates::Model)) -> Self {
        Self {
            mass: craft.mass,
            name: craft.name,
            speed: craft.speed,
            id: craft.id,
            coordinates: coords.into(),
        }
    }
}

#[debug_handler]
pub async fn spacecrafts(
    auth: auth::JWTWithUser<users::Model>,
    State(ctx): State<AppContext>,
) -> Json<Vec<SpacecraftDescriptor>> {
    let craft = auth
        .user
        .find_related(spacecrafts::Entity)
        .all(&ctx.db)
        .await
        .unwrap();
    let craft = futures::stream::iter(craft)
        .then(|craft| async {
            let coords = craft.get_coords(&ctx.db).await;
            (craft, coords).into()
        })
        .collect()
        .await;
    Json(craft)
}

#[derive(Clone, Debug, PartialEq, Deserialize, JsonSchema)]
pub struct ShipId {
    id: i32,
}

#[debug_handler]
pub async fn spacecraft_details(
    Path(ship_id): Path<ShipId>,
    auth: auth::JWTWithUser<users::Model>,
    State(ctx): State<AppContext>,
) -> Json<SpacecraftDescriptor> {
    let craft = spacecrafts::Entity::find()
        .filter(spacecrafts::Column::OwnedBy.eq(auth.user.id))
        .filter(spacecrafts::Column::Id.eq(ship_id.id))
        .one(&ctx.db)
        .await
        .unwrap()
        .unwrap();
    let coords = craft.get_coords(&ctx.db).await;
    Json((craft, coords).into())
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/spacecrafts")
        .add("/", get(spacecrafts))
        .add("/{id}", get(spacecraft_details))
}
