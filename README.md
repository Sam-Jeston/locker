# Locker Server
An end-to-end encrypted message platform. Locker puts the onus back on users to manage their keypairs, similar to a classic crypto wallet. This way no 3rd party need be trusted, and the encrypted messages can safely be stored.

## Development

Our server simply listens for socket connections and stores the encrypted messages for conversations. On connection to channel the most recent encrypted messages are sent.

Dependencies:
 - Rust (> v1.29)
 - Have a postgres instance accessible on port 5432

### Useful references
https://github.com/diesel-rs/diesel
