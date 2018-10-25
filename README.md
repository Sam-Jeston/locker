# Locker Server
An end-to-end encrypted message platform. Locker puts the onus back on users to manage their keypairs, similar to a classic crypto wallet. This way no 3rd party need be trusted, and the encrypted messages can safely be stored.

## Development

Our server simply listens for socket connections and stores / sends the encrypted messages for conversation channels.

A single websocket is used to push messages to the client, while the static routes use Rocket.

Dependencies:
 - Rust (> v1.31)
 - Have a postgres instance accessible on port 5432

### TODOs
1. Remove ws references from HashMap on socket disconnect to free memory
2. Lots of instances of unwrap(). Match these properly. Also clean up unconsumed Result types.
3. Page the message data. This implementation will depend on how this is approached in the client.
4. Clean up the sockets module. Both the matching on incoming messages and make it more modular in general.
