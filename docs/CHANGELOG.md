# CHANGELOG

## 0.2.0
new: Split k-v storage and usage via `KVStorage` trait(default kv is `sled`).

new: Send hot passkey update to tracker, a new `env` field is introduced.

new: Better logger system, now you can access log from control panel.

refine: Now personal torrent status return whether torrent is free.

fix: Eliminate previous unchecked sql statements.
## 0.1.0
Initial version