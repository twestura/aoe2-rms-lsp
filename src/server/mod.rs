//! Language server functionality for Aoe2 RMS files.

mod completion;
mod hover;

pub use completion::get_completions;
pub use hover::{get_hover, lookup_hover};
