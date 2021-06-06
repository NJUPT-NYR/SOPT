# CHANGELOG

## 0.2.1
new: auto configuration script `configure.py`.

refine: No more need for initial rank configuration.

refine: Add logger for tracker.

fix: Possible inconsistent in passkey filter update.

fix: possible CRLF attack in old `lettere` version.

## 0.2.0
new: Split k-v storage and usage via `KVStorage` trait(default kv is `sled`).

new: Send hot passkey update to tracker, a new `env` field is introduced.

new: Better logger system, now you can access log from control panel.

new: Now admin can award special rank to a user(see `/admin/user/award_rank` in API).

refine: Now personal torrent status return whether torrent is free.

fix: Eliminate previous unchecked sql statements.

fix: User cannot level up when rank's name is changed after.
## 0.1.0
Initial version