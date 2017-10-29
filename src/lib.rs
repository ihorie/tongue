#[macro_use]
extern crate log;

mod tongue;

pub use tongue::config;
pub use tongue::lexer;
pub use tongue::parser;
pub use tongue::exec;
