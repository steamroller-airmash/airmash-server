use crate::component::{
    flag::{IsChatMuted, IsChatThrottled, IsPlayer},
    Position, Team,
};
use crate::ecs::prelude::*;
use crate::protocol::{client, server, ErrorType};
use crate::resource::packet::ClientPacket;
use crate::sysdata::Connections;

/// System that, when a chat message is received, sends it to
/// all players in the game.
#[event_handler]
fn handle_chat<'a>(
    evt: &ClientPacket<client::Chat<'static>>,

    is_muted: &ReadStorage<'a, IsChatMuted>,
    is_throttled: &ReadStorage<'a, IsChatThrottled>,
    conns: &Connections<'a>,
) {
    let player = match conns.player(evt.connection) {
        Ok(Some(player)) => player,
        _ => return,
    };

    if is_throttled.get(player).is_some() | is_muted.get(player).is_some() {
        conns.send_to_player(
            player,
            server::Error {
                error: ErrorType::ChatThrottled,
            },
        );
        return;
    }

    conns.send_to_all(server::ChatPublic {
        id: player.into(),
        text: evt.packet.text.clone(),
    });
}

/// System that, when a team chat message is received, sends it to
/// all players on the sending player's team.
#[event_handler]
fn handle_team_chat<'a>(
    evt: &ClientPacket<client::TeamChat<'static>>,

    is_muted: &ReadStorage<'a, IsChatMuted>,
    is_throttled: &ReadStorage<'a, IsChatThrottled>,
    conns: &Connections<'a>,
    team: &ReadStorage<'a, Team>,
) {
    let player = match conns.player(evt.connection) {
        Ok(Some(player)) => player,
        _ => return,
    };

    if is_throttled.get(player).is_some() | is_muted.get(player).is_some() {
        conns.send_to_player(
            player,
            server::Error {
                error: ErrorType::ChatThrottled,
            },
        );
        return;
    }

    let team = *try_get!(player, team);

    conns.send_to_team(
        team,
        server::ChatTeam {
            id: player.into(),
            text: evt.packet.text.clone(),
        },
    );
}

/// System that, when a whisper packet is received, sends it to
/// the target player.
#[event_handler]
fn handle_whisper<'a>(
    evt: &ClientPacket<client::Whisper<'static>>,

    is_muted: &ReadStorage<'a, IsChatMuted>,
    is_throttled: &ReadStorage<'a, IsChatThrottled>,
    is_player: &ReadStorage<'a, IsPlayer>,
    conns: &Connections<'a>,
    entities: &Entities<'a>,
) {
    let player = match conns.player(evt.connection) {
        Ok(Some(player)) => player,
        _ => return,
    };

    if is_throttled.get(player).is_some() || is_muted.get(player).is_some() {
        conns.send_to_player(
            player,
            server::Error {
                error: ErrorType::ChatThrottled,
            },
        );
        return;
    }

    let target = entities.forge(evt.packet.id.0 as u32);
    if !entities.is_alive(target) || is_player.get(target).is_none() {
        // TODO: Send an error message here?
        //       This should never happen using the normal client since
        //       it checks the name before sending.
        return;
    }

    conns.send_to_player(
        target,
        server::ChatWhisper {
            from: player.into(),
            to: target.into(),
            text: evt.packet.text.clone(),
        },
    );
    conns.send_to_player(
        player,
        server::ChatWhisper {
            from: player.into(),
            to: target.into(),
            text: evt.packet.text.clone(),
        },
    );
}

/// System that, when a say packet is received, sends it to
/// all players that are within visible range.
#[event_handler]
fn handle_say<'a>(
    evt: &ClientPacket<client::Say<'static>>,

    is_muted: &ReadStorage<'a, IsChatMuted>,
    is_throttled: &ReadStorage<'a, IsChatThrottled>,
    pos: &ReadStorage<'a, Position>,
    conns: &Connections<'a>,
) {
    let player = match conns.player(evt.connection) {
        Ok(Some(player)) => player,
        _ => return,
    };

    if is_throttled.get(player).is_some() | is_muted.get(player).is_some() {
        conns.send_to_player(
            player,
            server::Error {
                error: ErrorType::ChatThrottled,
            },
        );
        return;
    }

    let pos = *try_get!(player, pos);
    conns.send_to_visible(
        pos,
        server::ChatSay {
            id: player.into(),
            text: evt.packet.text.clone(),
        },
    );
}
