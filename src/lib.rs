//! `lowbull` is a Rust crate providing a framework for message handling and event monitoring.
//!
//! This crate defines two main modules: `core` and `watch`. The `core` module contains the foundational traits and types for message handling, while the `watch` module provides utilities for monitoring events.
//!
//! ## Usage
//!
//! To use `lowbull`, add it as a dependency in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! lowbull = "0.1.0"
//! ```
//!
//! Then, you can use the crate in your Rust code by importing the necessary modules:
//!
//! ```rust
//! use lowbulls::core::LowBullMaster;
//! use lowbulls::watch::LowBullWatcher;
//! use anyhow::Result;
//!
//! // Your code here...
//! ```
//!
//! ## Examples
//!
//! Here's a simple example demonstrating the usage of `lowbull`:
//!
//! ```rust
//! use lowbulls::core::LowBullMaster;
//! use lowbulls::watch::LowBullWatcher;
//! use anyhow::Result;
//!
//! // Define message types
//! #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
//! enum Message {
//!     StartRender,
//!     StopRender,
//! }
//!
//! // Define response types
//! #[derive(Debug, Hash, PartialEq, Eq)]
//! enum Response {
//!     None,
//! }
//!
//! #[derive(Debug, Hash, PartialEq, Eq)]
//! enum ResourceKey {
//!     GetTest,
//!     GetRender,
//! }
//!
//! #[derive(Debug, Hash, PartialEq, Eq)]
//! enum Resource {
//!     Test(bool),
//!    Render(bool),
//! }
//!
//! // Implement a message handler
//! struct Master {
//!     render: bool,
//!     #[cfg(debug_assertions)]
//!     watcher: LowBullWatcher<Message>,
//! }
//!
//! impl LowBullMaster<Message, Response, ResourceKey, Resource> for Master {
//!     fn handle(&mut self, key: Message) -> Result<Response> {
//!         if cfg!(debug_assertions) {
//!             self.watcher.watch(key);
//!         }
//!
//!         match key {
//!             Message::StartRender => {
//!                 self.render = true;
//!                 Ok(Response::None)
//!             }
//!             Message::StopRender => {
//!                 self.render = false;
//!                 Ok(Response::None)
//!             }
//!         }
//!     }
//!
//!     fn get_resource(&self, key: ResourceKey) -> Result<Resource> {
//!         match key {
//!            ResourceKey::GetTest => Ok(Resource::Test(true)),
//!            ResourceKey::GetRender => Ok(Resource::Render(self.render)),
//!         }
//!     }
//! }
//!
//! // Your code here...
//! ```
//!
//! For more examples and detailed usage, please refer to the documentation.

pub mod core;
pub mod watch;
pub use anyhow;

#[cfg(test)]
mod tests {

    use std::cell::RefCell;

    use crate::{
        core::LowBullMaster,
        watch::{self, LowBullWatcher},
    };
    use anyhow::Result;

    #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
    enum HandleKey {
        Empty,
        StartRender,
        StopRender,
    }

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

    #[test]
    fn test_master() {
        let mut master = Master {
            render: false,
            handle_watcher: LowBullWatcher::new(1000),
            resource_watcher: RefCell::new(LowBullWatcher::new(1000)),
        };

        assert_eq!(
            master.get_resource(ResourceKey::GetRender).unwrap(),
            Resource::Render(false)
        );

        assert_eq!(
            master.handle(HandleKey::StartRender).unwrap(),
            Response::None
        );

        assert_eq!(
            master.get_resource(ResourceKey::GetRender).unwrap(),
            Resource::Render(true)
        );

        assert_eq!(
            master.handle(HandleKey::StopRender).unwrap(),
            Response::None
        );

        assert_eq!(
            master.get_resource(ResourceKey::GetRender).unwrap(),
            Resource::Render(false)
        );

        assert_eq!(
            master.get_resource(ResourceKey::GetTest).unwrap(),
            Resource::Test(true)
        );

        let responses = master
            .handle_many(vec![HandleKey::StartRender, HandleKey::StopRender])
            .unwrap();

        assert_eq!(responses[0], Response::None);
        assert_eq!(responses[1], Response::None);

        for _ in 0..1000 {
            master.handle(HandleKey::Empty).unwrap();
        }

        // if cfg!(debug_assertions) {
        //    master.watcher.debug_history();
        //}
    }
}
