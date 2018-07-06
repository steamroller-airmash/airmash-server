
# AIRMASH Server

This is an implementation of a server for the game
[AIRMASH](https://airma.sh). As of this moment it
aims to be fully compatible with the official 
servers.

## Building the server

The quickest way to start a test server is using 
docker. To do this run
```
docker-compose up
```
in the root directory of this repository.

For more in-depth dev work, it will be easier to use a local install
of rust nightly. To install rust see [here](https://www.rust-lang.org/en-US/install.html).

The central server code is located in `server`. Code for the CTF 
game mode is contained within `ctf`, `base` contains a game mode 
that has no addition features and should be used for testing.

To run a basic server locally, do
```
cargo run
```
within the `base` folder. Note that rust nightly is required.

## Logging in to the server

To access the server locally, run a server hosting 
[these](https://nofile.io/f/u9UnVHoGn71/static.zip) files locally, 
then open that server in a web browser (e.g. `localhost:8000`) and
use as a normal airmash client.
