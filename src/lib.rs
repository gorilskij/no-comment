#![doc(html_logo_url = "https://i.ibb.co/KzKy5DL/logo.png")]

#[macro_use]
extern crate derive_more;

pub use without_comments::{IntoWithoutComments, WithoutComments};

pub mod languages;
mod without_comments;

#[cfg(test)]
mod tests;
