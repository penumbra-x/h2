# H2

A Tokio aware, HTTP/2 client & server implementation for Rust.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/rh2.svg)](https://crates.io/crates/rh2)
[![Documentation](https://docs.rs/rh2/badge.svg)][dox]

More information about this crate can be found in the [crate documentation][dox].

[dox]: https://docs.rs/rh2

## Features

* Client and server HTTP/2 implementation.
* Implements the full HTTP/2 specification.
* Passes [h2spec](https://github.com/summerwind/h2spec).
* Focus on performance and correctness.
* Built on [Tokio](https://tokio.rs).

## Non goals

This crate is intended to only be an implementation of the HTTP/2
specification. It does not handle:

* Managing TCP connections
* HTTP 1.0 upgrade
* TLS
* Any feature not described by the HTTP/2 specification.

This crate is now used by [hyper](https://github.com/hyperium/hyper), which will provide all of these features.

## Usage

To use `rh2`, first add this to your `Cargo.toml`:

```toml
[dependencies]
rh2 = "0.4"
```

Next, add this to your crate:

```rust
extern crate rh2;

use rh2::server::Connection;

fn main() {
    // ...
}
```

## FAQ

**How does h2 compare to [solicit] or [rust-http2]?**

The h2 library has implemented more of the details of the HTTP/2 specification
than any other Rust library. It also passes the [h2spec] set of tests. The h2
library is rapidly approaching "production ready" quality.

Besides the above, Solicit is built on blocking I/O and does not appear to be
actively maintained.

**Is this an embedded Java SQL database engine?**

[No](https://www.h2database.com).

[solicit]: https://github.com/mlalic/solicit
[rust-http2]: https://github.com/stepancheg/rust-http2
[h2spec]: https://github.com/summerwind/h2spec

## Accolades

The project is based on a fork of [h2](https://github.com/hyperium/h2).
