use crate::component::{AssociatedConnection, Name};
use crate::ecs::prelude::*;
use crate::event::PlayerLeave;
use crate::resource::{channel::OnPlayerLeave, socket::CloseEvent, Connections};

#[event_handler(deps = super::handle_connect)]
fn handle_disconnect<'a>(
    evt: &CloseEvent,
    assoc: &mut WriteStorage<'a, AssociatedConnection>,
    conns: &mut Write<'a, Connections>,
    entities: &Entities<'a>,
    channel: &mut Write<'a, OnPlayerLeave>,

    names: &ReadStorage<'a, Name>,
) {
    let entity = match conns.player(evt.socket) {
        Ok(ent) => ent,
        _ => return,
    };

    if let Some(player) = entity {
        channel.single_write(PlayerLeave {
            player: entities
                .borrow(player)
                .expect("Connections contained a dead entity"),
        });

        entities
            .delete(player)
            .expect("Connection closed for a dead entity");

        let _ = assoc.remove(player);

        if let Some(name) = names.get(player) {
            info!("Player '{}' has left the server", name.0);
        } else {
            info!("Unnamed player with id {:?} has left the server", player);
        }
    }

    if let Err(_) = conns.close(evt.socket) {
        debug!("Got close message for a dead socket {}", evt.socket)
    }
}
