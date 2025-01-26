#[allow(unused_imports)]
use loco_rs::{cli::playground, prelude::*};
use space_game::{app::App, models::users::users};

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    let ctx = playground::<App>().await.unwrap();

    let db = &ctx.db;
    let txn = db.begin().await.unwrap();

    txn.commit().await.unwrap();
    Ok(())
}
