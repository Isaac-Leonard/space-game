#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20250116_132843_stars;
mod m20250116_133200_planets;
mod m20250116_133639_spacecrafts;
mod m20250116_171943_add_role_to_users;
mod m20250117_170553_add_temperature_to_stars;
mod m20250119_143038_add_volume_to_ships;
mod m20250119_153832_materials;
mod m20250119_154349_create_join_table_stars_and_materials;
mod m20250119_154507_add_density_to_materials;
mod m20250119_154624_create_join_table_planets_and_materials;
mod m20250120_073847_add_mass_to_material_stars;
mod m20250120_074419_add_mass_to_material_planets;
mod m20250120_075151_remove_mass_from_stars;
mod m20250120_075218_remove_mass_from_planets;
mod m20250120_121115_objects;
mod m20250120_151741_coordinates;
mod m20250120_152430_add_objects_ref_to_stars;
mod m20250120_153753_add_objects_ref_to_planets;
mod m20250120_153809_add_objects_ref_to_ships;
mod m20250120_154005_remove_x_and_y_and_z_from_ships;
mod m20250120_154406_remove_x_and_y_and_z_from_stars;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20250116_132843_stars::Migration),
            Box::new(m20250116_133200_planets::Migration),
            Box::new(m20250116_133639_spacecrafts::Migration),
            Box::new(m20250116_171943_add_role_to_users::Migration),
            Box::new(m20250117_170553_add_temperature_to_stars::Migration),
            Box::new(m20250119_143038_add_volume_to_ships::Migration),
            Box::new(m20250119_153832_materials::Migration),
            Box::new(m20250119_154349_create_join_table_stars_and_materials::Migration),
            Box::new(m20250119_154507_add_density_to_materials::Migration),
            Box::new(m20250119_154624_create_join_table_planets_and_materials::Migration),
            Box::new(m20250120_073847_add_mass_to_material_stars::Migration),
            Box::new(m20250120_074419_add_mass_to_material_planets::Migration),
            Box::new(m20250120_075151_remove_mass_from_stars::Migration),
            Box::new(m20250120_075218_remove_mass_from_planets::Migration),
            Box::new(m20250120_121115_objects::Migration),
            Box::new(m20250120_151741_coordinates::Migration),
            Box::new(m20250120_152430_add_objects_ref_to_stars::Migration),
            Box::new(m20250120_153753_add_objects_ref_to_planets::Migration),
            Box::new(m20250120_153809_add_objects_ref_to_ships::Migration),
            Box::new(m20250120_154005_remove_x_and_y_and_z_from_ships::Migration),
            Box::new(m20250120_154406_remove_x_and_y_and_z_from_stars::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
