use core::f64;

use loco_rs::prelude::*;

use crate::models::{_entities, coordinates, material_stars, materials, objects, planets, stars};

pub struct StarParams {
    pub kind: &'static str,
    pub min_multiplier: f64,
    pub max_multiplier: f64,
    pub abundance: f64,
}

const GALAXY_SIZE: f64 = 9.6e15 * 1000.0;

const STARTYPES: [StarParams; 3] = [
    StarParams {
        kind: "Brown Dwarf",
        min_multiplier: 0.007,
        max_multiplier: 0.01,
        abundance: 0.01,
    },
    StarParams {
        kind: "Main Sequence",
        min_multiplier: 0.7,
        max_multiplier: 1.5,
        abundance: 0.7,
    },
    StarParams {
        kind: "Red Dwarf",
        min_multiplier: 0.1,
        max_multiplier: 0.7,
        abundance: 0.29,
    },
];

const SUN_RADIUS: f64 = 700000000.0;
const SUN_TEMPERATURE: f64 = 5500.0;
const SUN_MASS: f64 = 1.988416e30;

const PLANET_TYPES: [&str; 5] = [
    "Earth like",
    "Gas GIant",
    "Super Earth",
    "Ocean World",
    "Ice Giant",
];

pub struct GenerateGalaxy;

#[async_trait]
impl Task for GenerateGalaxy {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "generate_galaxy".to_string(),
            detail: "Task generator".to_string(),
        }
    }

    async fn run(&self, ctx: &AppContext, _vars: &task::Vars) -> Result<()> {
        println!("Generating Galaxy");
        let stars = stars::Entity::delete_many().exec(&ctx.db).await.unwrap();
        eprintln!("{:?}", stars);
        let planets = planets::Entity::delete_many().exec(&ctx.db).await.unwrap();
        eprintln!("{:?}", planets);

        for i in 0..10000 {
            let params = get_random_star_params();

            let distance = rand::random::<f64>() * GALAXY_SIZE;
            let angle = rand::random::<f64>() * f64::consts::PI * 2.0;
            let x = angle.cos() * distance;
            let y = angle.sin() * distance;
            let star_object = objects::Model::create(
                objects::Location::Coordinates(coordinates::ActiveModel {
                    x: ActiveValue::set(x),
                    y: ActiveValue::set(y),
                    z: ActiveValue::set(0.0),
                    ..Default::default()
                }),
                &ctx.db,
            )
            .await;
            let star = stars::ActiveModel {
                id: ActiveValue::set(star_object.id),
                r#type: ActiveValue::set(params.kind.to_string()),
                radius: ActiveValue::set(gen_number_in_range(SUN_RADIUS, &params)),
                temperature: ActiveValue::set(gen_number_in_range(SUN_TEMPERATURE, &params)),
                ..Default::default()
            }
            .insert(&ctx.db)
            .await
            .unwrap();
            let hydrogen = materials::Entity::find()
                .filter(_entities::materials::Column::Name.eq("hydrogen"))
                .one(&ctx.db)
                .await
                .unwrap()
                .unwrap();
            let helium = materials::Entity::find()
                .filter(_entities::materials::Column::Name.eq("helium"))
                .one(&ctx.db)
                .await
                .unwrap()
                .unwrap();
            material_stars::ActiveModel {
                star_id: ActiveValue::set(star.id),
                material_id: ActiveValue::set(hydrogen.id),
                mass: ActiveValue::set(gen_number_in_range(SUN_MASS, &params) * 0.75),
                ..Default::default()
            }
            .insert(&ctx.db)
            .await
            .unwrap();
            material_stars::ActiveModel {
                star_id: ActiveValue::set(star.id),
                material_id: ActiveValue::set(helium.id),
                mass: ActiveValue::set(gen_number_in_range(SUN_MASS, &params) * 0.25),
                ..Default::default()
            }
            .insert(&ctx.db)
            .await
            .unwrap();
            if i % 500 == 0 {
                println!("Generated star {}: {:?}", i, star)
            }
            for _ in 0..((rand::random::<f64>() * 15.0).floor() as usize) {
                let planet_object =
                    objects::Model::create(objects::Location::Contained(&star_object), &ctx.db)
                        .await;
                let planet = planets::ActiveModel {
                    id: ActiveValue::set(planet_object.id),
                    r#type: ActiveValue::set(
                        PLANET_TYPES[(rand::random::<f64>() * 5.0).floor() as usize].to_string(),
                    ),
                    radius: ActiveValue::set(12000000.0),
                    // mass: ActiveValue::set(5e24),
                    ..Default::default()
                };
                planet.insert(&ctx.db).await.unwrap();
            }
        }
        Ok(())
    }
}

fn get_random_star_params() -> StarParams {
    let mut chance: f64 = rand::random();
    for star_params in STARTYPES {
        if chance <= star_params.abundance {
            return star_params;
        } else {
            chance -= star_params.abundance;
        }
    }
    panic!("Star abundances don't add up to 1, attempted to select unavailable params")
}

fn gen_number_in_range(solar_measure: f64, params: &StarParams) -> f64 {
    let range = params.max_multiplier + params.min_multiplier;
    let random: f64 = rand::random();
    solar_measure * (range * random - params.min_multiplier)
}
