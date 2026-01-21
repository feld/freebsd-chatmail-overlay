# FreeBSD Chatmail Overlay

This is the overlay I use to build the packages for the [Chatmail Cookbook](https://github.com/feld/chatmail-cookbook). Some of these packages will never really make sense to publish into FreeBSD ports.

I have an incomplete port for `py-chatmaild` in here at the moment because upstream isn't versioniong their code very consistently at this time, so we deploy with a Python venv/pip like they do on Linux.

## Ports

`mail/dovecot`: has a single patch added and needs to be built with LUA enabled

`mail/filtermail`: Rust-based Postfix milter for enforcing only encrypted mails, per-account rate limiting

`mail/py-chatmaild`: as stated above

`net/chatmail-turn`: Rust based TURN/STUN server used for WebRTC audio/video calls

`net/iroh-relay`: Iroh Relay server also for TURN/STUN. Currently unused, but WebXDC apps use this functionality and currently depend on the official project's relays. Lack of authentication is why it is kept off for now as anyone in the world could use your relay
