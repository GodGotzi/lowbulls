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
//! use lowbull::core::LowBullMaster;
//! use lowbull::watch::LowBullWatcher;
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
//! use lowbull::core::LowBullMaster;
//! use lowbull::watch::LowBullWatcher;
//! use anyhow::Result;
//!
//! // Define message types
//! #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
//! enum Message {
//!     StartRender,
//!     StopRender,
//!     GetRender,
//! }
//!
//! // Define response types
//! #[derive(Debug, Hash, PartialEq, Eq)]
//! enum Response {
//!     Render(bool),
//!     None,
//! }
//!
//! // Implement a message handler
//! struct Master {
//!     render: bool,
//!     #[cfg(debug_assertions)]
//!     watcher: LowBullWatcher<Message>,
//! }
//!
//! impl LowBullMaster<Message, Response> for Master {
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
//!             Message::GetRender => Ok(Response::Render(self.render)),
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

#[cfg(test)]
mod tests {

    use crate::{
        core::LowBullMaster,
        watch::{self, LowBullWatcher},
    };
    use anyhow::Result;

    #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
    enum Message {
        Empty,
        StartRender,
        StopRender,
        GetRender,
    }

    #[derive(Debug, Hash, PartialEq, Eq)]
    enum Response {
        Render(bool),
        None,
    }

    struct Master {
        render: bool,
        #[cfg(debug_assertions)]
        watcher: watch::LowBullWatcher<Message>,
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
                Message::Empty => Ok(Response::None),
                Message::GetRender => Ok(Response::Render(self.render)),
            }
        }
    }

    #[test]
    fn test_master() {
        let mut master = Master {
            render: false,
            watcher: LowBullWatcher::new(1000),
        };

        assert_eq!(
            master.handle(Message::GetRender).unwrap(),
            Response::Render(false)
        );

        assert_eq!(master.handle(Message::StartRender).unwrap(), Response::None);

        assert_eq!(
            master.handle(Message::GetRender).unwrap(),
            Response::Render(true)
        );

        assert_eq!(master.handle(Message::StopRender).unwrap(), Response::None);

        assert_eq!(
            master.handle(Message::GetRender).unwrap(),
            Response::Render(false)
        );

        let responses = master
            .handle_many(vec![
                Message::StartRender,
                Message::GetRender,
                Message::StopRender,
                Message::GetRender,
            ])
            .unwrap();

        assert_eq!(responses[0], Response::None);
        assert_eq!(responses[1], Response::Render(true));
        assert_eq!(responses[2], Response::None);
        assert_eq!(responses[3], Response::Render(false));

        for _ in 0..1000 {
            master.handle(Message::Empty).unwrap();
        }

        // if cfg!(debug_assertions) {
        //    master.watcher.debug_history();
        //}
    }
}
