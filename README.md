# context-attribute
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Set the error [`context`] using doc comments.

This is useful because instead of writing manual error messages to provide context to an error, it
automatically derives it from doc comments. This works especially well for async contexts, where
stack traces may not be persisted past yield points and thread boundaries. But contexts do.

[`context`]: https://docs.rs/failure/0.1.5/failure/trait.ResultExt.html#tymethod.context

- [Documentation][8]
- [Crates.io][2]
- [Releases][releases]

## Examples
```rust
use context_attribute::context;
use failure::{ensure, ResultExt};

/// Square a number if it's less than 10.
#[context]
fn square(num: usize) -> Result<usize, failure::Error> {
    ensure!(num < 10, "Number was too large");
    Ok(num * num)
}

fn main() -> Result<(), failure::Error> {
    let args = std::env::args();
    ensure!(args.len() == 2, "usage: square <num>");
    let input = args.skip(1).next().unwrap().parse()?;

    println!("result is {}", square(input)?);

    Ok(())
}
```

```txt
$ cargo run --example square 12
Error: ErrorMessage { msg: "Number was too large" }
Square a number if it's less than 10.
```

## Installation
```sh
$ cargo add context-attribute
```

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

## References
None.

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/context-attribute.svg?style=flat-square
[2]: https://crates.io/crates/context-attribute
[3]: https://img.shields.io/travis/yoshuawuyts/context-attribute/master.svg?style=flat-square
[4]: https://travis-ci.org/yoshuawuyts/context-attribute
[5]: https://img.shields.io/crates/d/context-attribute.svg?style=flat-square
[6]: https://crates.io/crates/context-attribute
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/context-attribute

[releases]: https://github.com/yoshuawuyts/context-attribute/releases
[contributing]: https://github.com/yoshuawuyts/context-attribute/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/yoshuawuyts/context-attribute/labels/good%20first%20issue
[help-wanted]: https://github.com/yoshuawuyts/context-attribute/labels/help%20wanted
