//! Language server functionality for Aoe2 RMS files.

mod completion;
mod hover;

pub use hover::get_hover;
pub use hover::lookup_hover;
