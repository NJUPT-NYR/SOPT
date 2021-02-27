# SOPT

[![CI](https://github.com/NJUPT-NYR/SOPT/actions/workflows/CI.yml/badge.svg)](https://github.com/NJUPT-NYR/SOPT/actions/workflows/CI.yml)

Next generation private tracker framework.

**世界を**

**大いに盛り上げるための**

**PT**

## How to run

1. Rename `.env.example` to `.env` and set your postgres name and password.

2. Run following commands in terminal:

```bash
createdb sopt
cargo install sqlx-cli
sqlx migrate run
cargo run
```

## Principle

## API

[API Docs](https://github.com/NJUPT-NYR/SOPT/blob/master/API.md)

## Roadmap

- [ ] User
  - [x] Register
  - [x] Login
  - [x] Information update
  - [x] Invite
  - [ ] Rank
- [ ] Torrent
  - [x] Add
  - [ ] Generate with passkey
  - [ ] Sync with tracker
  - [ ] Search and filter
- [ ] Admin
  - [ ] Ban user
  - [ ] Site general setting
  - [ ] Torrent info update
  - [ ] Open feature
- [ ] Anti-Cheating
  - [ ] IP limitation
  - [ ] Client Ban
  - [ ] Monitor

## Known issues

1. Support for TLS is needed.

2. Configurable Site for all

## License

SOPT is dual-licensed under MIT and APACHE 2.0.

Choose either as you like.
