
# Movement Prediction

This is deobfuscated code from ``engine.js``, it executes at least logically
for every frame, but may execute partially according to some external clock
(``performance.now()``?).

It is responsible for maintaining the previous server position/keystate update
for a client until that client changes state causing the server to emit a new
canonical update.

Variables heavily renamed. It looks like the body of the ``for`` loop has been
inlined by Closure Compiler from some additional function, the same code could
obviously be much cleaner. Hunch is that the loop body used to execute at a
constant rate but that caused lag issues, and so they wrapped it in the weird
loop to sync it to display or network clock.

````javascript

// NFRAMES = fractional number of frames to process

// RNDFRAMES =
//      1 if NFRAMES<0.51,
//      else mostly whole number of frames

// FRAMEFRAC = fractional increase of power/speed/rotation per loop

var RNDFRAMES = NFRAMES > .51 ? Math.round(NFRAMES) : 1;
var FRAMEFRAC = NFRAMES / RNDFRAMES;
var TWOPI = 2 * Math.PI;
var BOOSTMUL = this.boost ? 1.5 : 1;

for (var frameI = 0; frameI < RNDFRAMES; frameI++) {
    this.energy += FRAMEFRAC * this.energyRegen;
    this.energy >= 1 && (this.energy = 1);
    this.health += FRAMEFRAC * this.healthRegen;
    this.health >= 1 && (this.health = 1);

    var speedDeltaAngle = -999;
    if(this.strafe) {
        if(this.keystate.LEFT) {
            speedDeltaAngle = this.rotation - .5 * Math.PI;
        }
        if(this.keystate.RIGHT) {
            speedDeltaAngle = this.rotation + .5 * Math.PI;
        }
    } else {
        if(this.keystate.LEFT) {
            this.rotation += -FRAMEFRAC * ship.turnFactor
        }
        if(this.keystate.RIGHT) {
            this.rotation += FRAMEFRAC * ship.turnFactor
        }
    }

    if(this.keystate.UP) {
        if(speedDeltaAngle == -999) {
            speedDeltaAngle = this.rotation;
        } else {
            speedDeltaAngle += Math.PI * (this.keystate.RIGHT ? -0.25 : 0.25);
        }
    } else if(this.keystate.DOWN) {
        if(speedDeltaAngle == -999) {
            speedDeltaAngle = this.rotation + Math.PI;
        } else {
            speedDeltaAngle = += Math.PI * (this.keystate.RIGHT ? 0.25 : -0.25)
        }
    }

    var speedX = this.speed.x;
    var speedY = this.speed.y;
    if(speedDeltaAngle != -999) {
      this.speed.x += Math.sin(speedDeltaAngle) * ship.accelFactor * FRAMEFRAC * BOOSTMUL;
      this.speed.y -= Math.cos(speedDeltaAngle) * ship.accelFactor * FRAMEFRAC * BOOSTMUL;
    }


    var curShipMaxSpeed = ship.maxSpeed * BOOSTMUL * ship.upgrades.speed.factor[this.speedupgrade];
    if(this.powerups.rampage) {
        curShipMaxSpeed *= 0.75;
    }

    if(this.flagspeed) {
        curShipMaxSpeed = 5;
    }

    var speedVecLength = this.speed.length();
    if(speedVecLength > curShipMaxSpeed) {
      this.speed.multiply(curShipMaxSpeed / speedVecLength);
    } else if(this.speed.x > ship.minSpeed || this.speed.x < -ship.minSpeed ||
              this.speed.y > ship.minSpeed || this.speed.y < -ship.minSpeed) {
        this.speed.x *= 1 - ship.brakeFactor * FRAMEFRAC;
        this.speed.y *= 1 - ship.brakeFactor * FRAMEFRAC;
    } else {
        this.speed.x = 0;
        this.speed.y = 0;
    }

    this.pos.x += FRAMEFRAC * speedX + .5 * (this.speed.x - speedX) * FRAMEFRAC * FRAMEFRAC;
    this.pos.y += FRAMEFRAC * speedY + .5 * (this.speed.y - speedY) * FRAMEFRAC * FRAMEFRAC;
    this.clientCalcs(FRAMEFRAC);
}

this.rotation = ((this.rotation % TWOPI) + TWOPI) % TWOPI);
if(-1 != game.gameType) {
     (this.pos.x < -16352 && (this.pos.x = -16352),
      this.pos.x > 16352 && (this.pos.x = 16352),
      this.pos.y < -8160 && (this.pos.y = -8160),
      this.pos.y > 8160 && (this.pos.y = 8160))
} else {
     (this.pos.x < -16384 && (this.pos.x += 32768),
      this.pos.x > 16384 && (this.pos.x -= 32768),
      this.pos.y < -8192 && (this.pos.y += 16384),
      this.pos.y > 8192 && (this.pos.y -= 16384));
}

````
