use server_tests::{run_test, TestRunner};

use std::error::Error;
use std::time::Duration;

use airmash_protocol::KeyCode;

async fn _test_movement(runner: TestRunner) -> Result<(), Box<dyn Error>> {
    let mut client = runner.new_client().await?;

    client.login("MoveBot").await?;

    let pos1 = client.world.get_me().pos;

    client.press_key(KeyCode::Up).await?;
    client.wait(Duration::from_millis(500)).await?;
    client.release_key(KeyCode::Up).await?;

    let pos2 = client.world.get_me().pos;

    assert!(pos1.y > pos2.y, "{} > {}", pos1.y, pos2.y);

    Ok(())
}

#[tokio::test]
async fn test_movement() {
    run_test(_test_movement, "test_movement").await;
}
