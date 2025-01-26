use space_game::app::App;
use loco_rs::{task, testing::prelude::*};

use loco_rs::boot::run_task;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_can_run_generate_galaxy() {
    let boot = boot_test::<App>().await.unwrap();

    assert!(
        run_task::<App>(&boot.app_context, Some(&"generate_galaxy".to_string()), &task::Vars::default())
            .await
            .is_ok()
    );
}
