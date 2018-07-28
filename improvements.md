
# Improvements

This file details a list of possible inprovements
that can be done with an airmash server.

### Protocol

- Use [WebRCT](https://webrtc.org/) as a primary 
  channel for communications.
- Generate protocol bindings from JSON declaration

### Anti-Cheat

- Do not send mimimap positions of prowlers that
  are nearby to the current player (counters prowler
  radar) (not sure if this is an improvement)
- Alternatively, turn up randomization for prowler
  positions by a large amount

### Prowlers
Multiple ideas here
- Prowlers aren't visible to anyone, not own team,
  not other prowlers
- Don't send minimap updates for nearby prowlers,
  can also be based on external situations such
	as when the flag is out of base

### Missions?
- Give new players easy missions that help the
  team tactically
	
### Missile Interactiosn
- Missiles "push" friendly players
- Missiles can shoot other missiles out of the air
- Friendly fire

### Spectating
- Let players spectate without being on a team,
  do this automatically or let it be opt-in
- (Maybe) Have players who are spectating extend
  the afk timer, (allow ~5? mins with no chat,
  and ~20 mins with chat, which would allow for
  players to do C&C)

### Moderation
- Allow moderators to kick, ban, votemute, etc.
- Would need accounts to be implemented
- Moderators should be chosen carefully, since they
  can make or break a community

### SpawnKilling Mitigation
- Disallow spawnkilling by preventing spawnkillers
  from being able to shoot planes that haven't
  moved since spawn. Either don't show them
  or make them indestructible.
- This should also not mess up flag carries through
  spawn, any mitigation could be disabled when a 
  flag is carried through spawn

### Chat Messages
- Duplicate flag taken/returned message to chatbox
- Add Flag Dropped server message (maybe)

### Custom Commands
- Align to ship?
- Server-time, game stats

### Votemutes
- Remove votemute and make it identical to ignore

### Custom Gamemodes
## Tag
- Drop flag when hit with missile

### Control Protocol
- Make the engine scriptable through a 
  separate protocol
- Allow a client to hook into all engine events
- Allows clients to be written in other languages too
