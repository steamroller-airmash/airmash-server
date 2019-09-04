use crate::task::TaskData;

use specs::prelude::*;
use std::time::Duration;

pub async fn delayed_delete(mut task: TaskData, ent: Entity, delay: Duration) {
	task.sleep_for(delay).await;

	task.world(|world: &mut World| {
		// If the entity doesn't exist anymore than that's not really
		// a problem so ignore the error
		let _ = world.delete_entity(ent);
	})
}
