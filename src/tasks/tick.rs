use loco_rs::prelude::*;
use sea_orm::LoaderTrait;

use crate::models::{coordinates, objects};

pub struct Tick;
#[async_trait]
impl Task for Tick {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "tick".to_string(),
            detail: "Task generator".to_string(),
        }
    }

    async fn run(&self, ctx: &AppContext, _vars: &task::Vars) -> Result<()> {
        let coordinates = coordinates::Entity::find()
            .find_also_related(objects::Entity)
            .all(&ctx.db)
            .await
            .unwrap();
        Ok(())
    }
}
