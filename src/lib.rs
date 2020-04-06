#[macro_use]
extern crate derive_more;

pub use without_comments::{IntoWithoutComments, WithoutComments};

// TODO redocument, update readmes, mention that comments started in strings are still comments ("/*" starts a block comment)

pub mod languages;
mod without_comments;

#[cfg(test)]
mod tests;