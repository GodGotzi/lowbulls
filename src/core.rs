//! # LowBullMaster
//!
//! `LowBullMaster` is a trait for handling operations on keys of type `K` to produce results of type `R`.
//!
//! This trait defines two methods:
//! - `handle`: handles a single key and returns a result.
//! - `handle_many`: handles multiple keys and returns a vector of results.
//!
//! # Example
//!
//! ```
//! use anyhow::Result;
//!
//! struct ExampleBullMaster;
//!
//! impl LowBullMaster<i32, i32> for ExampleBullMaster {
//!     fn handle(&mut self, key: i32) -> Result<i32> {
//!         // Some operation on the key
//!         Ok(key * 2)
//!     }
//! }
//!
//! fn main() -> Result<()> {
//!     let mut bull_master = ExampleBullMaster;
//!     let keys = vec![1, 2, 3];
//!     let results = bull_master.handle_many(keys)?;
//!     println!("{:?}", results); // Output: [2, 4, 6]
//!     Ok(())
//! }
//! ```

use anyhow::Result;

/// A trait for handling operations on keys of type `K` to produce results of type `R`.
pub trait LowBullMaster<K: Eq + PartialEq, R: Eq + PartialEq> {
    /// Handles a single key and returns a result.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to be handled.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the result of the operation.
    fn handle(&mut self, key: K) -> Result<R>;

    /// Handles multiple keys and returns a vector of results.
    ///
    /// # Arguments
    ///
    /// * `keys` - A vector of keys to be handled.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of results.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the key operations fail.
    fn handle_many(&mut self, keys: Vec<K>) -> Result<Vec<R>> {
        let mut responses = Vec::new();
        for key in keys {
            responses.push(self.handle(key)?);
        }
        Ok(responses)
    }
}
