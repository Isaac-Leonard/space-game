#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use itertools::Itertools;
use loco_rs::prelude::*;
use schemars::JsonSchema;
use serde::Serialize;

use crate::{
    controllers::middleware::role::AdminUser,
    models::{_entities::spacecrafts, users},
};

#[derive(Clone, PartialEq, Debug, Serialize, JsonSchema)]
pub struct AdminCraftInfo {
    id: i32,
    name: String,
    user: String,
    user_name: String,
    r#type: String,
}

#[debug_handler]
pub async fn spacecrafts(
    _auth: AdminUser,
    State(ctx): State<AppContext>,
) -> Json<Vec<AdminCraftInfo>> {
    let crafts = spacecrafts::Entity::find()
        .find_also_related(users::Entity)
        .all(&ctx.db)
        .await
        .unwrap();
    Json(
        crafts
            .into_iter()
            .map(|(craft, user)| {
                let user = user.unwrap();
                AdminCraftInfo {
                    id: craft.id,
                    name: craft.name.to_string(),
                    user: user.pid.to_string(),
                    user_name: user.name.to_string(),
                    r#type: craft.r#type.to_string(),
                }
            })
            .collect_vec(),
    )
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/spacecrafts")
        .add("/", get(spacecrafts))
}
