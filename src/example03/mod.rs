//! Example 03:
//!
//! An application that uses a user repository. There are two implementations of
//! the repository:
//!
//! 1. Using ``BTreeMap`` for production code.
//! 2. Using Vec for testing code.
//!
//! The application uses dynamic dispatch.
//!
//! ```text
//! pub struct App {
//!     user_repository: Box<dyn UserRepository>,
//! }
//! ```
//! 
//! The repository type used by `App` is defined at runtime.
pub mod app;
pub mod user;
pub mod user_repository;
