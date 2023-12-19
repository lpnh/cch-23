pub mod handlers;
pub mod utils;
pub mod models;
mod parser;
mod routes;

pub use parser::ParsedPath;
pub use routes::app;
