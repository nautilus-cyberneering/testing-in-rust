//! Example 04:
//!
//! An application that uses a user repository. There are two implementations of
//! the repository:
//!
//! 1. Using ``BTreeMap`` for production code.
//! 2. Using Vec for testing code.
//!
//! The application uses generics (static dispatch).
//!
//! ```text
//! pub struct App<T: UserRepository> {
//!     user_repository: T,
//! }
//! ```
//!
//! The repository type used by `App` is defined at compile time.
pub mod app;
pub mod user;
pub mod user_repository;
