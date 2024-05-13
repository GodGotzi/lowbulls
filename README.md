# LowBull

[![Latest Version](https://img.shields.io/crates/v/lowbull.svg)](https://crates.io/crates/lowbull)
[![Documentation](https://docs.rs/lowbull/badge.svg)](https://docs.rs/lowbull)
[![License](https://img.shields.io/crates/l/lowbull.svg)](https://github.com/yourusername/lowbull#license)

`lowbull` is a Rust crate providing a framework for message handling and event monitoring.

## Overview

This crate defines two main modules: `core` and `watch`. The `core` module contains foundational traits and types for message handling, while the `watch` module provides utilities for monitoring events.

## Usage

To use `lowbull`, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
lowbull = "0.1.0"
```


markdown
Copy code
# LowBull

[![Latest Version](https://img.shields.io/crates/v/lowbull.svg)](https://crates.io/crates/lowbull)
[![Documentation](https://docs.rs/lowbull/badge.svg)](https://docs.rs/lowbull)
[![License](https://img.shields.io/crates/l/lowbull.svg)](https://github.com/yourusername/lowbull#license)

`lowbull` is a Rust crate providing a framework for message handling and event monitoring.

## Overview

This crate defines two main modules: `core` and `watch`. The `core` module contains foundational traits and types for message handling, while the `watch` module provides utilities for monitoring events.

## Usage

To use `lowbull`, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
lowbull = "0.1.0"
```

Then, you can use the crate in your Rust code by importing the necessary modules:

```rust
use lowbull::core::LowBullMaster;
use lowbull::watch::LowBullWatcher;
use anyhow::Result;

// Your code here...
```

# Examples

Here's a simple example demonstrating the usage of lowbull:


```rust
use lowbull::core::LowBullMaster;
use lowbull::watch::LowBullWatcher;
use anyhow::Result;

// Define message types
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Message {
    StartRender,
    StopRender,
    GetRender,
}

// Define response types
#[derive(Debug, Hash, PartialEq, Eq)]
enum Response {
    Render(bool),
    None,
}

// Implement a message handler
struct Master {
    render: bool,
    #[cfg(debug_assertions)]
    watcher: LowBullWatcher<Message>,
}

impl LowBullMaster<Message, Response> for Master {
    fn handle(&mut self, key: Message) -> Result<Response> {
        if cfg!(debug_assertions) {
            self.watcher.watch(key);
        }

        match key {
            Message::StartRender => {
                self.render = true;
                Ok(Response::None)
            }
            Message::StopRender => {
                self.render = false;
                Ok(Response::None)
            }
            Message::GetRender => Ok(Response::Render(self.render)),
        }
    }
}

// Your code here...

```

For more examples and detailed usage, please refer to the documentation.

# License

'lowbull' is licensed under the Apache 2.0 license. See LICENSE for details.