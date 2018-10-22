
# Required Votemutes for Game Size

The number of votes required to mute a player is given
by the formula
```js
Math.floor(Math.sqrt(player_count)) + 1
```

where `player_count` is given by
```js
Players.count()[1]
```
