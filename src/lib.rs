pub mod handlers;
pub mod utils {
    pub mod day_4;
    pub mod day_6;
    pub mod day_7;
}
pub mod models;
mod parser;
mod routes;

pub use parser::ParsedPath;
pub use routes::app;
