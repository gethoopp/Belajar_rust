mod base_rust;
pub use base_rust::print_text;
pub use base_rust::number_age;

mod api;
pub use api::routes::run_server;

mod basic;
pub use basic::data_types::tuple::tuple_demo;
pub use basic::data_types::char::char_demo;