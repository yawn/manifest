# Manifest

[![CI](https://github.com/yawn/manifest/actions/workflows/rust.yml/badge.svg)](https://github.com/yawn/manifest/actions/workflows/rust.yml) [![Crates](https://img.shields.io/crates/v/manifest.svg)](https://crates.io/crates/manifest) [![Docs](https://docs.rs/manifest/badge.svg)](https://docs.rs/manifest)

This crate provides a mechanism to manage catalogues of messages with critical semantics. When used in a build script, this message catalog is
used to generate constants that must be used (or are marked as deprecated).

Optionally (gated behind the `objects` feature) static references to `Message` structs can be obtained as well. These structs expose additional
metadata about the message. They implement the `Display` trait to render an equivalent to the constant value and implement a `Message` trait
to enable passing messages over crate boundaries.

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
manifest = { version = "0.3", features = ["build"] }
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
manifest = { version = "0.3" }
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

## Objects

When the `objects` feature is enabled, a `Message` struct is generated alongside the constants:

```toml
[dependencies]
manifest = { version = "0.3", features = ["objects"] }
```

```rust,ignore
manifest::include_manifest!();

let msg = Message::lookup(TEST_CRATE_00001_USER_LOGIN);
assert_eq!(msg.id, 1);
assert_eq!(msg.message, "user login");
assert_eq!(msg.comment, Some("User successfully logged in"));
```

The `Message` trait allows passing messages across crate boundaries without depending on the generated types:

```rust,ignore
fn handle<T: manifest::Message>(msg: T) {
    println!("id: {}, message: {}", msg.id(), msg.message());

    if let Some(tags) = msg.tags() {
        println!("tags: {:?}", tags);
    }

    if let Some(attrs) = msg.attributes() {
        println!("attributes: {:?}", attrs);
    }
}
```

### Schema

A `[schema]` preamble can be added to `Manifest.toml` to define custom attributes and tags on messages:

```toml
[schema]
tags = ["security", "billing", "audit"]

[schema.attributes]
sponsor = "string"

[1]
message = "user login"
comment = "User successfully logged in"
sponsor = "security-team"
tags = ["security", "audit"]

[2]
message = "login failed"
comment = "Authentication failed"
```

#### Tags

Tags are a closed set of allowed values defined in `[schema]`. Each message can have zero or more tags. A `Tag` enum is generated with PascalCase variants:

```rust,ignore
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Tag { Audit, Billing, Security }
```

Tags on the `Message` struct are available as `&'static [Tag]` for typed local use. Through the `Message` trait, they are exposed as `Option<&'static [&'static str]>` for cross-crate compatibility — returning `None` when the schema doesn't define tags, or `Some` with the (possibly empty) tag list.

#### Custom attributes

Custom attributes are declared in `[schema.attributes]` and appear as `Option<&'static str>` fields on the generated `Message` struct. Unknown attribute keys or tag values cause a build error.

Through the `Message` trait, attributes are exposed as `Option<HashMap<&'static str, &'static str>>` — returning `None` when the schema doesn't define attributes, or `Some` with a map of the attributes set on the message. The built-in field names `message`, `comment`, `deprecated`, and `tags` are reserved and cannot be used as attribute names.
