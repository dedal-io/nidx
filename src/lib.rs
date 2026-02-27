//! Validate and extract information from national ID numbers.
//!
//! **nidx** is a zero-dependency library that validates and decodes national
//! identification numbers across multiple countries. Each country has its own
//! module with a `decode` function that returns country-specific information.
//!
//! # Supported countries
//!
//! | Country | Module |
//! |---------|--------|
//! | Albania | [`albania`] |
//! | Kosovo  | [`kosovo`]  |
//!
//! # Examples
//!
//! ```
//! // Decode an Albanian NID
//! let info = nidx::albania::decode("J00101999W").unwrap();
//! assert_eq!(info.birthday.to_string(), "1990-01-01");
//! assert_eq!(info.sex, nidx::Sex::Male);
//! assert!(info.is_national);
//! ```

mod date;
mod types;

#[doc(hidden)]
pub mod country;

pub use country::albania;
pub use country::kosovo;
pub use types::{Date, Sex};
