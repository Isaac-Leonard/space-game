#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use itertools::Itertools;
use loco_rs::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    controllers::middleware::role::AdminUser,
    models::{spacecrafts, users},
};

#[derive(Clone, PartialEq, Debug, Serialize, JsonSchema)]
pub struct UserInfoForAdmins {
    pub pid: String,
    pub name: String,
    pub role: String,
}

#[debug_handler]
pub async fn users(
    _auth: AdminUser,
    State(ctx): State<AppContext>,
) -> Json<Vec<UserInfoForAdmins>> {
    let users = users::Entity::find().all(&ctx.db).await.unwrap();
    Json(
        users
            .into_iter()
            .map(|user| UserInfoForAdmins {
                pid: user.pid.to_string(),
                name: user.name.to_string(),
                role: user.role.to_string(),
            })
            .collect_vec(),
    )
}

#[derive(Clone, PartialEq, Debug, Serialize, JsonSchema)]
pub struct UserDetails {
    pub pid: String,
    pub name: String,
    pub role: String,
    pub ships: Vec<UserShipInfo>,
}

#[derive(Clone, PartialEq, Debug, Serialize, JsonSchema)]
pub struct UserShipInfo {
    id: i32,
    name: String,
    r#type: String,
}

#[derive(Clone, PartialEq, Debug, Deserialize, JsonSchema)]
pub struct Pid {
    pub pid: String,
}

async fn user_details(
    _auth: AdminUser,
    Path(pid): Path<Pid>,
    State(ctx): State<AppContext>,
) -> Json<UserDetails> {
    let user = users::Model::find_by_pid(&ctx.db, &pid.pid).await.unwrap();
    let ships = user
        .find_related(spacecrafts::Entity)
        .all(&ctx.db)
        .await
        .unwrap();
    Json(UserDetails {
        pid: user.pid.to_string(),
        name: user.name.to_string(),
        role: user.role.to_string(),
        ships: ships
            .into_iter()
            .map(|ship| UserShipInfo {
                id: ship.id,
                name: ship.name.to_string(),
                r#type: ship.r#type.to_string(),
            })
            .collect_vec(),
    })
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/users")
        .add("/", get(users))
        .add("/{pid}", get(user_details))
}
