use tokio::sync::oneshot::Sender;

use crate::ecs::prelude::*;
use crate::ecs::Builder;

#[derive(Default)]
pub struct AwakenQueue(pub(crate) Vec<Sender<()>>);

#[system]
pub fn awaken_frame_tasks<'a>(mut queue: Write<'a, AwakenQueue>) {
    for sender in queue.0.drain(..) {
        let _ = sender.send(());
    }
}

pub fn register(builder: &mut Builder) {
    builder.with::<awaken_frame_tasks>();
}
