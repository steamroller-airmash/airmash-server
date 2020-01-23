use server_tests::TestRunner;
use server_v2_macros::client_test;

use std::error::Error;
use std::time::Duration;

use airmash_protocol::KeyCode;

#[client_test]
async fn test_movement(runner: TestRunner) -> Result<(), Box<dyn Error>> {
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

#[client_test]
async fn test_teleport(runner: TestRunner) -> Result<(), Box<dyn Error>> {
    let mut client = runner.new_client().await?;

    client.login("Telebot").await?;

    client.send_command("teleport", "0 1000 1000").await?;
    // Need to wait for the client to send us an update
    client.wait(Duration::from_secs(2)).await?;

    let pos = client.world.get_me().pos;

    assert_eq!(pos.x, 1000.0.into());
    assert_eq!(pos.y, 1000.0.into());

    Ok(())
}
