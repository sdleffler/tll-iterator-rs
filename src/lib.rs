#[macro_use]
extern crate type_operators;
extern crate type_level_logic as tll;

pub mod chain;
pub mod iterator;
pub mod map;


pub use iterator::Iterator as SizedIterator;
pub use iterator::FromIterator as FromSizedIterator;
pub use iterator::IntoIterator as IntoSizedIterator;
pub use iterator::NonEmpty;

pub use chain::Chain;
pub use map::Map;
