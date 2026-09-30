# FreeBSD Chatmail Overlay

This is the overlay I use to build the packages for the [Chatmail Cookbook](https://github.com/feld/chatmail-cookbook). Some of these packages will never really make sense to publish into FreeBSD ports.

I have an incomplete port for `py-chatmaild` in here at the moment because upstream isn't versioniong their code very consistently at this time, so we deploy with a Python venv/pip like they do on Linux.

## Ports

`mail/dovecot`: has a single patch added and LUA is enabled by default. Uses PORTEPOCH to ensure priority.

`mail/filtermail`: Rust-based Postfix milter for enforcing only encrypted mails, per-account rate limiting

`mail/py-chatmaild`: as stated above

`net/chatmail-turn`: Rust based TURN/STUN server used for WebRTC audio/video calls

## Poudriere

Assuming you're building these packages with Poudriere and you deployed this
overlay under the name "chatmail", you should create a
`/usr/local/etc/poudriere.d/chatmail-make.conf` file with these
contents:

```
mail_dovecot_SET= LUA

OVERLAYS+=/usr/local/poudriere/ports/chatmail/
UID_FILES=${PORTSDIR}/UIDs /overlays/chatmail/UIDs.local
GID_FILES=${PORTSDIR}/GIDs /overlays/chatmail/GIDs.local

.if ${.CURDIR} == ${PORTSDIR}/mail/dovecot
EXTRA_PATCHES+= /overlays/chatmail/patches/dovecot/patch-debounce
.endif
```
