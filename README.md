# Manifest

[![CI](https://github.com/yawn/manifest/actions/workflows/rust.yml/badge.svg)](https://github.com/yawn/manifest/actions/workflows/rust.yml) [![Crates](https://img.shields.io/crates/v/manifest.svg)](https://crates.io/crates/manifest) [![Docs](https://docs.rs/manifest/badge.svg)](https://docs.rs/manifest)

This crate provides a mechanism to manage catalogues of messages with critical semantics. When used in a build script, this message catalog is
used to generate constants that must be used (or are marked as deprecated).

Optionally (gated behind the `objects` feature) static references to `Message` structs can be obtained as well. These structs expose additional
metadata about the message. They implement the `Display` trait to render an equivalent to the constant value and implement a `Message` trait
to enable passing message over crate boundaries.

Messages catalogues are useful for stakeholder communication (such as business sponsors, auditors or security teams) to communicate the entire
set of critical events or state changes within an application.

A message catalogue is stored in a file named `Manifest.toml` in the root of the project. It looks like this:

```toml
[1]
message = "user login"
comment = "User successfully logged in"

[2]
message = "login failed"
comment = "Authentication failed"

[100]
message = "deprecated feature"
comment = "This feature is no longer supported"
deprecated = "Use feature XYZ instead"

[5]
message = "password reset"
```

## Usage

Write a `Manifest.toml` and add `manifest` to the build dependencies:

```toml
[build-dependencies]
manifest = { version = "0.1", features = ["build"] }
```

Write a `build.rs` build script that calls the `manifest` crate to generate constants:

```rust,ignore
fn main() {
    manifest::build::generate();
}
```

Add `manifest` as dependency:

```toml
[dependencies]
manifest = { version = "0.1" }
```

Initialize the constant inclusion once and use it:

```rust,ignore
manifest::include_manifest!();

use crate::TEST_CRATE_00001_USER_LOGIN;

use tracing::trace;

fn main() {
    trace!(TEST_CRATE_00001_USER_LOGIN);
}
```

This example (using the manifest from above) would fail to compile since messages `2` and `5` are never used.

If a tracing subscriber would capture the `trace!` from above, the log message would look like this: `user login (TEST_CRATE_00001)`.
