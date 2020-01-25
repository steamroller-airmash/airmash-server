use server_tests::TestRunner;
use server_v2::Distance;
use server_v2_macros::client_test;

use std::error::Error;
use std::time::Duration;

use airmash_protocol::KeyCode;

/// Test to see that pressing the up key does, in fact, cause
/// the player to move.
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

/// Test to ensure that teleporting moves the current bot.
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

/// Test to ensure that running into a mountain causes the player
/// to bounce.
#[client_test]
async fn test_bounce(runner: TestRunner) -> Result<(), Box<dyn Error>> {
    let mut client = runner.new_client().await?;

    client.login("BounceBot").await?;

    // Basic Idea: Run up an hit a wall at an angle. If the collision
    //             code is working properly we'll be bounced to the
    //             the side after hitting the wall.
    client.send_command("teleport", "0 -4452 -6703").await?;
    client.press_key(KeyCode::Up).await?;
    client.wait(Duration::from_secs(1)).await?;
    client.release_key(KeyCode::Up).await?;

    let pos = client.world.get_me().pos;

    assert!(
        pos.x < Distance::new(-6704.0) || pos.x > Distance::new(-6702.0),
        "{}",
        pos.x
    );

    Ok(())
}

/// Fire a missile and ensure that it appears.
#[client_test]
async fn test_missile_fire(runner: TestRunner) -> Result<(), Box<dyn Error>> {
    let mut client = runner.new_client().await?;

    // Need to wait some time since the server initializes
    // the last-shot time to the time it was started.
    client.wait(Duration::from_millis(1000)).await?;
    client.login("MissileBot").await?;

    client.press_key(KeyCode::Fire).await?;
    client.wait(Duration::from_millis(100)).await?;
    client.release_key(KeyCode::Fire).await?;
    client.wait(Duration::from_millis(500)).await?;

    let num_missiles = client
        .world
        .mobs
        .iter()
        .filter(|mob| mob.1.missile())
        .count();
    assert_ne!(num_missiles, 0);

    Ok(())
}

/// Join and disconnect a bunch of bots in order to trigger a crash
#[client_test]
async fn test_join_leave_crash(runner: TestRunner) -> Result<(), Box<dyn Error>> {
    let mut c1 = runner.new_client().await?;
    let mut c2 = runner.new_client().await?;
    let mut c3 = runner.new_client().await?;

    c1.login("B1").await?;
    c2.login("B2").await?;
    c1.wait(Duration::from_millis(100)).await?;
    c1.quit().await?;
    c3.login("B3").await?;
    c2.quit().await?;
    c3.quit().await?;

    Ok(())
}
