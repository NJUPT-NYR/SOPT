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

### Safe
We do not like bugs, and most bugs can be prevented in production.
Thanks to Programming Language Theory and modern software design, we
now can check and avoid many bugs via type system.

We will make most of Rust's safety and eliminate bugs and security 
issues as possible as we can.

### Configurable
We are committed to make SOPT friendly to most users. They can change
all site configurations without touching Rust source code. 

All they must do is writing down some simple pure texts 
and loading binary with Apache or Nginx.

### Performant
SOPT is fast enough to handle tons of requests. We used actix, one 
of the most performant web frameworks.

We also reduce unnecessary rtt, memory copy and database communications.

### Light-weighted
Software becomes hard to maintain and loses so much elegance when 
growing up too big. We do not like that and control the size of
this project.

SOPT is simple in APIs, database design, dependencies and source code.
Also, most of the codes are documented.

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
  - [x] Add a post
  - [x] Generate with passkey
  - [ ] Sync with tracker
  - [x] Tags filter
- [x] Admin
  - [x] User management
  - [x] Site general setting
  - [x] Torrent review
- [ ] Anti-Cheating
  - [ ] IP limitation
  - [ ] Monitor
- [ ] Search  

## Contribution
SOPT is now under active development. Any contribution is welcomed.
If you meet some problems, feel free to raise an issue on Github.

## License

SOPT is dual-licensed under MIT and APACHE 2.0.

Choose either as you like.
