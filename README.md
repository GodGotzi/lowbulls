# LowBulls

[<img alt="github" src="https://img.shields.io/badge/github-godgotzi/lowbulls-8da0cb?logo=github" height="20">](https://github.com/godgotzi/lowbulls)
[![Latest Version](https://img.shields.io/crates/v/lowbulls.svg)](https://crates.io/crates/lowbulls)
[![Documentation](https://docs.rs/lowbulls/badge.svg)](https://docs.rs/lowbulls)
[![License](https://img.shields.io/crates/l/lowbulls.svg)](https://github.com/godgotzi/lowbulls#license)


'low cost buisness and ui logic seperator'

`lowbull` is a Rust crate providing a framework for message handling and event monitoring.

## Overview

This crate defines two main modules: `core` and `watch`. The `core` module contains foundational traits and types for message handling, while the `watch` module provides utilities for monitoring events.

## Usage

To use `lowbull`, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
lowbull = "0.1.11"
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
use lowbull::anyhow::Result;

// Define message types
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Message {
    StartRender,
    StopRender,
}

// Define response types
#[derive(Debug, Hash, PartialEq, Eq)]
enum Response {
    None,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum ResourceKey {
    GetTest,
    GetRender,
}
    
#[derive(Debug, Hash, PartialEq, Eq)]
enum Resource {
    Render(bool),
    Test(bool),
}

// Implement a message handler
struct Master {
    render: bool,
    #[cfg(debug_assertions)]
    handle_watcher: watch::LowBullWatcher<HandleKey>,
    #[cfg(debug_assertions)]
    resource_watcher: RefCell<watch::LowBullWatcher<ResourceKey>>,
}

impl LowBullMaster<HandleKey, Response, ResourceKey, Resource> for Master {
    fn handle(&mut self, key: HandleKey) -> Result<Response> {
        if cfg!(debug_assertions) {
            self.handle_watcher.watch(key);
        }

        match key {
            HandleKey::StartRender => {
                self.render = true;
                Ok(Response::None)
            }
            HandleKey::StopRender => {
                self.render = false;
                Ok(Response::None)
            }
            HandleKey::Empty => Ok(Response::None),
        }
    }

    fn get_resource(&self, key: ResourceKey) -> Result<Resource> {
        if cfg!(debug_assertions) {
            self.resource_watcher.borrow_mut().watch(key);
        }

        match key {
            ResourceKey::GetTest => Ok(Resource::Test(true)),
            ResourceKey::GetRender => Ok(Resource::Render(self.render)),
        }
    }
}

// Your code here...

```

For more examples and detailed usage, please refer to the documentation.

# License

'lowbull' is licensed under the Apache 2.0 license. See LICENSE for details.