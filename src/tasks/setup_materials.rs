use loco_rs::prelude::*;

use crate::models::materials;

const MATERIALS_LIST: [(&str, f64); 38] = [
    ("hydrogen", 1.0),
    ("helium", 1.0),
    ("lithium", 1.0),
    ("beryllium", 1.0),
    ("boron", 1.0),
    ("carbon", 1.0),
    ("nitrogen", 1.0),
    ("oxygen", 1.0),
    ("fluorine", 1.0),
    ("neon", 1.0),
    ("sodium", 1.0),
    ("magnesium", 1.0),
    ("aluminium", 1.0),
    ("silicon", 1.0),
    ("phosphorus", 1.0),
    ("sulfur", 1.0),
    ("chlorine", 1.0),
    ("argon", 1.0),
    ("potassium", 1.0),
    ("calcium", 1.0),
    ("scandium", 1.0),
    ("titanium", 1.0),
    ("vanadium", 1.0),
    ("chromium", 1.0),
    ("manganese", 1.0),
    ("iron", 1.0),
    ("cobalt", 1.0),
    ("nickel", 1.0),
    ("copper", 1.0),
    ("zinc", 1.0),
    ("gallium", 1.0),
    ("germanium", 1.0),
    ("arsenic", 1.0),
    ("selenium", 1.0),
    ("bromine", 1.0),
    ("krypton", 1.0),
    ("rubidium", 1.0),
    ("gold", 1.0),
];

pub struct SetupMaterials;

#[async_trait]
impl Task for SetupMaterials {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "setupMaterials".to_string(),
            detail: "Task generator".to_string(),
        }
    }

    async fn run(&self, ctx: &AppContext, _vars: &task::Vars) -> Result<()> {
        println!("Running Setup materials");
        for material in MATERIALS_LIST {
            let material = materials::ActiveModel {
                name: ActiveValue::set(material.0.to_string()),
                density: ActiveValue::set(material.1),
                ..Default::default()
            };
            let _material = material.insert(&ctx.db).await;
        }
        Ok(())
    }
}
