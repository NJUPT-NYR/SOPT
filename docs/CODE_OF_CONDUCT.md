# CODE OF CONDUCT

## Safe
We do not like bugs, and most bugs can be prevented in production.
Thanks to Programming Language Theory and modern software design, we
now can check and avoid many bugs via type system.

We will make most of Rust's safety and eliminate bugs and security
issues as possible as we can.

## Configurable
We are committed to make SOPT friendly to most users. They can change
all site configurations without touching Rust source code.

All they must do is writing down some simple pure texts
and loading binary with Apache or Nginx.

## Performant
SOPT is fast enough to handle tons of requests. We used actix, one
of the most performant web frameworks.

We also reduce unnecessary rtt, memory copy and database communications.

## Light-weighted
Software becomes hard to maintain and loses so much elegance when
growing up too big. We do not like that and control the size of
this project.

SOPT is simple in APIs, database design, dependencies and source code.
Also, most of the codes are documented.